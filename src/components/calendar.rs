use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsValue;
use wasm_bindgen::prelude::wasm_bindgen;
use web_sys::Element;
use yew::{Callback, Component, Context, Html};
use yew::prelude::*;

pub struct Calendar {
    format: String,
    date: Option<String>,
    id: String,
}


#[derive(Clone, PartialEq, Properties)]
pub struct CalendarProps {
    pub id: String,
    #[prop_or_default]
    pub date_format: String,
    #[prop_or_default]
    pub time_format: String,
    pub date: Option<String>,
    pub on_date_changed: Callback<String>,
    pub class: Vec<String>,
}

pub enum Msg {
    DateChanged(String),
}

impl Component for Calendar {
    type Message = Msg;
    type Properties = CalendarProps;

    fn create(ctx: &Context<Self>) -> Self {
        Calendar {
            format: "%Y-%m-%d %H:%M".to_string(),
            date: ctx.props().date.clone(),
            id: ctx.props().id.clone(),
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::DateChanged(date) => {
                self.date = Some(date.clone());
                ctx.props().on_date_changed.emit(date);
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let _value = self.date.clone();
        let _id = self.id.clone();
        let classes = classes!("input", ctx.props().class.clone());
        html! {<input id={_id} class={classes!(classes)} type="datetime"
                                             value={_value} />
        }
    }

    fn rendered(&mut self, ctx: &Context<Self>, first_render: bool) {
        if first_render {
            let window = web_sys::window().expect("no global `window` exists");
            let document = window.document().expect("should have a document on window");

            let element = document
                .get_element_by_id(self.id.as_str())
                .expect(format!("should have #{} on the page", self.id.as_str()).as_str());

            // Clone the link from the context
            let link = ctx.link().clone();

            // Move the cloned link into the closure
            let callback = Closure::wrap(Box::new(move |date: JsValue| {
                let date_str = date.as_string().unwrap_or_default();
                link.send_message(Msg::DateChanged(date_str));
            }) as Box<dyn FnMut(JsValue)>);

            unsafe {
                let _date_format = if ctx.props().date_format.trim().len() == 0 {
                    "yyyy-MM-dd".to_string()
                } else {
                    ctx.props().date_format.trim().to_string()
                };

                let _time_format = if ctx.props().time_format.trim().len() == 0 {
                    "HH:mm".to_string()
                } else {
                    ctx.props().time_format.trim().to_string()
                };

                setup_date_picker(&element, callback.as_ref(), &JsValue::from(self.date.as_ref().unwrap_or(&"".to_string())),
                &JsValue::from(_date_format), &JsValue::from(_time_format));
            }

            // Don't forget to forget the callback, otherwise it will be cleaned up when it goes out of scope.
            callback.forget();
        }
    }


    fn destroy(&mut self, ctx: &Context<Self>) {
        unsafe {
            detach_date_picker(&JsValue::from(self.id.as_str()));
        }
    }
}

#[wasm_bindgen(inline_js = r#"
let init = new Map();
export function setup_date_picker(element, callback, initial_date, date_format, time_format) {
    // console.log('Setting up date picker ID:' + element.id + 'date format:' + date_format + ' time format:' + time_format);
    if (!init.has(element.id)) {
      let calendarInstances = bulmaCalendar.attach(element, {
            type: 'datetime',
            color: 'info',
            lang: 'en',
            dateFormat: date_format,
            timeFormat: time_format,
            // other options...
        });
        init.set(element.id, calendarInstances[0]);
        let calendarInstance = calendarInstances[0];
        calendarInstance.on('select', function(datepicker) {
        // console.log("Selected date: " + datepicker.data.value());
         callback(datepicker.data.value());
        // You can add more code here to handle the selected date
    });
    }
    // console.log('Setting up date picker:' + initial_date);
    // console.dir(bulmaCalendar);
    init.get(element.id).value(initial_date);

    // Define the callback function here


}

export function detach_date_picker(id) {
    init.delete(id);
    // console.log('Detaching date picker #id='+id+'!');
}


"#)]
extern "C" {
    fn setup_date_picker(element: &Element, callback: &JsValue, initial_date: &JsValue, date_format: &JsValue, time_format: &JsValue);
    fn detach_date_picker(id: &JsValue);
}
