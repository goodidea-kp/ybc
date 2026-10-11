changelog
=========
This changelog follows the patterns described here: https://keepachangelog.com/en/1.0.0/.

Subheadings to categorize changes are `added, changed, deprecated, removed, fixed, security`.

## Unreleased
### added
- List controls (Bastion BASTION-105), generalised from the Bastion console's
  shared listing markup. They hold no list state: the page owns the query.
  - `SortHeader<C>` (`SortHeaderProps { column, label, direction, on_sort,
    testid, classes }`, `SortDirection { Ascending, Descending }`): a `<th>`
    with `aria-sort` (`none` when unsorted) around a real `<button>` that
    emits its column.
  - `Pager` (`PagerProps { current, last, on_page, aria_label, testid,
    classes }`, zero-based pages): a named `role="navigation"` of buttons,
    `aria-current="page"` on the current one, ellipses past seven pages
    (`page_links`), nothing for a single page. At an end Previous/Next stay
    focusable with `aria-disabled` and point to one "This is the first/last/
    only page." line.
  - `ListToolbar` (`ListToolbarProps { filter, on_filter, on_clear,
    filter_label, placeholder, size, sizes, on_size, range_text, testid }`,
    `PAGE_SIZES = [25, 50, 100]`): a named search box with Clear while it has
    text, the range sentence as a polite live region (a busy skeleton line
    while `range_text` is `None`) and a labelled rows-per-page select.
  - Test ids follow the console's: `<testid>-sort-*` is the caller's,
    `-pagination`, `-page-prev|next|<n>|edge`, `-filter-name|clear`,
    `-range`, `-page-size`.
