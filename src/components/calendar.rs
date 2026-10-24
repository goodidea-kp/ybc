//! A date field a keyboard types into, with a calendar popup beside it for the mouse.
//!
//! The field is the browser's own `<input type="date">` (or `time` /
//! `datetime-local`): typed into, stepped with the arrow keys, named by its
//! `<label for=id>` or by `aria_label`. For a date, a calendar button beside it
//! opens a picker in a native modal `<dialog>`:
//!
//! - it sits in the top layer, so a modal's body never clips it, and inside a
//!   [`crate::ModalCard`] Esc closes the picker, not the modal;
//! - the days are buttons with their full date as their name, one tab stop,
//!   moved with the arrow keys, Home/End and PageUp/PageDown (the WAI-ARIA date
//!   picker dialog pattern); Enter or a click picks the day;
//! - Esc, Close, a click outside or a pick closes it, and focus returns to the field;
//! - it is drawn with Bulma's scheme variables, so it follows the light and dark
//!   schemes; under `prefers-contrast: more` or `data-contrast="more"` its
//!   borders turn to the strong text colour.
//!
//! No script and no stylesheet of its own to load: the picker is Yew, its rules
//! a `<style>` it renders (a `style-src 'unsafe-inline'` page already allows
//! it). The browser's own picker indicator is hidden where it has one
//! (Chromium, WebKit), so a date field shows one calendar button.
//!
//! `date` and callbacks use `date_format`: `yyyy-MM-dd` (default) or
//! `mm/dd/yyyy` (also `MM/dd/yyyy`). Unsupported formats safely fall back to ISO.
//! Date/time values use a space separator and 24-hour `HH:mm` time.
//! Empty values, whitespace and `None` clear the input. `disabled` prevents edits.

use super::calendar_month::Day;
use super::calendar_popup::{CalendarPopup, DAY_ATTRIBUTE, title_id};
use super::calendar_value::{from_native, to_native};
use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::JsCast;
use web_sys::{HtmlDialogElement, HtmlElement, HtmlInputElement};
use yew::prelude::*;

/// The class on the field's wrapper (a Bulma `field has-addons`), for a page's own rules.
pub const CALENDAR_CLASS: &str = "ybc-calendar";

/// The picker's look, in Bulma's scheme variables only, so it changes with the
/// scheme. No CSS comments: this text reaches every page (see the modal's).
const CALENDAR_STYLE: &str = r#"
.ybc-calendar .input::-webkit-calendar-picker-indicator { display: none; }
dialog.ybc-calendar-popup { padding: 0; border: 1px solid var(--bulma-border); border-radius: var(--bulma-radius-large);
    background: var(--bulma-scheme-main); color: var(--bulma-text); box-shadow: var(--bulma-shadow); }
dialog.ybc-calendar-popup::backdrop { background: hsla(var(--bulma-scheme-h, 221deg), var(--bulma-scheme-s, 14%), var(--bulma-scheme-invert-l, 4%), 0.5); }
.ybc-calendar-body { padding: 0.75rem; }
.ybc-calendar-head, .ybc-calendar-foot { display: flex; align-items: center; justify-content: space-between; gap: 0.5rem; }
.ybc-calendar-foot { justify-content: flex-end; margin-top: 0.5rem; }
.ybc-calendar-title { font-weight: 600; color: var(--bulma-text-strong); }
.ybc-calendar-popup table { border-collapse: separate; border-spacing: 2px; margin-top: 0.5rem; background: transparent; color: inherit; }
.ybc-calendar-popup th { text-align: center; font-weight: 600; color: var(--bulma-text-weak); padding: 0.25rem 0; }
.ybc-calendar-popup td { padding: 0; }
.ybc-calendar-day { width: 2.5rem; height: 2.5rem; padding: 0; border-color: var(--bulma-border); }
.ybc-calendar-day[aria-current="date"] { border-color: var(--bulma-link); font-weight: 700; }
.ybc-calendar-popup .button:focus-visible { outline: 3px solid var(--bulma-link); outline-offset: 2px; }
@media (prefers-contrast: more) {
    dialog.ybc-calendar-popup, .ybc-calendar-popup .button { border-color: var(--bulma-text-strong); }
    dialog.ybc-calendar-popup { border-width: 2px; }
}
:root[data-contrast="more"] dialog.ybc-calendar-popup, :root[data-contrast="more"] .ybc-calendar-popup .button { border-color: var(--bulma-text-strong); }
:root[data-contrast="more"] dialog.ybc-calendar-popup { border-width: 2px; }
"#;

