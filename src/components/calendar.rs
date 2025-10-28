//! Calendar component: a thin Yew wrapper around the bulma‑calendar JS date/time picker.
//!
//! Summary
//! - Enhances a plain `<input>` with bulmaCalendar for date and time selection.
//! - Emits changes through a Yew `Callback<String>` whenever the user selects, validates, or clears.
//! - Requires bulmaCalendar JS and CSS to be loaded globally (available as `bulmaCalendar`).
//!
//! Value format
//! - The emitted string follows the configured `date_format` and `time_format` patterns understood by bulmaCalendar.
//! - Clearing the picker emits an empty string.
//!
//! Usage
//! ```rust
//! use yew::prelude::*;
//! use crate::components::calendar::Calendar;
//!
//! #[function_component(Form)]
//! fn form() -> Html {
//!     let date = use_state(|| Option::<String>::None);
//!     let on_date_changed = {
//!         let date = date.clone();
//!         Callback::from(move |d: String| {
//!             date.set(if d.is_empty() { None } else { Some(d) });
//!         })
//!     };
//!
//!     html! {
//!         <Calendar
//!             id="appointment"
//!             date_format="yyyy-MM-dd"
//!             time_format="HH:mm"
//!             date={(*date).clone()}
//!             on_date_changed={on_date_changed}
//!             class={vec!["is-small".into()]}
//!         />
//!     }
//! }
//! ```
//!
//! Required static assets
//! - Add the bulma‑calendar CSS into your HTML <head>:
//!   <link rel="stylesheet" href="https://cdn.jsdelivr.net/npm/bulma-calendar@6.1.19/dist/css/bulma-calendar.min.css"/>
//!
//! - Add the bulma‑calendar JS so `bulmaCalendar` is available on window. Place this before your wasm bootstrap script
//!   (or ensure it loads before your Yew app mounts):
//!   <script src="https://cdn.jsdelivr.net/npm/bulma-calendar@7.1.1/dist/js/bulma-calendar.min.js"></script>
//!
//! How to configure index.html
//! - Minimal example (place CSS in <head>, script before the wasm init script):
//!   ```html
//!   <!doctype html>
//!   <html>
//!   <head>
//!     <meta charset="utf-8" />
//!     <meta name="viewport" content="width=device-width,initial-scale=1" />
//!     <!-- bulma-calendar CSS -->
//!     <link rel="stylesheet" href="https://cdn.jsdelivr.net/npm/bulma-calendar@6.1.19/dist/css/bulma-calendar.min.css"/>
//!   </head>
//!   <body>
//!     <div id="root"></div>
//!
//!     <!-- bulma-calendar JS: ensure this runs before your wasm bootstrap so `bulmaCalendar` exists -->
//!     <script src="https://cdn.jsdelivr.net/npm/bulma-calendar@7.1.1/dist/js/bulma-calendar.min.js"></script>
//!
//!     <!-- Your wasm/bootstrap script that starts the Yew app -->
//!     <script type="module">
//!       import init from './pkg/your_crate.js';
//!       init();
//!     </script>
//!   </body>
//!   </html>
//!   ```
//!
//! Notes and alternatives
//! - If you use a bundler (webpack, vite, etc.) you can install bulma-calendar from npm and import it in your JS entry:
//!     npm install bulma-calendar
//!     // in your entry file
//!     import 'bulma-calendar/dist/css/bulma-calendar.min.css';
//!     import bulmaCalendar from 'bulma-calendar';
//!   Ensure the import runs before the Yew bootstrap so `bulmaCalendar` is available globally (or adapt the setup to pass the module).
//!
//! - The important requirement: bulmaCalendar must be defined on window when setup_date_picker is called in the component's rendered() hook.
//!
//! - If you prefer loading the script asynchronously, make sure to delay Yew app start until bulmaCalendar is available (e.g. listen for script load).
//!
//! Implementation notes
//! - The `id` must be unique in the DOM; it is used to attach and detach the JS calendar instance.
//! - The underlying `<input>` uses `type="datetime"` for the JS widget; the plugin drives rendering and value handling.
//!
//! ...

use wasm_bindgen::JsValue;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::prelude::wasm_bindgen;
use web_sys::Element;
use yew::prelude::*;
use yew::{Callback, Component, Context, Html};

/// Internal component state for the calendar widget.
pub struct Calendar {
    /// Local format placeholder (currently unused by the widget; formats are passed to JS).
    format: String,
    /// Current date/time value as a string matching the widget's configured formats.
    date: Option<String>,
    /// DOM id of the `<input>` used to attach bulmaCalendar.
    id: String,
}

