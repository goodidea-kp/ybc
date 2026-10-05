# Yew Framework Best Practices

This project is a **Yew 0.23** WASM single-page app (struct components, `yew-router` 0.20, `ybc`
Bulma widgets, `gloo-*`, `reqwasm`). These guidelines extend the Rust style rules with framework
conventions specific to Yew. They are grounded in this codebase's existing patterns: struct
components with a `Msg` enum, `Properties` carrying `Callback`s, and async work driven through
`ctx.link().send_future(...)`.

> **Convention first:** this codebase uses **struct components** uniformly. Match that for changes to
> existing components. Reach for **function components + hooks** only for genuinely new, self-contained
> widgets (see the last section) — do not rewrite working struct components into hooks.

## Contents
- [Keep `view` pure — no side effects, no panics](#keep-view-pure)
- [Drive side effects from messages, never from `changed()`](#side-effects-from-messages)
- [Always give keyed list items a stable `key`](#keyed-lists)
- [Don't panic in WASM — route failures through `Msg::Error`](#dont-panic)
- [Clone deliberately, not reflexively](#clone-deliberately)
- [Properties: minimal, `PartialEq`, `Rc` for big payloads](#properties)
- [Load initial data in `create`/`rendered`, not `view`](#initial-load)
- [Callbacks: build in `view`/`create`, keep handlers thin](#callbacks)
- [Prefer shared context over scattered storage reads](#shared-context)
- [Routing with yew-router](#routing)
- [When to use function components + hooks](#function-components)

## Keep `view` pure {#keep-view-pure}

`view` runs on every re-render. It must be a pure function of `self` + `ctx.props()`: build `Html`,
nothing else. No network calls, no storage writes, no panics, no expensive computation.

**DON'T** — panics inside `view` (a single bad row blanks the whole page):
```rust
impl Admins {
    fn is_row_selected(&self, id: u64) -> Classes {
        let admin = self.list.iter().filter(|a| a.id == id).next().unwrap(); // panics if absent
        if admin.selected { classes!("is-selected") } else { classes!("") }
    }
}
```

**DO** — total functions, no unwrap, use iterator adapters with names:
```rust
fn row_class(&self, admin_id: u64) -> Classes {
    let is_selected = self.list.iter().any(|admin| admin.id == admin_id && admin.selected);
    if is_selected { classes!("is-selected") } else { Classes::new() }
}
```

Extract complex markup into helper methods returning `Html` (e.g. `fn render_row(&self, ...) -> Html`)
to keep `view` scannable.

## Drive side effects from messages, never from `changed()` {#side-effects-from-messages}

`changed()` exists to reconcile new props into local state and report whether a re-render is needed.
Performing I/O, showing dialogs, or firing network requests there is an anti-pattern: `changed()` is
called on **every** parent re-render and prop change, so the effect fires unpredictably and repeatedly.

**DON'T** — a DELETE request + a blocking prompt run inside `changed()`:
```rust
fn changed(&mut self, ctx: &Context<Self>, _old: &Self::Properties) -> bool {
    if ctx.props().on_delete.is_some() {
        if let Some(reason) = gloo_dialogs::prompt("Reason...", None) {
            ctx.link().send_future(async move { /* DELETE ... */ });   // fires on any prop change
        }
    }
    true
}
```

**DO** — reconcile props only; trigger the effect from an explicit message:
```rust
fn changed(&mut self, ctx: &Context<Self>, _old: &Self::Properties) -> bool {
    let endpoint_changed = ctx.props().end_point != self.endpoint;
    if endpoint_changed {
        self.endpoint = ctx.props().end_point.clone();
        ctx.link().send_message(Msg::Loading);
    }
    endpoint_changed
}

// the delete is its own message, raised by the button's onclick:
Msg::DeleteRequested => {
    let Some(reason) = gloo_dialogs::prompt("Reason...", None).filter(|r| !r.trim().is_empty())
        else { return false; };
    ctx.link().send_future(async move { /* DELETE ... */ });
    false
}
```

Rule of thumb: **a user action → a `Msg` → an effect.** Props changing should at most schedule a
`Msg`, never perform the work directly.

## Always give keyed list items a stable `key` {#keyed-lists}

When rendering a `Vec` into rows/cards with `.map()`, attach a `key` that is stable across renders
(a uuid or db id — never the array index). Without keys, Yew falls back to positional diffing:
edits/deletes can reattach state to the wrong element and re-render more than necessary.

**DON'T:**
```rust
let rows: Html = self.list.iter().map(|admin| html! {
    <tr class={self.row_class(admin.id)}> /* ... */ </tr>
}).collect();
```

**DO:**
```rust
let rows: Html = self.list.iter().map(|admin| html! {
    <tr key={admin.uuid.clone()} class={self.row_class(admin.id)}> /* ... */ </tr>
}).collect();
```

## Don't panic in WASM — route failures through `Msg::Error` {#dont-panic}

A panic in WASM aborts the module and leaves a blank page with only a console trace — there is no
unwinding and no recovery. The "avoid `unwrap()`/`expect()`" rule from `rust-idioms.md` is therefore
**hard** in components. Two recurring offenders here:

- `gloo_storage::SessionStorage::get("jwt").unwrap()` at the top of nearly every async block — panics
  if the user is logged out / the key is missing.
- Unwrapping the response `Authorization` header (including inside the `bastion_resp!` macro) — panics
  on any 2xx response that doesn't carry the header.

**DO** — fall back to a recoverable `Msg::Error` (and a logoff where auth is missing):
```rust
let jwt = match SessionStorage::get::<String>("jwt") {
    Ok(token) => token,
    Err(_) => return Msg::Logoff,
};
// for the macro: treat a missing Authorization header as Err, not a panic.
```

Selecting `.unwrap()`-free patterns here also matters because WASM panics are invisible to most
end users — they just see a broken screen.

## Clone deliberately, not reflexively {#clone-deliberately}

`.clone()` is sometimes unavoidable in Yew (moving owned data into `async move` blocks and `Callback`
closures). But high clone density (some files here clone 30–50×) usually signals reflexive cloning in
`view` and handlers that re-clones large structs every render.

- Clone **only** the fields you actually move into an `async move`/closure, not the whole struct.
- For data shared across components or stored in props, wrap it in `Rc<T>` (or `Rc<[T]>`) so a clone is
  a refcount bump, not a deep copy. `Rc` also makes `Properties` `PartialEq` cheap.
- Borrow in `view` where the value is only read; clone at the point of capture.

**DON'T:**
```rust
let whole_admin = self.admin_for_config.as_ref().unwrap().clone(); // clones everything
let uuid = whole_admin.uuid;
```
**DO:**
```rust
let admin_uuid = self.admin_for_config.as_ref().map(|admin| admin.uuid.clone());
let Some(admin_uuid) = admin_uuid else { return false; };
```

## Properties: minimal, `PartialEq`, `Rc` for big payloads {#properties}

- Every `Properties` struct must derive `PartialEq` (it does here) — Yew uses it to skip re-rendering
  children whose props are unchanged. Don't break it by storing non-comparable or always-changing
  fields.
- Keep props **small**: pass identifiers + `Callback`s, not large owned collections. If a child needs
  a big list, pass `Rc<Vec<T>>` so equality is a pointer compare and clones are cheap.
- Name callback props for the event they signal (`on_delete`, `on_selection`, `on_update`) and emit
  domain data, not UI noise.

## Load initial data in `create`/`rendered`, not `view` {#initial-load}

Kick off the first fetch from `create` (via `ctx.link().send_message(Msg::Loading)`) or from
`rendered(first_render)` guarded by `if first_render`. Never start loads from `view` (it re-runs on
every render → request storms). This codebase already does this correctly — keep it.

```rust
fn rendered(&mut self, ctx: &Context<Self>, first_render: bool) {
    if first_render {
        ctx.link().send_message(Msg::Loading);
    }
}
```

## Callbacks: build in `view`/`create`, keep handlers thin {#callbacks}

- Create callbacks with `ctx.link().callback(...)` / `callback_future(...)`. Capturing per-row values
  (e.g. an id) into the closure is fine.
- Keep `update` arms short and single-purpose (same ≤25-line / single-responsibility rule). Extract a
  fetch into a private `async fn` or helper that returns the next `Msg`, rather than inlining a long
  request builder in every arm.
- Prefer `send_message`/`send_future` over `wasm_bindgen_futures::spawn_local` inside components, so
  results flow back as messages through the normal update loop.

## Prefer shared context over scattered storage reads {#shared-context}

Reading `jwt`/`puuid` from `SessionStorage` at the top of every async block couples each component to
global storage and multiplies the panic surface. For shared session state, prefer a Yew
`ContextProvider<SessionState>` consumed via `ctx.link().context(...)` (struct components) or
`use_context` (function components), or a state crate (`yewdux`). Centralizes the read, makes the
dependency explicit in props/context, and gives one place to handle "not logged in".

## Logging: use the `log` facade, not `gloo_console` directly {#logging}

The app initializes `wasm_logger` in `src/bin/app.rs`, which bridges the `log` crate to the browser
console with level filtering. **Log through the `log` facade** (`log::error!`, `log::warn!`,
`log::info!`, `log::debug!`) — not `gloo_console::log!`/`error!` directly. The facade gives one place
to set verbosity (raise to `warn` for production via `wasm_logger::Config::new(log::Level::Warn)`),
consistent formatting, and the option to swap backends later.

- **Levels:** `error!` for failures the user is told about; `warn!` for handled-but-notable
  (HTTP non-2xx, missing selection); `info!` for coarse lifecycle (logged in/out); `debug!` for
  developer tracing. Don't log at `info`/`error` for routine flow.
- **No commented-out logs.** Delete `// console::log!(...)` lines — they are dead code. If a trace is
  worth keeping, make it a real `log::debug!` (filtered out in production by level).
- **Capture syntax:** `log::warn!("request failed with HTTP {status}")`, not `"{}", status`.
- **Don't log secrets** — never log the JWT, passwords, or secret `value`/`userAttributes` payloads.

```rust
// DON'T: direct console + dead commented trace + leaks the token
// console::log!("jwt:{}", &jwt);
gloo_console::error!("Parse error in json:{}", err.to_string());

// DO: facade, level-appropriate, no secret
log::error!("could not parse response JSON: {error}");
```

## Testing: extract pure logic, run it on the host target {#testing}

Component lifecycle and `view` can only run under `wasm-bindgen-test` (a browser/node
runtime). So the highest-value, cheapest tests come from **extracting pure logic into free
functions / associated functions** and testing them with plain `#[test]` (host `cargo test`):
selection counts, row-class predicates, grouping/sorting, formatting/truncation, filter builders.
This is a direct payoff of the "keep `view` pure" rule — pure helpers are trivially testable.

Two gotchas when building fixtures for host tests:
- **Don't call `Model::default()` if its `Default` impl touches `js_sys`/`web_sys`.** Several models
  here (e.g. `SymmetricKey`) build a date via `js_sys::Date` in `Default`, which panics on the host
  target ("cannot call wasm-bindgen imported functions on non-wasm targets"). Construct the fixture
  field-by-field instead, or deserialize it from a JSON literal.
- **Deserialize from a representative server payload** when a struct has many fields — it doubles as a
  contract check that the model still parses what the API returns (see the `GenericPage` tests).

```rust
#[cfg(test)]
mod tests {
    use super::*;
    fn admins() -> Vec<Admin> {
        let page: AdminPage = serde_json::from_str(SAMPLE_PAGE_JSON).expect("sample parses");
        page.content
    }
    #[test]
    fn count_selected_counts_only_marked() { /* ... */ }
}
```

## Routing with yew-router {#routing}

- Define routes as a `#[derive(Routable)]` enum; render with `<Switch<Route> render={...}/>`.
- Navigate with the `Navigator` (`ctx.link().navigator()` / `use_navigator`), and link with
  `<Link<Route> to={...}>` — don't hand-build `href`s or poke `window.location` for in-app navigation.
- Keep the route→component mapping in one `switch` function.

## When to use function components + hooks {#function-components}

For a **new, self-contained** widget with simple local state, a function component is less ceremony:

```rust
#[function_component(KeyPicker)]
fn key_picker(props: &KeyPickerProps) -> Html {
    let selected = use_state(|| props.default_uuid.clone());
    let on_change = {
        let selected = selected.clone();
        Callback::from(move |uuid: String| selected.set(uuid))
    };
    // use_effect_with(deps, ...) for side effects; use_memo for derived data.
    html! { /* ... */ }
}
```

Guidance:
- Use `use_effect_with(deps, ...)` (not bare `use_effect`) so effects re-run only when `deps` change —
  the hook equivalent of "don't fire side effects on every render".
- `use_state`/`use_reducer` for local state, `use_memo` for derived values, `use_callback` for stable
  callbacks, `use_context` for shared session state.
- **Do not** convert the existing 35 struct components to hooks as part of unrelated work — that's a
  separate, deliberate migration. Match the struct-component convention when editing them.

## Yew review checklist

- [ ] `view` is pure: no I/O, no storage writes, no `unwrap()`/panics, no heavy computation
- [ ] No side effects (fetch/dialog/storage) inside `changed()` — only prop→state reconciliation (+ optional `send_message`)
- [ ] Every `.map()`-rendered list element has a stable `key` (uuid/id, not index)
- [ ] No `unwrap()`/`expect()` on `SessionStorage`, response headers, or JSON in components — route to `Msg::Error`/`Logoff`
- [ ] `.clone()` is limited to values genuinely moved into closures/`async move`; large shared data uses `Rc<T>`
- [ ] `Properties` derive `PartialEq`, stay small, and pass `Callback`s/ids (or `Rc<…>`) not big owned collections
- [ ] Initial loads happen in `create`/`rendered(first_render)`, never in `view`
- [ ] `update` arms are short and single-purpose; long fetches extracted to helpers
- [ ] In-app navigation uses `yew-router` (`Navigator`/`Link`), not `window.location`
- [ ] Logging goes through the `log` facade at an appropriate level (not raw `gloo_console`); no commented-out logs; no secrets logged
- [ ] New self-contained widgets may use function components + hooks (`use_effect_with` for effects); existing struct components are left as struct components