const SIZE_CLASSES: [&str; 3] = ["is-small", "is-medium", "is-large"];

/// The month the picker opens on when the field is empty and the date of today
/// is unknown (outside a browser).
const FALLBACK_DAY: Day = Day {
    year: 2000,
    month: 1,
    day: 1,
};

/// Optional test attribute rendered on the input.
///
/// Supported keys:
/// - `data-testid`
/// - `data-cy`
#[derive(Clone, Debug, PartialEq)]
pub struct TestAttr {
    pub key: AttrValue,
    pub value: AttrValue,
}

impl TestAttr {
    pub fn test_id(value: impl Into<AttrValue>) -> Self {
        Self {
            key: AttrValue::from("data-testid"),
            value: value.into(),
        }
    }

    pub fn data_cy(value: impl Into<AttrValue>) -> Self {
        Self {
            key: AttrValue::from("data-cy"),
            value: value.into(),
        }
    }
}

impl<T> From<T> for TestAttr
where
    T: Into<AttrValue>,
{
    fn from(value: T) -> Self {
        Self::test_id(value)
    }
}

/// Properties for [`Calendar`].
#[derive(Clone, PartialEq, Properties)]
pub struct CalendarProps {
    /// DOM id of the input, for its `<label for>`; the picker's ids start with it.
    pub id: String,

    /// Form value format: `yyyy-MM-dd` (default) or `mm/dd/yyyy`.
    /// Unsupported formats fall back to `yyyy-MM-dd` without panicking.
    #[prop_or_default]
    pub date_format: AttrValue,

    /// A nonempty value enables date/time mode unless `calendar_type` is explicit.
    /// The input uses 24-hour time; custom patterns are ignored.
    #[prop_or_default]
    pub time_format: AttrValue,

    /// Current form value. `None`, empty strings and whitespace clear the input.
    #[prop_or_default]
    pub date: Option<String>,

    /// Callback invoked when the date/time changes; receives empty string on clear.
    pub on_date_changed: Callback<String>,

    /// Extra classes appended after Bulma `input` (`is-danger`, ...).
    #[prop_or_default]
    pub class: Vec<String>,

    /// Optional test attribute on the input (`data-testid` or `data-cy`). A
    /// `data-testid` of `X` also names the picker button `X-picker`, its dialog
    /// `X-picker-dialog` and the dialog's buttons `X-picker-previous|next|close`.
    #[prop_or_default]
    pub test_attr: Option<TestAttr>,

    /// Input type (`date`, `time`, `datetime`).
    /// If empty, defaults to `datetime` when `time_format` is present, otherwise `date`.
    /// Only `date` has the picker; the others are the browser's own control alone.
    #[prop_or_default]
    pub calendar_type: AttrValue,

    /// Kept for callers of the bulma-calendar picker; the picker is always a
    /// modal dialog now, which works inside a modal as well as outside one.
    #[prop_or_default]
    pub display_mode: AttrValue,

    /// Locks the input and the picker: nothing opens, nothing changes.
    #[prop_or_default]
    pub disabled: bool,

    /// The input's name when no `<label for>` names it ("From date").
    #[prop_or_default]
    pub aria_label: Option<AttrValue>,

    /// Ids of the lines that describe the input (hint, error).
    #[prop_or_default]
    pub aria_describedby: Option<AttrValue>,

    /// Marks the input `aria-invalid="true"`; pair it with `is-danger` in `class`.
    #[prop_or_default]
    pub invalid: bool,

    /// The picker button's name. Defaults to "Choose <aria_label> from a
    /// calendar", or "Choose a date from a calendar"; give one per field when a
    /// form has several dates and no `aria_label`.
    #[prop_or_default]
    pub picker_label: Option<AttrValue>,
}

impl CalendarProps {
    fn input_type(&self) -> &'static str {
        match self.calendar_type.trim() {
            "time" => "time",
            "datetime" | "datetime-local" => "datetime-local",
            "date" => "date",
            _ if !self.time_format.trim().is_empty() => "datetime-local",
            _ => "date",
        }
    }

    fn picker_name(&self) -> String {
        match (&self.picker_label, &self.aria_label) {
            (Some(label), _) => label.to_string(),
            (None, Some(field)) => format!("Choose {field} from a calendar"),
            (None, None) => "Choose a date from a calendar".to_owned(),
        }
    }

    /// The input's Bulma size (`is-small`, ...), which the button takes too so
    /// the two stay the same height.
    fn size_class(&self) -> Option<String> {
        self.class.iter().find(|class| SIZE_CLASSES.contains(&class.as_str())).cloned()
    }

    fn testid(&self) -> Option<AttrValue> {
        self.test_attr
            .as_ref()
            .filter(|attr| attr.key == "data-testid")
            .map(|attr| attr.value.clone())
    }
}

