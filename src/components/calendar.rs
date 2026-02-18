/*!
Calendar component: a thin Yew wrapper around the bulma-calendar JS date/time picker.

Summary
- Enhances a plain `<input>` with bulmaCalendar for date and time selection.
- Emits changes through a Rust callback whenever the user selects, validates, or clears.
- Requires bulmaCalendar JS and CSS to be loaded globally (available as `bulmaCalendar`).

Value format
- The emitted string follows the configured `date_format` and `time_format` patterns understood by bulmaCalendar.
- Clearing the picker emits an empty string.

Programmatic control
- To update the picker value from the outside, update the `date` prop.
- To clear the picker from the outside, set `date` to a single space `" "`.

Required static assets
- CSS (add in `<head>`):
  https://cdn.jsdelivr.net/npm/bulma-calendar@7.1.1/dist/css/bulma-calendar.min.css
- JS (load before WASM bootstrap so `bulmaCalendar` exists):
  https://cdn.jsdelivr.net/npm/bulma-calendar@7.1.1/dist/js/bulma-calendar.min.js
*/

use yew::prelude::*;

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::JsCast;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::JsValue;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::closure::Closure;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::wasm_bindgen;
#[cfg(target_arch = "wasm32")]
use web_sys::Element;

#[cfg(target_arch = "wasm32")]
type CalendarClosure = Closure<dyn FnMut(JsValue)>;
#[cfg(not(target_arch = "wasm32"))]
type CalendarClosure = ();

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
    /// Unique DOM id for the input (used to attach/detach the JS widget).
    pub id: String,

    /// Date format understood by bulmaCalendar. Defaults to `yyyy-MM-dd` when empty.
    #[prop_or_default]
    pub date_format: AttrValue,

    /// Time format understood by bulmaCalendar. Defaults to `HH:mm` when empty.
    #[prop_or_default]
    pub time_format: AttrValue,

    /// Optional initial/current value to seed or update the widget.
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
}

