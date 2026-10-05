---
name: ui-cognitive-simplicity
description: Reduce operator cognitive load in the Bastion Config UI (Rust/Yew/WASM). Load this BEFORE writing or editing ANY operator-facing Rust/Yew code — component, page, form, flow, error message, wizard, and specifically before ADDING a panel, notice, badge, chip, banner or button to a page that already has some. Applies even when the request is framed as a feature ("add a teardown ladder"), a backend follow-on ("surface the new field"), a bug fix, or "just add X" — and even if it never says UX, UI, simplicity or cognitive load. Also use when deciding whether logic belongs in the UI or the Bastion backend, and when reviewing an existing flow — always propose the simpler flow, not merely implement the requested one. If you are about to write html! for anything an operator will read, this skill applies.
---

# UI Cognitive Simplicity (Bastion Config, Yew/WASM)

North star: **an operator should never hold more than a few things in working memory at once.** Human working memory fits roughly 4–6 items ("The Programmer's Brain", Hermans). Bastion's domain — key ceremonies, cert lifecycles, policies, migrations — is intrinsically complex; the UI's job is to eliminate all *extraneous* load so the operator's budget is spent on the decision that actually matters ("rotate now or later?"), not on remembering steps, states, or vocabulary.

This skill applies the cognitive-complexity reduction method from *Rust Full-Stack Development with AI Pair Programming* (vocabulary preload, schema-first, controlled expansion, explicit pitfall callouts) to UI work.

## The prime directive: UI renders state, it never derives it

The single biggest complexity leak is a UI that *computes* what the backend already knows.

- If a component decides "which buttons are valid for a key in state `PREPARED`", that state machine now lives in two codebases and drifts. The backend must send the current state **and the allowed next actions**; the UI maps them to buttons. Nothing else.
- If the UI must chain more than one API call to fulfil one operator intent ("rotate cert" = issue + install + verify), that is a backend defect. Do not build a client-side orchestration. Propose a single ceremony endpoint to the Bastion repo (see its `rest-api-cognitive-simplicity` skill) and keep the UI to: one click → one request → one streamed/polled status.
- Client-side validation may *mirror* server rules for fast feedback, but the server response is the source of truth. Never encode policy (allowed algorithms, rotation windows, RBAC) in the UI.

Why: every rule moved server-side is a rule the operator (and the UI maintainer) no longer has to remember, and a rule that can't silently drift.

## One journey per screen — the panel budget

A page answers **one** question: "what, if anything, must I do here?" Every
element competing to answer it costs the reader a decision before they reach the
answer.

- **Count the panels a page renders in its HEALTHY steady state. The target is
  zero.** A finished state ("Attach complete") is a fact, not an instruction —
  put it in the header line as a chip. Status that needs nobody (a current
  revocation list, a version that matches) is a chip too.
- **The advice slot always means somebody must act.** The moment it also carries
  congratulations or routine status, the reader learns to skim it, and the one
  time it matters they skim past that too.
- **Never render advice for a journey the operator has not started.** A teardown
  ladder shown to everyone advertises "Step 1 of 3" of something nobody is
  doing, beside a panel saying the opposite. Destructive flows are ENTERED —
  the destructive button opens the checklist, and a second press performs it.
- **Two panels disagreeing in mood is a defect**, not a layout issue: a green
  "you are done" beside a blue "step 1 of 3" beside a red alarm makes the
  reader's first task working out which one is about them.

Observed 2026-08-04: a healthy PG deployment rendered exactly those three at
once, one of them added by a change that never loaded this skill.

## Flow audit — run this before and after any flow change

Count three numbers for the operator's path:

1. **Steps** — clicks/screens from intent to done.
2. **Memories** — things they must carry between steps (an ID, a filename, "which state was it in?", a value copied from another screen).
3. **Decisions** — questions the UI asks them.

A change is a simplification only if it lowers at least one number without raising the others. When asked to modify an existing flow, report these numbers for current vs. proposed — even when not asked. Proposing the better flow is in scope by default in this project.

For every input field and decision, ask in order (stop at the first yes):
1. Can the server **discover** it? (it already knows the domain, the key, the last-used value) → remove the field.
2. Can it be **defaulted**? → default it, tuck the override under "Advanced".
3. Can it be **derived** from something already entered? → derive it, show it read-only.
4. Can it be **postponed** until actually needed? → move it out of the main path.
Only then may it stay as a question.

## Yew/Rust specifics — make the compiler carry the load

Rust's type system is a cognitive aid: complexity encoded in types is complexity the reader no longer simulates in their head.

- **Model async screens as one enum, not booleans.** `is_loading` + `error: Option<String>` + `data: Option<T>` creates 8 theoretical states of which 4 are impossible — the reader must prove that to themselves every time. Instead:

  ```rust
  enum Remote<T> { Idle, Loading, Loaded(T), Failed(ApiError) }
  ```

  One `match` renders it; the compiler guarantees every state has a face. Same pattern for ceremonies: `enum RotateFlow { PickTarget, Confirming(Target), Streaming(Progress), Done(Summary), Failed(ApiError) }`.