/// A typable date field with a calendar picker for the mouse.
#[function_component(Calendar)]
pub fn calendar(props: &CalendarProps) -> Html {
    let input_type = props.input_type();
    let value = to_native(props.date.as_deref().unwrap_or_default(), &props.date_format, input_type);
    let input = date_input(props, input_type, &value);
    if input_type != "date" {
        return input;
    }
    html! {
        // Bulma addons: the input and the button read as one control. The
        // <style> sits in the button's control, so the input's control stays
        // the field's :first-child and keeps its rounded start.
        <div class={classes!("field", "has-addons", CALENDAR_CLASS)}>
            <div class="control is-expanded">{ input }</div>
            <div class="control">
                <Picker field={props.clone()} selected={Day::parse_iso(&value)} />
                <style>{CALENDAR_STYLE}</style>
            </div>
        </div>
    }
}

fn date_input(props: &CalendarProps, input_type: &'static str, value: &str) -> Html {
    let (data_testid, data_cy) = match props.test_attr.as_ref() {
        Some(attr) if attr.key == "data-testid" => (Some(attr.value.clone()), None),
        Some(attr) if attr.key == "data-cy" => (None, Some(attr.value.clone())),
        _ => (None, None),
    };
    let onchange = {
        let callback = props.on_date_changed.clone();
        let format = props.date_format.clone();
        let disabled = props.disabled;
        Callback::from(move |event: Event| {
            if !disabled && let Some(input) = event.target_dyn_into::<HtmlInputElement>() {
                callback.emit(from_native(&input.value(), &format, input_type));
            }
        })
    };
    html! {
        <input
            id={props.id.clone()}
            class={classes!("input", props.class.clone())}
            type={input_type}
            value={value.to_owned()}
            {onchange}
            disabled={props.disabled}
            aria-label={props.aria_label.clone()}
            aria-describedby={props.aria_describedby.clone()}
            aria-invalid={props.invalid.then_some("true")}
            data-testid={data_testid}
            data-cy={data_cy}
        />
    }
}

#[derive(Clone, PartialEq, Properties)]
struct PickerProps {
    field: CalendarProps,
    selected: Option<Day>,
}

/// The calendar button and its dialog. Open state lives here; the dialog's own
/// `close` event is the one place that ends it, whatever closed it.
#[function_component(Picker)]
fn picker(props: &PickerProps) -> Html {
    let dialog_ref = use_node_ref();
    let focused = use_state(|| None::<Day>);
    let focus_grid = use_mut_ref(|| false);
    {
        let dialog_ref = dialog_ref.clone();
        let focus_grid = focus_grid.clone();
        use_effect_with(*focused, move |focused| {
            show_dialog(&dialog_ref, *focused, &focus_grid);
            || {}
        });
    }
    let field = &props.field;
    let popup_id = format!("{}-picker-dialog", field.id);
    let testid = field.testid();
    let open = open_picker(focused.clone(), focus_grid.clone(), props.selected, field.disabled);
    let close = close_picker(dialog_ref.clone());
    let on_focus = {
        let (focused, focus_grid) = (focused.clone(), focus_grid.clone());
        Callback::from(move |(day, onto_grid): (Day, bool)| {
            *focus_grid.borrow_mut() = onto_grid;
            focused.set(Some(day));
        })
    };
    let on_pick = {
        let (callback, format, close) = (field.on_date_changed.clone(), field.date_format.clone(), close.clone());
        Callback::from(move |day: Day| {
            callback.emit(from_native(&day.iso(), &format, "date"));
            close.emit(());
        })
    };
    html! {
        <>
            <button type="button" class={classes!("button", field.size_class())} aria-label={field.picker_name()} aria-haspopup="dialog"
                    aria-expanded={focused.is_some().to_string()} aria-controls={popup_id.clone()}
                    disabled={field.disabled} data-testid={testid.as_ref().map(|testid| format!("{testid}-picker"))}
                    onclick={open}>
                <span class="icon"><i class="fas fa-calendar-days" aria-hidden="true"></i></span>
            </button>
            <dialog ref={dialog_ref.clone()} id={popup_id} class="ybc-calendar-popup" aria-labelledby={title_id(&field.id)}
                    data-testid={testid.as_ref().map(|testid| format!("{testid}-picker-dialog"))}
                    oncancel={Callback::from(|event: Event| event.stop_propagation())}
                    onclose={on_dialog_closed(focused.clone(), field.id.clone())}
                    onclick={on_backdrop_click(dialog_ref, close.clone())}>
                if let Some(day) = *focused {
                    <CalendarPopup id={field.id.clone()} focused={day} selected={props.selected} today={today()}
                                   {on_pick} {on_focus} on_close={close}
                                   testid={testid.map(|testid| AttrValue::from(format!("{testid}-picker")))} />
                }
            </dialog>
        </>
    }
}

