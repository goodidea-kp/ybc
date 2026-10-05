---
name: rust-code-style
description: Project conventions for this Yew/WASM frontend — Rust coding style grounded in programmer-cognition research (Felienne Hermans' "The Programmer's Brain") plus Yew 0.23 framework best practices. Use when writing, reviewing, or refactoring code in this project to keep names descriptive, functions short and single-purpose, error handling explicit, Rust idioms (iterators, `?`, exhaustive matching, newtypes) applied consistently, AND Yew components correct (pure `view`, side effects driven by messages not `changed()`, keyed lists, no panics in WASM, deliberate cloning).
---

# Rust + Yew Code Style

Conventions for writing and reviewing this project's code. It is a **Yew 0.23** WASM single-page app
(struct components, `yew-router`, `ybc`/Bulma, `gloo-*`, `reqwasm`), so the rules cover both general
Rust style **and** Yew framework usage. The guiding rule: **write code for the human brain, not the computer.** Code is read far more often than written, so every decision should reduce the reader's cognitive load.

## Core Principles

1. **Optimize for reading, not writing** — code is read ~10x more than written.
2. **Reduce cognitive load** — working memory holds ~7 items; don't waste slots on cryptic names.
3. **Make intent explicit** — names reveal purpose without forcing the reader to hold context in their head.
4. **Favor clear beacons** — readers scan for recognizable patterns; descriptive names beat single letters.

These cash out into four defaults: full names over abbreviations, explicit over implicit, simple over clever, readable over writeable.

## The non-negotiable rule

**Never use single-letter variable names**, except in three narrow cases: tiny (1–2 line) closures with obvious context, math formulas matching standard domain notation (`dx`, `dy`), and generic type parameters (`T`, `U`). When in doubt, use a full name.

## Reference guides

Load the relevant guide when working in that area — each contains DON'T/DO examples:

- **[references/naming.md](references/naming.md)** — variable, loop, iterator, boolean, function, and domain naming; abbreviations; pronounceability.
- **[references/functions.md](references/functions.md)** — function length (≤25 lines), single responsibility, parameter limits (≤3), nesting depth (≤2), guard clauses.
- **[references/rust-idioms.md](references/rust-idioms.md)** — explicit signatures, type aliases, newtypes, enums for state, avoiding `unwrap()`/`expect()`, error context, exhaustive matching, `#[must_use]`, iterators, `?`, `let-else`, `LazyLock`/`OnceLock`, `NonZero<T>`, `#[non_exhaustive]`.
- **[references/organization-and-quality.md](references/organization-and-quality.md)** — module structure, file size limits, comments (why not what), public-API docs, error design, testing conventions, anti-patterns (god objects, magic numbers, clever code).
- **[references/yew-best-practices.md](references/yew-best-practices.md)** — Yew 0.23 component conventions: pure `view`, side effects from messages (not `changed()`), keyed lists, no panics in WASM (`SessionStorage`/header/JSON `unwrap`), deliberate cloning + `Rc`, `Properties` design, initial-load placement, callbacks, shared context, `yew-router`, and when to use function components + hooks. **Load this whenever touching anything under `src/components/`, `src/pages/`, or `lib.rs`.**

## Code review checklist

Tooling (run first — catches most mechanical issues automatically):
- [ ] `cargo fmt` applied
- [ ] `cargo clippy -- -D warnings` passes

Naming and structure:
- [ ] No single-letter variable names (except the limited exceptions above)
- [ ] All names are clear and descriptive
- [ ] Functions are short and focused (<25 lines)
- [ ] Nesting depth is minimal (≤2 levels) — use `let-else` for destructuring guards

Error handling:
- [ ] No `unwrap()` or `expect()` in production code
- [ ] Error handling is explicit with context
- [ ] String formatting uses capture syntax: `"{val}"` not `"{}"` + val

API design:
- [ ] Public APIs have documentation
- [ ] Public enums in library code are `#[non_exhaustive]` if they may grow
- [ ] Types make invalid states unrepresentable where possible

Yew components (see [references/yew-best-practices.md](references/yew-best-practices.md) for details):
- [ ] `view` is pure — no I/O, storage writes, `unwrap()`/panics, or heavy computation
- [ ] No side effects in `changed()` — only prop→state reconciliation (a user action → `Msg` → effect)
- [ ] Every `.map()`-rendered list element has a stable `key` (uuid/id, not the array index)
- [ ] No `unwrap()`/`expect()` on `SessionStorage`, response headers, or JSON — route to `Msg::Error`/`Logoff` (WASM panics blank the page)
- [ ] `.clone()` limited to values moved into closures/`async move`; large shared data uses `Rc<T>`
- [ ] `Properties` derive `PartialEq`, stay small, pass `Callback`s/ids (or `Rc<…>`) not big owned collections
- [ ] Initial data loads in `create`/`rendered(first_render)`, never in `view`
- [ ] In-app navigation uses `yew-router` (`Navigator`/`Link`), not `window.location`

General:
- [ ] File is ≤500 lines — split by responsibility if over; target is 200–300
- [ ] Tests are comprehensive with descriptive names
- [ ] No commented-out code
- [ ] No magic numbers — all constants are named
- [ ] Code follows existing patterns in the codebase (struct components stay struct components)
- [ ] Rust idioms are used (iterators, `?`, `let-else`, pattern matching)