- **Routes stay a typed enum** (`#[derive(Routable)]`) — never build URLs from strings. Missing pages become compile errors, and the enum is the site map (a schema the reader can load in one glance).
- **One component = one concept.** If you cannot name a component without "And" (`KeyListAndRotationPanel`), split it. Props read-only, state owned, messages up — the book's "rooms and mailboxes" model. A component whose `html!` needs scrolling is two components.
- **Name states with domain vocabulary, verbatim from the backend enums** (`PREPARED`, `ACTIVE`, `REVOKED`). The same noun everywhere — UI label, Rust variant, API field — is vocabulary preload: the operator learns the term once and it works on every screen, in every error, in the docs.
- Keep `spawn_local` blocks tiny: fetch → set state. Business logic in async blocks inside components is unfindable later.

## UX rules for this product (recent best practices, applied)

- **Schema-first screens.** Every screen answers within one second: where am I (domain, entity), what state is it in, what can I do next. For lifecycle entities (keys, certs) show the state machine visually — current state highlighted, possible transitions as the actual buttons. The diagram *is* the controls.
- **Progressive disclosure (controlled expansion).** First render shows the happy path only. Advanced options (algo overrides, custom validity, raw config) live behind one "Advanced" expander, collapsed by default. Never present 12 fields when 2 are required.
- **Recognition over recall.** Pickers over free-text IDs, recent values, last-used defaults. If the operator has to paste a UUID from another screen, the flow failed the audit above.
- **Visibility of long operations.** Migrations and rotations stream progress (SSE). Render structured phases ("Connecting → Applying changelog → Done"), not a raw log; keep the raw log behind an expander for debugging. Announce completion via `aria-live` — the operator will tab away.
- **Errors: what happened, why, what to do next** — all three, in operator vocabulary, with the failing field highlighted in place. Show all invalid fields at once, not first-failure-only. A dead-end error ("400 Bad Request") is a bug; if the server response lacks remediation, file it against the API.
- **Undo beats confirmation** for anything reversible. Reserve confirmation for genuinely irreversible ceremonies (key `destroy`, cert `revoke`) and make it informative: state exactly what stops working ("3 services currently decrypt with this key"). Type-to-confirm only for the truly destructive.
- **Empty states teach.** A domain with no keys shows the one next action and one sentence of why — not a blank table.
- **No optimistic UI for crypto state.** Ceremonies render server-confirmed state only; a spinner is honest, a premature "ACTIVE" badge is a lie the operator acts on.

## Poka-yoke: the operator never fills a form the server will refuse

Every flow and ceremony where a human can make a mistake is mistake-proofed. Use the highest rung that fits:

Strongest first (Shingo); use the highest rung that fits, and keep the last one always:

1. **Eliminate** -- make the wrong state unrepresentable. Precedent: the attach ticket, one redeemable value instead of two the operator had to match.
2. **Prevent** -- the wrong action is not offered. The read that precedes the action carries `allowedActions` plus `disabledReasons`/`whyNot`, so the button is disabled WITH its reason before the operator invests effort.
3. **Warn at the source** -- an action that creates a condition a LATER flow will fail on says so in its own confirmation ("after this, nobody in domain X can approve JIT sessions").
4. **Detect early** -- shape rules checked as the operator types.
5. **Detect late** -- the refusal at submit. Always present (state can change between read and write), but never the FIRST time the operator learns something the server already knew.

In the UI that means:

- **Render `whyNot` / `disabledReasons` before the first input.** If the read behind a screen says an action is unavailable, its button is disabled and the reason (with its next step and who to ask) is visible where the button is -- before a dialog opens, not after Submit. If the server does not send it, the UI does NOT compute it: file the gap against the API (`rest-api-cognitive-simplicity`, "Poka-yoke").
- **Confirmations of actions that break later flows say so**, in the server's words ("after this, nobody in domain X can approve JIT sessions").
- **A refusal is shown where the operator is looking**: `FormError` brings itself into view, marked fields scroll into view (`first_problem`), and a long dialog body shows that more is hidden (ybc scroll shadow, BASTION-108).
- Flow audit: count "steps until the operator learns it cannot work". Anything above 1 for a condition the server knew is a defect.
- Found 2026-10-05 (BASTION-109 -> 111): "no eligible approver" arrived after the DBA had filled duration, reason and ticket.

## Cognitive pitfalls — call these out explicitly in reviews

The book's practice: naming a confusion point removes its cost. When you review UI code, flag these patterns by name:

- **State machine duplicated in the UI** ("the backend already knows this — render `allowed_actions` instead").
- **Boolean explosion** ("model this as one enum, impossible states unrepresentable").
- **Vocabulary drift** (UI says "remove", API says "destroy", docs say "delete" — pick the API term everywhere).
- **Operator-as-orchestrator** (a runbook step that says "then click X, then go to Y" is a missing backend endpoint).
- **Hidden prerequisite / refusal after effort** (a flow that fails late because of something checkable up front — surface it before step 1, disabled-with-reason, not as an error after step 3; see "Poka-yoke").
- **Silent cause** (an action whose confirmation does not say which later flow it makes impossible).

## Flow proposal template

When proposing a better flow (do this even for working flows you touch):

```
Current:  N steps / M memories / K decisions — list them
Proposed: n steps / m memories / k decisions — list them
Moved to backend: <endpoints to add/change, filed against bastion repo>
Removed questions: <field → discover/default/derive/postpone>
```