fn open_picker(focused: UseStateHandle<Option<Day>>, focus_grid: Rc<RefCell<bool>>, selected: Option<Day>, disabled: bool) -> Callback<MouseEvent> {
    Callback::from(move |_| {
        if disabled {
            return;
        }
        *focus_grid.borrow_mut() = true;
        focused.set(Some(selected.or_else(today).unwrap_or(FALLBACK_DAY)));
    })
}

fn close_picker(dialog_ref: NodeRef) -> Callback<()> {
    Callback::from(move |()| {
        if let Some(dialog) = dialog_ref.cast::<HtmlDialogElement>()
            && dialog.open()
        {
            dialog.close();
        }
    })
}

/// Opens the dialog for a focused day and, after a key or the opening click,
/// puts focus on that day's button.
fn show_dialog(dialog_ref: &NodeRef, focused: Option<Day>, focus_grid: &Rc<RefCell<bool>>) {
    let (Some(dialog), Some(day)) = (dialog_ref.cast::<HtmlDialogElement>(), focused) else {
        return;
    };
    if !dialog.open() && dialog.show_modal().is_err() {
        return;
    }
    if !focus_grid.replace(false) {
        return;
    }
    let selector = format!(r#"[{DAY_ATTRIBUTE}="{}"]"#, day.iso());
    if let Ok(Some(button)) = dialog.query_selector(&selector)
        && let Ok(button) = button.dyn_into::<HtmlElement>()
    {
        let _ = button.focus();
    }
}

/// Esc, Close, a pick or a click outside all end here: the picker forgets its
/// month and focus goes back to the field (WCAG 2.4.3). The event stops here,
/// or Yew would bubble it to an enclosing modal, which would close too.
fn on_dialog_closed(focused: UseStateHandle<Option<Day>>, input_id: String) -> Callback<Event> {
    Callback::from(move |event: Event| {
        event.stop_propagation();
        focused.set(None);
        let input = web_sys::window()
            .and_then(|window| window.document())
            .and_then(|document| document.get_element_by_id(&input_id));
        if let Some(input) = input.and_then(|input| input.dyn_into::<HtmlElement>().ok()) {
            let _ = input.focus();
        }
    })
}

/// A click on the dialog element itself is a click on its backdrop: the
/// content fills the dialog, so any click inside lands on a child.
fn on_backdrop_click(dialog_ref: NodeRef, close: Callback<()>) -> Callback<MouseEvent> {
    Callback::from(move |event: MouseEvent| {
        let on_dialog = event
            .target()
            .zip(dialog_ref.get())
            .is_some_and(|(target, dialog)| target.dyn_ref::<web_sys::Node>().is_some_and(|node| node.is_same_node(Some(&dialog))));
        if on_dialog {
            close.emit(());
        }
    })
}

/// Today in the browser's time zone; unknown outside a browser.
fn today() -> Option<Day> {
    #[cfg(target_arch = "wasm32")]
    {
        let now = web_sys::js_sys::Date::new_0();
        Day::new(now.get_full_year(), now.get_month() + 1, now.get_date())
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::CALENDAR_STYLE;

    #[test]
    fn the_shipped_style_carries_no_css_comment() {
        assert!(!CALENDAR_STYLE.contains("/*"), "a CSS comment reached the page");
    }

    #[test]
    fn the_style_follows_the_scheme_and_raises_contrast_on_request() {
        for token in [
            "var(--bulma-scheme-main)",
            "var(--bulma-text)",
            "var(--bulma-border)",
            "prefers-contrast: more",
            r#"data-contrast="more""#,
        ] {
            assert!(CALENDAR_STYLE.contains(token), "lost {token}");
        }
        assert!(
            !CALENDAR_STYLE.contains('#') && !CALENDAR_STYLE.contains("rgb"),
            "a literal colour does not follow the scheme"
        );
        assert!(CALENDAR_STYLE.contains("--bulma-scheme-invert-l"), "the backdrop is Bulma's modal shade");
    }
}