/// Properties for the `Calendar` component.
///
/// - `id`: required, unique DOM id for the input (used to attach/detach the JS widget).
/// - `date_format`: optional; bulmaCalendar date pattern (default: `yyyy-MM-dd`).
/// - `time_format`: optional; bulmaCalendar time pattern (default: `HH:mm`).
/// - `date`: optional initial/current value; will be pushed into the widget on first render.
/// - `on_date_changed`: invoked on select/validate/clear with the current value (empty on clear).
/// - `class`: optional extra CSS classes appended to the input, e.g., `vec!["is-small".into()]`.
#[derive(Clone, PartialEq, Properties)]
pub struct CalendarProps {
    /// Unique DOM id of the input element.
    pub id: String,
    /// bulmaCalendar date pattern, e.g., `yyyy-MM-dd`. Falls back to `yyyy-MM-dd` if empty.
    #[prop_or_default]
    pub date_format: String,
    /// bulmaCalendar time pattern, e.g., `HH:mm`. Falls back to `HH:mm` if empty.
    #[prop_or_default]
    pub time_format: String,
    /// Optional initial/current value for the calendar.
    pub date: Option<String>,
    /// Callback fired whenever the date/time value changes. Receives empty string on clear.
    pub on_date_changed: Callback<String>,
    /// Extra CSS classes for the input.
    pub class: Vec<String>,
}

/// Internal messages for the component update loop.
pub enum Msg {
    /// User changed the date/time value through the widget.
    DateChanged(String),
}

impl Component for Calendar {
    type Message = Msg;
    type Properties = CalendarProps;

    /// Initializes the component with provided props and default format strings.
    fn create(ctx: &Context<Self>) -> Self {
        Calendar {
            format: "%Y-%m-%d %H:%M".to_string(),
            date: ctx.props().date.clone(),
            id: ctx.props().id.clone(),
        }
    }

    /// Handles internal messages by updating local state and notifying the parent via callback.
    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::DateChanged(date) => {
                self.date = Some(date.clone());
                ctx.props().on_date_changed.emit(date);
                true
            }
        }
    }

    /// Renders the backing input element. The bulmaCalendar widget attaches to this element.
    fn view(&self, ctx: &Context<Self>) -> Html {
        let _value = self.date.clone();
        let _id = self.id.clone();
        let classes = classes!("input", ctx.props().class.clone());
        html! {
            <input id={_id} class={classes!(classes)} type="datetime" value={_value} />
        }
    }

    /// After first render, attaches the bulmaCalendar instance and wires event callbacks.
    fn rendered(&mut self, ctx: &Context<Self>, first_render: bool) {
        if first_render {
            let window = web_sys::window().expect("no global `window` exists");
            let document = window.document().expect("should have a document on window");

            let element = document
                .get_element_by_id(self.id.as_str())
                .expect(format!("should have #{} on the page", self.id.as_str()).as_str());

            // Clone the link for use inside the JS callback closure.
            let link = ctx.link().clone();

            // JS -> Rust bridge: receive the string value and forward as a Yew message.
            let callback = Closure::wrap(Box::new(move |date: JsValue| {
                let date_str = date.as_string().unwrap_or_default();
                link.send_message(Msg::DateChanged(date_str));
            }) as Box<dyn FnMut(JsValue)>);

            // Resolve formats, falling back to sensible defaults if props are empty.
            let _date_format = if ctx.props().date_format.trim().is_empty() {
                "yyyy-MM-dd".to_string()
            } else {
                ctx.props().date_format.trim().to_string()
            };

            let _time_format = if ctx.props().time_format.trim().is_empty() {
                "HH:mm".to_string()
            } else {
                ctx.props().time_format.trim().to_string()
            };

            // Attach the JS widget and seed its initial value.
            setup_date_picker(
                &element,
                callback.as_ref(),
                &JsValue::from(self.date.as_ref().unwrap_or(&"".to_string())),
                &JsValue::from(_date_format),
                &JsValue::from(_time_format),
            );

            // Intentionally leak the closure to keep it alive for the widget's lifetime.
            callback.forget();
        }
    }

    /// Before unmount, detach JS state keyed by the element id.
    fn destroy(&mut self, _ctx: &Context<Self>) {
        detach_date_picker(&JsValue::from(self.id.as_str()));
    }
}

/// JS bridge that attaches bulmaCalendar to the provided element and wires a change callback.
///
/// Safety/expectations:
/// - `element` must be an `<input>` present in the DOM with a stable `id`.
/// - `callback` must remain alive for as long as the widget can invoke it (we call `forget()`).
/// - bulmaCalendar must be globally available.
#[wasm_bindgen(inline_js = r#"
let init = new Map();
export function setup_date_picker(element, callback, initial_date, date_format, time_format) {
    if (!init.has(element.id)) {
      let calendarInstances = bulmaCalendar.attach(element, {
            type: 'datetime',
            color: 'info',
            lang: 'en',
            dateFormat: date_format,
            timeFormat: time_format,
        });
        init.set(element.id, calendarInstances[0]);
        let calendarInstance = calendarInstances[0];
        calendarInstance.on('select', function(datepicker) {
         callback(datepicker.data.value());
        });
        calendarInstance.on('clear', function(_datepicker) {
         callback('');
        });
        calendarInstance.on('validate', function(datepicker) {
         callback(datepicker.data.value());
        });
    }
    init.get(element.id).value(initial_date);
}

export function detach_date_picker(id) {
    init.delete(id);
}
"#)]
extern "C" {
    /// Attach bulmaCalendar to `element`, register `callback`, seed with `initial_date`,
    /// and apply `date_format`/`time_format`.
    fn setup_date_picker(
        element: &Element,
        callback: &JsValue,
        initial_date: &JsValue,
        date_format: &JsValue,
        time_format: &JsValue,
    );

    /// Remove the stored calendar instance keyed by the given `id`.
    fn detach_date_picker(id: &JsValue);
}