/// A date/time input enhanced by bulma-calendar.
#[function_component(Calendar)]
pub fn calendar(props: &CalendarProps) -> Html {
    let input_ref = use_node_ref();

    let date_format_raw = props.date_format.trim().to_string();
    assert!(
        date_format_raw.is_empty() || date_format_raw == "yyyy-MM-dd",
        "Calendar date_format must be exactly 'yyyy-MM-dd' (lowercase yyyy-MM-dd). Got '{}'",
        props.date_format
    );

    let date_format = if date_format_raw.is_empty() {
        "yyyy-MM-dd".to_owned()
    } else {
        date_format_raw
    };

    let time_format_raw = props.time_format.trim().to_string();
    let time_format = if time_format_raw.is_empty() {
        "HH:mm".to_owned()
    } else {
        time_format_raw.clone()
    };

    let calendar_type = {
        let explicit = props.calendar_type.trim();
        if explicit.is_empty() {
            if props.time_format.trim().is_empty() {
                "date".to_owned()
            } else {
                "datetime".to_owned()
            }
        } else {
            explicit.to_owned()
        }
    };

    let initial_value = props.date.clone().unwrap_or_default();
    let class = classes!("input", props.class.clone());

    let (data_testid, data_cy) = match props.test_attr.as_ref() {
        Some(attr) if attr.key == "data-testid" => (Some(attr.value.clone()), None),
        Some(attr) if attr.key == "data-cy" => (None, Some(attr.value.clone())),
        _ => (None, None),
    };

    let input_type = if props.time_format.trim().is_empty() {
        AttrValue::from("date")
    } else {
        AttrValue::from("datetime")
    };

    let callback_store = use_mut_ref(|| None::<CalendarClosure>);
    let on_date_changed_ref = use_mut_ref(|| props.on_date_changed.clone());
    *on_date_changed_ref.borrow_mut() = props.on_date_changed.clone();

    {
        let id = props.id.clone();
        let input_ref = input_ref.clone();
        let callback_store = callback_store.clone();
        let on_date_changed_ref = on_date_changed_ref.clone();
        let date_format = date_format.clone();
        let time_format = time_format.clone();
        let calendar_type = calendar_type.clone();
        let initial_value = initial_value.clone();

        use_effect_with(
            (id.clone(), date_format.clone(), time_format.clone(), calendar_type.clone()),
            move |(id, date_format, time_format, calendar_type)| {
                #[cfg(target_arch = "wasm32")]
                {
                    if let Some(element) = input_ref.cast::<Element>() {
                        let on_date_changed_ref = on_date_changed_ref.clone();
                        let callback = Closure::wrap(Box::new(move |date: JsValue| {
                            let s = date.as_string().unwrap_or_default();
                            on_date_changed_ref.borrow().emit(s);
                        }) as Box<dyn FnMut(JsValue)>);

                        setup_date_picker(
                            &element,
                            callback.as_ref(),
                            &JsValue::from(initial_value.clone()),
                            &JsValue::from(date_format.clone()),
                            &JsValue::from(time_format.clone()),
                            &JsValue::from(calendar_type.clone()),
                        );

                        *callback_store.borrow_mut() = Some(callback);
                    }

                    let callback_store = callback_store.clone();
                    let id_for_cleanup = id.clone();
                    return move || {
                        detach_date_picker(&JsValue::from(id_for_cleanup.as_str()));
                        callback_store.borrow_mut().take();
                    };
                }

                #[cfg(not(target_arch = "wasm32"))]
                {
                    let _ = (
                        &input_ref,
                        &callback_store,
                        &on_date_changed_ref,
                        &initial_value,
                        id,
                        date_format,
                        time_format,
                        calendar_type,
                    );
                    || {}
                }
            },
        );
    }

    {
        let id = props.id.clone();
        let date = props.date.clone();
        use_effect_with((id, date), move |(id, date)| {
            #[cfg(target_arch = "wasm32")]
            {
                match date.as_deref() {
                    Some(" ") | Some("") => {
                        clear_date(&JsValue::from(id.as_str()));
                    }
                    Some(v) => {
                        update_value(&JsValue::from(id.as_str()), &JsValue::from(v));
                    }
                    None => {}
                }
            }
            #[cfg(not(target_arch = "wasm32"))]
            {
                let _ = (id, date);
            }

            || {}
        });
    }

    html! {
        <input
            id={props.id.clone()}
            class={class}
            type={input_type}
            value={initial_value}
            ref={input_ref}
            data-testid={data_testid}
            data-cy={data_cy}
        />
    }
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen(inline_js = r#"
let init = new Map();

export function setup_date_picker(element, callback, initial_date, date_format, time_format, picker_type) {
    if (!element || !element.id) {
        return;
    }

    if (typeof bulmaCalendar === 'undefined' || typeof bulmaCalendar.attach !== 'function') {
        console.warn('bulmaCalendar is not available on window. Calendar will remain a plain input.');
        return;
    }

    if (!init.has(element.id)) {
        const instances = bulmaCalendar.attach(element, {
            type: picker_type || (String(time_format || '').trim() ? 'datetime' : 'date'),
            color: 'info',
            lang: 'en',
            dateFormat: date_format,
            timeFormat: time_format,
            showTodayButton: false
        });

        if (!instances || !instances.length) {
            return;
        }

        const calendarInstance = instances[0];
        init.set(element.id, calendarInstance);

        calendarInstance.on('select', function(datepicker) {
            callback(datepicker.data.value());
        });

        calendarInstance.on('clear', function(_datepicker) {
            callback('');
        });

        calendarInstance.on('validate', function(datepicker) {
            callback(datepicker.data.value());
            if (typeof calendarInstance.hide === 'function') {
                calendarInstance.hide();
            }
        });
    }

    if (init.has(element.id)) {
        init.get(element.id).value(initial_date || '');
    }
}

export function detach_date_picker(id) {
    if (init.has(id)) {
        const instance = init.get(id);
        if (instance && typeof instance.destroy === 'function') {
            instance.destroy();
        }
        init.delete(id);
    }
}

export function clear_date(id) {
    if (init.has(id)) {
        init.get(id).clear();
    }
}

export function update_value(id, value) {
    if (init.has(id)) {
        init.get(id).value(value || '');
    }
}
"#)]
#[allow(improper_ctypes, improper_ctypes_definitions)]
extern "C" {
    fn setup_date_picker(
        element: &Element, callback: &JsValue, initial_date: &JsValue, date_format: &JsValue, time_format: &JsValue, picker_type: &JsValue,
    );

    fn detach_date_picker(id: &JsValue);

    fn clear_date(id: &JsValue);

    fn update_value(id: &JsValue, value: &JsValue);
}
