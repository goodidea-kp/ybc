use std::rc::Rc;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::JsCast;
use wasm_bindgen::JsValue;
use web_sys::js_sys::{Reflect, JSON};
use web_sys::{js_sys, Element};
use yew::prelude::*;

pub struct AutoComplete {
    id: Rc<str>,
}

#[derive(Clone, PartialEq, Properties)]
pub struct AutoCompleteProps {
    #[prop_or("".to_string().into())]
    pub id: Rc<str>,
    #[prop_or(10)]
    pub max_items: u32,
    #[prop_or_default]
    pub items: Vec<String>,
    pub on_update: Callback<String>,
    pub on_remove: Callback<String>,
    #[prop_or("".to_string().into())]
    pub current_selector: Rc<str>,
    #[prop_or("Choose Tags".to_string().into())]
    pub placeholder: Rc<str>,
    #[prop_or(classes ! ("".to_string()))]
    pub classes: Classes,
    #[prop_or(true)]
    pub case_sensitive: bool,
    #[prop_or("".to_string().into())]
    pub data_item_text: Rc<str>,
    #[prop_or("".to_string().into())]
    pub data_item_value: Rc<str>,
    #[prop_or("".to_string().into())]
    pub url_for_fetch: Rc<str>,
    #[prop_or("".to_string().into())]
    pub auth_header: Rc<str>,
}

pub enum Msg {
    Added(String),
    Removed(String),
}

impl Component for AutoComplete {
    type Message = Msg;
    type Properties = AutoCompleteProps;
    fn create(ctx: &Context<Self>) -> Self {
        Self { id: ctx.props().id.clone() }
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
        let current_selector = ctx.props().current_selector.to_string();
        let items = ctx
            .props()
            .items
            .iter()
            .map(|item| {
                if item == current_selector.as_str() {
                    html! {
                         <option value={item.to_string()} selected=true>{&item}</option>
                    }
                } else {
                    html! {
                         <option value={item.to_string()}>{&item}</option>
                    }
                }
            })
            .collect::<Html>();
        if ctx.props().items.len() > 0 && ctx.props().data_item_text.len() == 0 && ctx.props().data_item_value.len() == 0 {
            html! {
                <div class={classes!(ctx.props().classes.clone(), "select")}>
                    <select
                        id={self.id.clone()} data-type="tags"
                        data-placeholder={ctx.props().placeholder.clone()}
                        value={current_selector.clone()}
                        >
                        {items}
                    </select>
                </div>
            }
        } else if ctx.props().data_item_text.len() > 0 && ctx.props().data_item_value.len() > 0 {
            let has_value = current_selector.len() > 0;
            let value = format!("{{\"{}\":\"{}\"}}", ctx.props().data_item_value, current_selector);
            html! {
                   <input type="text"
                     class={classes!(ctx.props().classes.clone(), "input")}
                             // data-type="name"
                          data-item-text={ctx.props().data_item_text.clone()}
                          data-item-value={ctx.props().data_item_value.clone()}
                          id={self.id.to_string()} data-placeholder={ctx.props().placeholder.clone()}
                          value={if has_value {value} else {"{}".to_string()}} />
            }
        } else {
            html! {
                   <input type="text"
                     class={classes!(ctx.props().classes.clone(), "input")}
                          id={self.id.clone()} data-placeholder={ctx.props().placeholder.clone()} value={current_selector.clone()} />
            }
        }
    }

    fn rendered(&mut self, ctx: &Context<Self>, first_render: bool) {
        if first_render {
            let _max_items = ctx.props().max_items;
            let _case_sensitive = ctx.props().case_sensitive;
            let _url_for_fetch = ctx.props().url_for_fetch.clone();
            let _auth_header = ctx.props().auth_header.clone();
            let window = web_sys::window().expect("no global `window` exists");
            let document = window.document().expect("should have a document on window");

            let element = document
                .get_element_by_id(&*self.id)
                .expect(format!("should have #{} on the page", self.id).as_str());

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
            if _url_for_fetch.len() == 0 {
                setup_static_autocomplete(&element, callback.as_ref(), &JsValue::from(_max_items), &JsValue::from(_case_sensitive));
            } else {
                setup_dynamic_autocomplete(
                    &element,
                    callback.as_ref(),
                    &JsValue::from(_max_items),
                    &JsValue::from(_url_for_fetch.to_string()),
                    &JsValue::from(_auth_header.to_string()),
                    &JsValue::from(_case_sensitive),
                    &JsValue::from(ctx.props().data_item_value.to_string()),
                );
            }
            callback.forget();
        }
    }

