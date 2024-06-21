use wasm_bindgen::closure::Closure;
use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::JsCast;
use wasm_bindgen::JsValue;
use web_sys::js_sys::{Reflect, JSON};
use web_sys::{js_sys, Element};
use yew::prelude::*;

pub struct AutoComplete {
    current_selector: String,
    id: String,
    items: Vec<String>,
}

#[derive(Clone, PartialEq, Properties)]
pub struct AutoCompleteProps {
    #[prop_or("autocomplete".to_string())]
    pub id: String,
    #[prop_or(10)]
    pub max_items: u32,
    #[prop_or_default]
    pub items: Vec<String>,
    pub on_update: Callback<String>,
    pub on_remove: Callback<String>,
    #[prop_or("".to_string())]
    pub current_selector: String,
    #[prop_or("Choose Tags".to_string())]
    pub placeholder: String,
    #[prop_or(classes!("".to_string()))]
    pub classes: Classes,
    #[prop_or(false)]
    pub is_multiple: bool,
}

pub enum Msg {
    Added(String),
    Removed(String),
}

impl Component for AutoComplete {
    type Message = Msg;
    type Properties = AutoCompleteProps;
    fn create(ctx: &Context<Self>) -> Self {
        Self {
            current_selector: ctx.props().current_selector.clone(),
            id: ctx.props().id.clone(),
            items: ctx.props().items.clone(),
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Added(tag) => {
                ctx.props().on_update.emit(tag);
                // gloo_console::log!("Added: {}", tag.as_str());
            }
            Msg::Removed(tag) => {
                ctx.props().on_remove.emit(tag);
                // gloo_console::log!("Removed: {}", tag.as_str());
            }
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let items = self
            .items
            .iter()
            .clone()
            .map(|item| {
                if item == &self.current_selector {
                    html! {
                         <option value={item.clone()} selected=true>{item.clone()}</option>
                    }
                } else {
                    html! {
                         <option value={item.clone()}>{item.clone()}</option>
                    }
                }
            })
            .collect::<Html>();
        if ctx.props().is_multiple {
            html! {
                <div class={classes!(ctx.props().classes.clone(), "select")}>
                    <select
                        id={self.id.clone()} multiple={true} data-type="tags" data-placeholder={ctx.props().placeholder.clone()}
                        value={self.current_selector.clone()}
                        >
                        {items}
                    </select>
                </div>
            }
        } else {
            html! {
                   <input type="text"
                     class={classes!(ctx.props().classes.clone(), "input")}
                          id={self.id.clone()} data-type="tags" data-placeholder={ctx.props().placeholder.clone()} value={self.current_selector.clone()} />
            }
        }
    }

    fn rendered(&mut self, ctx: &Context<Self>, first_render: bool) {
        if first_render {
            let _max_items = ctx.props().max_items;
            let _is_multiple = ctx.props().is_multiple;
            let window = web_sys::window().expect("no global `window` exists");
            let document = window.document().expect("should have a document on window");

            let element = document
                .get_element_by_id(self.id.as_str())
                .expect(format!("should have #{} on the page", self.id.as_str()).as_str());

            // Clone the link from the context
            let link = ctx.link().clone();

            // Move the cloned link into the closure
            let callback = Closure::wrap(Box::new(move |tag: JsValue| {
                // gloo_console::log!("Value changed: {}", tag.clone());
                let command: js_sys::Object = JSON::parse(tag.as_string().unwrap().as_str()).unwrap().dyn_into().unwrap();
                let op = Reflect::get(&command, &JsValue::from_str("op")).unwrap();
                let value = Reflect::get(&command, &JsValue::from_str("value")).unwrap();
                if op.as_string().unwrap() == "add" {
                    link.send_message(Msg::Added(value.as_string().unwrap()));
                } else {
                    link.send_message(Msg::Removed(value.as_string().unwrap()));
                }
            }) as Box<dyn FnMut(JsValue)>);

            setup_autocomplete(&element, callback.as_ref(), &JsValue::from(_max_items), &JsValue::from(_is_multiple));
            callback.forget();
        }
    }

    fn destroy(&mut self, ctx: &Context<Self>) {
        detach_autocomplete(&JsValue::from(self.id.as_str()));
    }
}

#[wasm_bindgen(inline_js = r#"
let init = new Map();
export function setup_autocomplete(element, callback, max_tags, is_multiple) {
    // Attach Bulma autocomplete here
    // console.log('Setting up autocomplete ID:' + element.id);
     if (!init.has(element.id)) {
         let autocompleteInstance = BulmaTagsInput.attach( element, {
            maxTags: max_tags,
         });
           let autocomplete = autocompleteInstance[0];
           // console.log('Attached autocomplete:'+element.id + ' ' + autocomplete);
            autocomplete.on('after.add', function(tag) {
                // console.log('tag:'+tag);
                if (is_multiple) {
                   callback('{"op":"add","value":"'+tag.item.value+'"}');
                } else {
                   callback('{"op":"add","value":"'+tag.item+'"}');
                }
            });
            autocomplete.on('after.remove', function(tag) {
                // console.log('tag2:'+tag);
                if (is_multiple) {
                   callback('{"op":"remove","value":"'+tag.value+'"}');
                } else {
                   callback('{"op":"remove","value":"'+tag+'"}');
                }
            });

          init.set(element.id, autocomplete);

     }


    // Call callback when an item is selected
}

export function detach_autocomplete(id) {
   init.delete(id);
   // console.log('Detached autocomplete:'+id);
}
"#)]
extern "C" {
    fn setup_autocomplete(element: &Element, callback: &JsValue, max_tags: &JsValue, is_multiple: &JsValue);
    fn detach_autocomplete(id: &JsValue);
}
