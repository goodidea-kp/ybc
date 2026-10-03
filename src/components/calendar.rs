//! Bulma Calendar with a working native date/time fallback when JavaScript is unavailable.
//!
//! `date` and callbacks use `date_format`: `yyyy-MM-dd` (default) or
//! `mm/dd/yyyy` (also `MM/dd/yyyy`). Unsupported formats safely fall back to ISO.
//! Date/time values use a space separator and 24-hour `HH:mm` time.
//! Empty values, whitespace and `None` clear the input. `disabled` prevents edits.
//! Load bulma-calendar 7.1.1 JS and CSS before mounting the app to enable the picker.
//! The widget uses ISO dates internally; form value conversion stays inside this component.
//! CSS: `https://cdn.jsdelivr.net/npm/bulma-calendar@7.1.1/dist/css/bulma-calendar.min.css`
//! JS: `https://cdn.jsdelivr.net/npm/bulma-calendar@7.1.1/dist/js/bulma-calendar.min.js`

use super::calendar_value::{from_native, to_native};
use web_sys::HtmlInputElement;
use yew::prelude::*;

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::{closure::Closure, prelude::*};

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen(module = "/src/components/calendar.js")]
extern "C" {
    #[wasm_bindgen(js_name = mountCalendar)]
    fn mount_calendar(host: &web_sys::Element, fallback: &HtmlInputElement, callback: &JsValue, kind: &str, mode: &str) -> JsValue;
    #[wasm_bindgen(js_name = syncCalendar)]
    fn sync_calendar(state: &JsValue, value: &str, disabled: bool);
    #[wasm_bindgen(js_name = unmountCalendar)]
    fn unmount_calendar(state: &JsValue);
}

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
    /// DOM id for labels and accessibility; not used to track picker instances.
    pub id: String,

    /// Form value format: `yyyy-MM-dd` (default) or `mm/dd/yyyy`.
    /// Unsupported formats fall back to `yyyy-MM-dd` without panicking.
    #[prop_or_default]
    pub date_format: AttrValue,

    /// A nonempty value enables date/time mode unless `calendar_type` is explicit.
    /// Both picker and fallback use 24-hour time; custom patterns are ignored.
    #[prop_or_default]
    pub time_format: AttrValue,

    /// Current form value. `None`, empty strings and whitespace clear the input.
    #[prop_or_default]
    pub date: Option<String>,

    /// Callback invoked when the date/time changes; receives empty string on clear.
    pub on_date_changed: Callback<String>,

    /// Extra classes appended after Bulma `input`.
    #[prop_or_default]
    pub class: Vec<String>,

    /// Optional test attribute on the input (`data-testid` or `data-cy`).
    #[prop_or_default]
    pub test_attr: Option<TestAttr>,

    /// Picker type (`date`, `time`, `datetime`).
    /// If empty, defaults to `datetime` when `time_format` is present, otherwise `date`.
    #[prop_or_default]
    pub calendar_type: AttrValue,

    /// Bulma display mode: `default`, `dialog` (for modals), or `inline`.
    /// Unsupported modes fall back to `default`; native fallback ignores this prop.
    #[prop_or_default]
    pub display_mode: AttrValue,

    /// Locks the input and the picker: nothing opens, nothing changes.
    #[prop_or_default]
    pub disabled: bool,
}

/// A Bulma picker with native fallback and conversion at the form boundary.
#[function_component(Calendar)]
pub fn calendar(props: &CalendarProps) -> Html {
    let input_ref = use_node_ref();
    let host_ref = use_node_ref();
    let input_type = match props.calendar_type.trim() {
        "time" => "time",
        "datetime" | "datetime-local" => "datetime-local",
        "date" => "date",
        _ if !props.time_format.trim().is_empty() => "datetime-local",
        _ => "date",
    };
    let value = to_native(props.date.as_deref().unwrap_or_default(), &props.date_format, input_type);
    let class = classes!("input", props.class.clone());

    #[cfg(target_arch = "wasm32")]
    {
        let state = use_mut_ref(|| JsValue::NULL);
        let current = use_mut_ref(|| props.clone());
        *current.borrow_mut() = props.clone();
        {
            let state = state.clone();
            let current = current.clone();
            let input_ref = input_ref.clone();
            let host_ref = host_ref.clone();
            use_effect_with(
                (
                    input_type,
                    props.display_mode.clone(),
                    props.class.clone(),
                    props.id.clone(),
                    props.test_attr.clone(),
                ),
                move |(kind, mode, _, _, _)| {
                    let kind = *kind;
                    let callback = Closure::wrap(Box::new(move |value: String| {
                        let props = current.borrow().clone();
                        if !props.disabled {
                            props
                                .on_date_changed
                                .emit(from_native(&value.replace(' ', "T"), &props.date_format, kind));
                        }
                    }) as Box<dyn FnMut(String)>);
                    if let (Some(host), Some(input)) = (host_ref.cast::<web_sys::Element>(), input_ref.cast::<HtmlInputElement>()) {
                        *state.borrow_mut() = mount_calendar(&host, &input, callback.as_ref(), kind, mode);
                    }
                    move || {
                        unmount_calendar(&state.borrow());
                        *state.borrow_mut() = JsValue::NULL;
                        drop(callback);
                    }
                },
            );
        }
        let value = value.clone();
        let disabled = props.disabled;
        use_effect(move || {
            sync_calendar(&state.borrow(), &value, disabled);
            || {}
        });
    }

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
            if !disabled {
                if let Some(input) = event.target_dyn_into::<HtmlInputElement>() {
                    callback.emit(from_native(&input.value(), &format, input_type));
                }
            }
        })
    };

    html! {
        <>
        <input
            ref={input_ref}
            id={props.id.clone()}
            class={class}
            type={input_type}
            value={value}
            onchange={onchange}
            disabled={props.disabled}
            data-testid={data_testid}
            data-cy={data_cy}
        />
        <div ref={host_ref} />
        </>
    }
}