    fn destroy(&mut self, ctx: &Context<Self>) {
        detach_autocomplete(&JsValue::from(self.id.as_ptr()));
    }
}

#[wasm_bindgen(inline_js = r#"
let init = new Map();
export function setup_dynamic_autocomplete(element, callback, max_tags, url_for_fetch, auth_header, case_sensitive, data_item_value) {
    // Attach Bulma autocomplete here
    // console.log('Setting up dynamic autocomplete ID:' + element.id + ' fetch:' + url_for_fetch + ' auth:' + auth_header + ' case:' + case_sensitive + ' max:' + max_tags);
     if (!init.has(element.id)) {
		 // console.log('Setting up dynamic autocomplete ID:' + element.id);
         let autocompleteInstance = BulmaTagsInput.attach( element, {
            maxTags: max_tags,
            caseSensitive: case_sensitive,
            source: async function(value) {
                // console.log('Fetching data for:'+value);
				return await fetch(url_for_fetch + value)
					.then(function(response) {
					    if (response.status !== 200) {
                            throw new Error('Failed to fetch data');
                        }
						return response.json();
					});},
         });
           let autocomplete = autocompleteInstance[0];
           // console.log('Attached autocomplete:'+element.id + ' ' + autocomplete);
            autocomplete.on('after.add', function(tag) {
                // console.log(tag);
                callback('{"op":"add","value":"'+tag.item[data_item_value]+'"}');
            });
            autocomplete.on('after.remove', function(tag) {
                // console.log(tag);
                callback('{"op":"remove","value":"'+tag[data_item_value]+'"}');
            });

          init.set(element.id, autocomplete);
     }
}

export function setup_static_autocomplete(element, callback, max_tags, case_sensitive) {
    // Attach Bulma autocomplete here
    // console.log('Setting up static autocomplete ID:' + element.id + ' case:' + case_sensitive + ' max:' + max_tags);
     if (!init.has(element.id)) {
         let autocompleteInstance = BulmaTagsInput.attach( element, {
            maxTags: max_tags,
            caseSensitive: case_sensitive,
         });
           let autocomplete = autocompleteInstance[0];
           // console.log('Attached autocomplete:'+element.id + ' ' + autocomplete);
            autocomplete.on('after.add', function(tag) {
                // console.log(tag);
                if (tag.item && tag.item.value) {
                    callback('{"op":"add","value":"'+tag.item.value+'"}');
                } else if (tag.value) {
                    callback('{"op":"add","value":"'+tag.value+'"}');
                } else {
                    callback('{"op":"add","value":"'+tag.item+'"}');
                }
            });
            autocomplete.on('after.remove', function(tag) {
                // console.log(tag);
                if (tag.item && tag.item.value) {
                    callback('{"op":"remove","value":"'+tag.item.value+'"}');
                } else if (tag.value) {
                    callback('{"op":"remove","value":"'+tag.value+'"}');
                } else {
                    callback('{"op":"remove","value":"'+tag+'"}');
                }
            });

          init.set(element.id, autocomplete);

     }
}

export function detach_autocomplete(id) {
   init.delete(id);
   // console.log('Detached autocomplete:'+id);
}

"#)]
extern "C" {
    fn setup_dynamic_autocomplete(
        element: &Element, callback: &JsValue, max_tags: &JsValue, url_to_fetch: &JsValue, auth_header: &JsValue, case_sensitive: &JsValue,
        data_item_value: &JsValue,
    );
    fn setup_static_autocomplete(element: &Element, callback: &JsValue, max_tags: &JsValue, case_sensitive: &JsValue);
    fn detach_autocomplete(id: &JsValue);
}
