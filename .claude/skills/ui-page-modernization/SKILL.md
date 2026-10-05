---
name: ui-page-modernization
description: Repeatable per-page loop to harden and modernize the Bastion operator UI (Rust/Yew/WASM) one page at a time — enumerate happy/unhappy cases, mirror the spec's input constraints with shared Rust validation, cover with repeatable Playwright e2e (deterministic seed, idempotent login, X-Trace-Id correlation), then simplify. Use whenever fixing UI defects, adding or changing form validation (min/max/pattern/required), writing or updating Playwright specs, syncing the UI to spec/bastion-api.yaml, or redesigning a page/flow, or ADDING any panel/notice/badge to an existing page — even if the request only says "fix the login page", "clean up this form", or "add X to the deployment page". Pair it with `ui-cognitive-simplicity`, which owns the panel budget and the flow audit; run that one first when the change alters what a page renders. The backend half of this contract is bastion's `validation-error-contract` skill.
---

# UI Page Modernization Loop (Bastion Config, Yew/WASM)

North star: **modernize the UI one page at a time, and never let a page regress once it's green.** Each page is a vertical slice — its input rules, its errors, its tests, and its layout are brought into agreement in a fixed order, then locked behind repeatable Playwright coverage. The loop only moves forward: you do not simplify a page's UX until its validation and tests are green, because the tests are what make the simplification safe.

Companions in this repo: `ui-cognitive-simplicity` (the *what* of good UX — steps/memories/decisions), `rust-code-style` (Yew/Rust idiom). The server side of the same contract lives in the **bastion** repo's `validation-error-contract` and `rest-api-cognitive-simplicity` skills. This skill is the *process* that drives all of them page by page.

## The loop — run these seven steps per page, in order

Do not skip ahead. Each step has an exit gate; if the gate fails, fix it before the next step.

1. **Enumerate cases.** Write the happy path plus every unhappy path *first*, as a list, before touching code. For each input: empty, below-min, above-max, boundary (min and max exactly), wrong pattern, and server-reject (a value that passes the client but the backend refuses). For login this is: valid creds → dashboard/reset; empty field → dialog; 404 → "combination not found"; 412 → YubiKey path. *Gate: the list exists and names the expected user-visible outcome for each case.*

2. **Spec is truth.** Encode the field constraints and the error codes in [`spec/bastion-api.yaml`](../../../spec/bastion-api.yaml). If a rule (a min length, a regex, a required field) is not in the spec, it does not exist — add it there before enforcing it anywhere. The spec is the shared contract with the backend; a rule that lives only in `#[validate(...)]` will silently drift. *Gate: every constraint you're about to enforce has a home in the spec.*

3. **Mirror on the UI — via the shared validation module, never copy-paste.** Enforce the spec's rules with the `validator` crate, but source shared patterns from **one** place. Today the mm/dd/yyyy `DATE_REGEXP` is redefined in `model/symmetric_key.rs`, `model/cert.rs`, and `lib.rs`, and `pg_migration.rs` hand-rolls its own `validate()` — that is the drift this step eliminates. Create/extend a single `src/validation` module that owns the shared regexes and validators; every component imports from it. *Gate: no new bespoke regex or one-off `fn validate`; the component uses the shared module and its `#[validate]` messages match the spec.*

4. **Hand off to the backend.** Client validation is for *fast feedback only* — the server is the source of truth (see `ui-cognitive-simplicity`: "UI renders state, it never derives it"). Open a matching change in the bastion repo (`validation-error-contract`) so the same constraint is enforced server-side and returns a stable error code. Never encode policy (allowed algorithms, RBAC, rotation windows) in the UI; mirror only shape-level rules (length/pattern/required). *Gate: the backend enforces the rule and the UI maps its error code to a message rather than inventing one.*

5. **Unify the error surface.** The UI must present one consistent error experience. Map the backend's stable error code → a user-facing message in one place; do not scatter `format!("Error:{}", status)` across components (login.rs still does this). When the backend consolidates its two error shapes (`ApiErrorResponse` vs `GenericErrorMessage`), consume the unified one. *Gate: each unhappy case shows a specific, human message — never a bare status code.*

6. **Cover with repeatable Playwright.** Tests must be deterministic and re-runnable against the same deployment without hand-resetting state:
   - **Selectors:** add `data-testid` to the page's inputs, buttons, and error containers *as part of this step* — this is also what makes step 7's redesign safe. Never select on Bulma classes or label text.
   - **Login fixture is state-aware** (the deployment tiers down nightly): try the post-reset password first; on a fresh boot, log in with the initial password, detect the `/reset-pwd` screen, and reset. Assert on *being on the reset screen*, not on which password worked.
   - **Correlation:** set `X-Trace-Id: <runId>` via `page.setExtraHTTPHeaders` — the backend's `AuditWebFilter` threads it into the audit/trace record, so a failing case can be joined to the exact backend line.
   - **One spec per page**, one `test()` per enumerated case from step 1, each asserting the specific outcome. *Gate: the whole spec passes twice in a row against the same running deployment with no manual reset.*

7. **Simplify — only now.** With the page green, apply `ui-cognitive-simplicity`: run its flow audit (count steps / memories / decisions), remove or default or derive every field the server already knows, model async screens as one enum, and align to modern form UX (inline field-level errors, disabled-until-valid submit, single primary action). The Playwright snapshot + assertions from step 6 are your regression net — a redesign that changes an asserted outcome or the visual baseline must be intentional. *Gate: at least one of steps/memories/decisions went down, none went up, and the spec from step 6 is still green.*

## Repeatability rules (non-negotiable)

- **Deterministic starting state.** Seed via the Bruno `api-walkthrough` golden path (local/ephemeral only). Against the real AWS deployment, keep the suite **read-only** except the one first-boot password reset — never seed or run destructive actions there.
- **No sleeps.** Wait on conditions (element visible, request settled), never `waitForTimeout`.
- **Idempotent.** A spec must pass on a fresh boot *and* on a re-run the same day. If it only passes once, it is not done.
- **Isolated by trace id.** Every run tags its requests so failures are diagnosable after the fact via CloudWatch + the audit trail.

## Anti-patterns this loop exists to kill

- A regex or validation rule copy-pasted into a second component instead of imported from `src/validation`.
- A client rule with no spec entry and no backend counterpart (it *will* drift).
- A test that selects on `.button.is-primary` or visible text, or that needs the DB reset by hand between runs.
- Redesigning a page's layout before it has passing e2e coverage — you've removed your own safety net.
- The UI deciding *policy* (what's allowed) rather than *presentation* (how to show what the server allowed).