- `StatusTag` (`tone: StatusTone`, `icon`, `title`, `light`, `size`, `id`,
  `testid`, `classes`, children): a status chip that never relies on colour
  alone. With text its icon is `aria-hidden`; without text it is
  `role="img"` named by `title` (or the tone's word) and shows the tone's
  icon when given none.
- `Tag` takes `icon` (an `aria-hidden` icon before the text) and `skeleton`.
- Bulma 1.x skeletons: a `skeleton` prop on `Title`, `Button` and `Tag`
  (`is-skeleton` + `aria-busy="true"`; a skeleton `Button` is also disabled)
  and `SkeletonBlock { lines, label, testid, classes }` (`skeleton-lines`, or
  one `skeleton-block` for `lines = 0`, with a visually hidden "Loading…").
- `Remote<T, E = String> { Idle, Loading, Loaded(T), Failed(E) }` with
  `is_loading`, `loaded`, `failure`, `map`, `as_ref` and `From<Result<T, E>>`;
  `RemoteView<T, E>` (`remote`, `render`, `skeleton`, `idle`,
  `render_failure`, `on_retry`, `testid`, `classes`) gives every state a face
  inside an `aria-busy` wrapper: a skeleton, the data, or a danger `Callout`
  with "Try again".
- `Disclosure` (`summary`, `open`, `on_toggle`, `id`, `testid`, `classes`,
  children) over native `<details>`/`<summary>`.
- `Theme::HighContrast` (Bastion BASTION-103): Bulma's light scheme plus
  `data-contrast="more"` (`CONTRAST_ATTRIBUTE`, `Theme::contrast_value`) for
  the page's own high-contrast CSS; stored as `high-contrast`. `ThemeToggle`
  offers it as a fourth button with visible text ("High contrast",
  `<testid>-high-contrast`). The no-flash head script in the module docs now
  handles it.
- `ReasonedButton` takes `style`; the disabled look is appended to it.
- `MIN_TARGET_STYLE`: every new list button, the pager's buttons and the
  disclosure summary are at least 24×24 CSS px (WCAG 2.2, 2.5.8).
- `ybc::theme`: light and dark colour schemes on top of Bulma 1.x.
  `Theme { System, Light, Dark }` (`#[non_exhaustive]`, room for a
  high-contrast variant); `apply` sets `data-theme` on `<html>` and removes it
  for `System`, so Bulma's `prefers-color-scheme` query decides again;
  `store` / `stored` keep the choice in `localStorage` under
  `THEME_STORAGE_KEY` (`ybc.theme`); the `use_theme()` hook applies, stores
  and re-renders. Outside a browser `stored()` is `System` and `apply` does
  nothing, so server-side rendering and host tests never touch the DOM. The
  module docs carry the inline `<head>` script that applies the stored choice
  before the WASM loads (no flash of the wrong scheme).
- `ThemeToggle`: a named `role="group"` of three buttons -- System, Light,
  Dark -- with visible text, an `aria-hidden` icon and `aria-pressed` on the
  current choice; never a colour-only switch. `testid` names the group and
  each button (`<testid>-system|light|dark`); `onchange` fires after a choice
  is applied and stored.
- `Table` takes `id`, `testid`, `aria_label`, `aria_labelledby` and
  `aria_describedby`. Each is rendered on the `<table>` element, also when
  `scrollable` wraps it in the one `div.table-container`, and omitted when
  `None`, so existing call sites render exactly as before. `classes` and the
  modifiers stay on the `<table>` too; the wrapper carries only
  `table-container`.
- `id: Option<AttrValue>` prop on every form control (`Select`, `MultiSelect`,
  `Input`, `TextArea`, `Checkbox`, `Radio`, `File`). It is rendered on the
  native control (not the Bulma wrapper) and omitted when `None`, so existing
  call sites are unaffected.
- `label_for: Option<AttrValue>` prop on `Field`, rendered as `<label for="...">`.
  Together with the control `id` this gives form fields an accessible name and
  makes clicking the label focus the control.
- `Field` now labels its control automatically. It mints an id, points
  `<label for>` at it, and shares it through the new `FieldContext` so `Input`,
  `Select`, `MultiSelect`, `TextArea` and `File` pick it up with no explicit
  `id`. Grouped and addons fields opt out to avoid duplicate ids.
- `Field` help text is linked to the control with `aria-describedby`, and
  `help_has_error` now also marks the control `aria-invalid` and `is-danger`.
- `invalid` and `required` props on every form control.
- `autocomplete` and `inputmode` (new `InputMode` enum) props on `Input`.
- `placeholder` prop on `Select`: a disabled, empty-valued first option that is
  selected while `value` is empty, so `required` rejects the untouched select.
- `RadioGroup` component: a `fieldset` + `legend` wrapper that gives a set of
  `Radio` buttons one accessible name and a single `disabled` switch.
- `PieChart` component (`PieSegment` data): an inline-SVG donut (or pie with
  `hole={0.0}`) whose centre shows the total and, on hover or focus, the active
  segment. Segments and legend entries are keyboard-reachable hit targets, render
  as real links when given an `href`, and emit `on_select` for router
  navigation. Colours come from `--ybc-chart-1..8` custom properties with
  colour-vision-safe fallbacks; segments past `max_segments` fold into "Other".

- `Calendar` supports native `disabled` controls and `mm/dd/yyyy` form values.

### changed
- A `Modal`/`ModalCard` `<dialog>` gives focus back to the control that
  opened it when it closes, also when the page removes it while open
  (WCAG 2.4.3; Bastion BASTION-104). Focus that already moved elsewhere on
  purpose is left alone.
- `Theme::ALL` has four entries.
- `Calendar` enhances a native date/time fallback with bulma-calendar 7.1.1
  when its JS and CSS assets are loaded. Each component owns its picker DOM and
  removes document listeners on unmount, without shared ID registries.
  ISO values remain the default; unsupported date formats fall back to ISO
  without panics. `None` clears the input, `disabled` locks both controls, and
  `display_mode` supports dropdown, dialog and inline pickers. Time values use
  24-hour `HH:mm`; custom time patterns are not applied.
- `Field` help text renders as `<p class="help">` (Bulma's own markup) instead
  of an orphan `<label>`, which assistive technology could not attach to any control.

### fixed
- `Calendar`: Cancel closes the picker. bulma-calendar 7.1.1's own Cancel does
  not close it in a real browser, though it does under jsdom, so a person who
  pressed Cancel stayed stuck with the picker open. The adapter now closes it
  and leaves the value as it was.
- `Calendar`: the hidden native fallback no longer shows beside the picker under
  Bulma, whose `.input { display: inline-flex }` beat the `hidden` attribute.
  The picker's input keeps its own class (the room for its icon) and takes only
  the fallback's modifiers (`is-small`, `is-danger`, ...), not `input`. Test hooks
  (`data-testid`, `data-cy`) move to the picker's input while it is mounted, so a
  test finds one control, and return to the fallback when it unmounts.

## 0.4.7
### fixed
- `PanelTabs` no longer emits `role="tablist"`. Bulma `.panel-tabs` is filter
  navigation, not a WAI-ARIA tablist widget, so the role tripped axe
  `aria-required-children` (its children are links, not `role="tab"`). It now
  renders as a labelled `role="navigation"` region. The accessible tablist widget
  remains available via `Tabs`/`TabItem`/`TabPanel`.

## 0.4.0
### added
- Configurable character of PaginationEllipsis.

### fixed
- ImageSize should store `is-square` instead of `is-Square` to html class attribute.

## 0.4.0-beta.0
The only real change here is that YBC has been updated to Yew 0.20, which included updating only a few components.

### changed
- Updated to yew 0.20
- Updated to yew-agent 0.2
- Updated to yew-router 0.17

## 0.3.0-beta.0
The only real change here is that YBC has been updated to Yew 0.19, which included updating basically all components. Huge shoutout to @Follpvosten for all of her work on making this happen.

Once this release has stabilized a bit, we will cut an official `0.3.0`, and will then follow-up with a `0.4.0` for Yew `0.20`.

### changed
- Updated to Yew 0.19.

## 0.2.0
### added
- Added support for yew `0.18.x`.
- Added `head_classes`, `body_classes` & `foot_classes` optional properties to the `Hero` component. These extra classes will be added to the stanard classes assigned to each of the corresponding hero component sections.
- Added `navburger_classes` prop to the `Navbar` component. These extra classes will be added to the standard classes for the `navbar-burger` element.
- Added `help_has_error` prop to the `Field` component. This `bool` prop will add the `is-danger` class to the field's help text when `true`.
- Added `horizontal` prop to the `Field` component. This replaces the `FieldHorizontal` component.
- Added `disabled` prop to the `Select` & `MultiSelect` components.
- All components have been updated to accept `yew::Classes` instead of `String` for their `classes` props.

### removed
- The `FieldHorizontal` component has been removed in favor of the `horizontal` prop of the `Field` component.

## 0.1.6
### changed
- Updates to the `Navbar` component:
    - `navbrand`, `navstart`, `navend` are now all optional.
    - a new `navburger: bool` property has been added. This bool controls whether or not a `navbar-burger` will be rendered inside of the navbar when being rendered within smaller viewports. This value defaults to `true`, maintaining backwards compatibility.

## 0.1.5
### fixed
- Fixed a few of the button & button-like components to use the HTML `disabled` attribute instead of the Bulma `is-disabled` CSS class. The latter has been deprecated for some time.
- Update docs in the README to point to the latest Bulma 0.9.1 release.

## 0.1.4
### added
- Add prop `padded` to Navbar. Setting this to true will wrap the contents of the navbar in a container to add padding to the contents under some circumstances.
- The default tag type for NavbarItems is now `div`.
- Added the `href`, `rel` & `target` props to the `NavbarItem` component. They will only be used when the `NavbarItemTag::A` is being used for the component.
- Added the `rel` & `target` props to the `ButtonAnchor` component.
- Adds an additional size for heroes: `is-fullheight-with-navbar`. This one is present in the Bulma docs, but is slightly hidden.

## 0.1.3
### fixed
- Removed `Children` props from Hero & made header & footer optional.

### added
- Added the `centered`, `multiline` & `vcentered` props to the `Columns` component, corresponding to the standard Bulma classes bearing the same name prefixed with `is-`.

## 0.1.0
- Initialize release. See the [release notes on Github](https://github.com/thedodd/ybc/releases/tag/v0.1.0) for more info.
