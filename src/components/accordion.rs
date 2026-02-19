//! Accordion component: a Yew wrapper around the bulma-accordion plugin.
//!
//! Required static assets
//! - Add the bulma-accordion CSS into your HTML <head>:
//!   <link rel="stylesheet" href="https://cdn.jsdelivr.net/npm/bulma-accordion@2.0.1/dist/css/bulma-accordion.min.css"/>
//!
//! - Add the bulma-accordion JS so `bulmaAccordion` is available on window. Place this before your wasm bootstrap script
//!   (or ensure it loads before your Yew app mounts):
//!   <script src="https://cdn.jsdelivr.net/npm/bulma-accordion@2.0.1/dist/js/bulma-accordion.min.js"></script>
//!
//! How to configure index.html
//! - Minimal example (place CSS in <head>, script before the wasm init script):
//!   ```html
//!   <!doctype html>
//!   <html>
//!   <head>
//!     <meta charset="utf-8" />
//!     <meta name="viewport" content="width=device-width,initial-scale=1" />
//!     <!-- bulma-accordion CSS -->
//!     <link rel="stylesheet" href="https://cdn.jsdelivr.net/npm/bulma-accordion@2.0.1/dist/css/bulma-accordion.min.css"/>
//!   </head>
//!   <body>
//!     <div id="root"></div>
//!
//!     <!-- bulma-accordion JS: ensure this runs before your wasm bootstrap so `bulmaAccordion` exists -->
//!     <script src="https://cdn.jsdelivr.net/npm/bulma-accordion@2.0.1/dist/js/bulma-accordion.min.js"></script>
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
//! - If you use a bundler (webpack, vite, etc.) you can install bulma-accordion from npm and import it in your JS entry:
//!   npm install bulma-accordion
//!   // in your entry file
//!   import 'bulma-accordion/dist/css/bulma-accordion.min.css';
//!   import 'bulma-accordion/dist/js/bulma-accordion.min.js';
//!   Ensure the import runs before the Yew bootstrap so `bulmaAccordion` is available globally (or adapt the setup to pass the module).
//!
//! - The important requirement: bulmaAccordion must be defined on window when setup_accordion is called in rendered().

use std::rc::Rc;
use std::sync::atomic::{AtomicUsize, Ordering};
use wasm_bindgen::JsValue;
use wasm_bindgen::prelude::wasm_bindgen;
use web_sys::Element;
use yew::events::{KeyboardEvent, MouseEvent};
use yew::prelude::*;

static ACCORDION_ITEM_AUTO_ID: AtomicUsize = AtomicUsize::new(1);

fn next_accordion_item_id() -> String {
    format!("accordion-item-{}", ACCORDION_ITEM_AUTO_ID.fetch_add(1, Ordering::Relaxed))
}

#[component(AccordionItem)]
pub fn accordion_item(props: &AccordionItemProps) -> Html {
    let internal_open = use_state(|| props.open);
    let is_controlled = props.controlled_open.is_some() && props.set_open.is_some();
    let is_open = props.controlled_open.unwrap_or(*internal_open);

    let set_local_open = {
        let internal_open = internal_open.clone();
        let set_open = props.set_open.clone();
        Callback::from(move |value: bool| {
            if is_controlled {
                if let Some(set_open) = set_open.as_ref() {
                    set_open.emit(value);
                }
            } else {
                internal_open.set(value);
            }
        })
    };

    {
        let on_open = props.on_open.clone();
        let on_close = props.on_close.clone();
        let prev_open = use_mut_ref(move || is_open);
        use_effect_with(is_open, move |is_open| {
            let mut prev = prev_open.borrow_mut();
            if *prev != *is_open {
                if *is_open {
                    on_open.emit(());
                } else {
                    on_close.emit(());
                }
                *prev = *is_open;
            }
            || {}
        });
    }

    let auto_id = use_state(|| Rc::<str>::from(next_accordion_item_id()));
    let item_id = if props.id.is_empty() { (*auto_id).clone() } else { props.id.clone() };
    let header_id = AttrValue::from(format!("{}-header", item_id));
    let panel_id = AttrValue::from(format!("{}-panel", item_id));
    let accordion_classes = if is_open { "accordion is-active" } else { "accordion" };

    let on_click = {
        let set_local_open = set_local_open.clone();
        let on_toggle = props.on_toggle.clone();
        let is_open = is_open;
        Callback::from(move |event: MouseEvent| {
            set_local_open.emit(!is_open);
            on_toggle.emit(event);
        })
    };

    let on_keydown = {
        let set_local_open = set_local_open.clone();
        let is_open = is_open;
        Callback::from(move |event: KeyboardEvent| {
            let key = event.key();
            if key == "Enter" || key == " " {
                event.prevent_default();
                set_local_open.emit(!is_open);
            }
        })
    };

    html! {
        <article class={classes!(accordion_classes)}>
            <div
                id={header_id.clone()}
                class="accordion-header toggle"
                role="button"
                tabindex="0"
                aria-expanded={if is_open { "true" } else { "false" }}
                aria-controls={panel_id.clone()}
                onclick={on_click}
                onkeydown={on_keydown}
            >
                <p>{props.title.to_string()}</p>
            </div>
            <div id={panel_id} class="accordion-body" role="region" aria-labelledby={header_id} aria-hidden={if is_open { "false" } else { "true" }}>
                <div class="accordion-content">
                    {props.children.clone()}
                </div>
            </div>
        </article>
    }
}

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct AccordionsProps {
    pub children: ChildrenWithProps<AccordionItem>,
    pub id: Rc<str>,
}

pub struct Accordions {
    props: AccordionsProps,
}

#[derive(Properties, Clone, PartialEq)]
pub struct AccordionItemProps {
    pub title: Rc<str>,
    pub children: Children,
    /// Initial open state for uncontrolled mode.
    #[prop_or_default]
    pub open: bool,
    /// Controlled open state.
    #[prop_or_default]
    pub controlled_open: Option<bool>,
    /// Controlled open setter.
    #[prop_or_default]
    pub set_open: Option<Callback<bool>>,
    /// Called when the item opens.
    #[prop_or_default]
    pub on_open: Callback<()>,
    /// Called when the item closes.
    #[prop_or_default]
    pub on_close: Callback<()>,
    #[prop_or_else(Callback::noop)]
    pub on_toggle: Callback<MouseEvent>,
    #[prop_or("".into())]
    pub id: Rc<str>,
}

impl Component for Accordions {
    type Message = ();
    type Properties = AccordionsProps;

    fn create(ctx: &Context<Self>) -> Self {
        Self { props: ctx.props().clone() }
    }

    fn update(&mut self, ctx: &Context<Self>, _msg: Self::Message) -> bool {
        self.props = ctx.props().clone();
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <section id={ctx.props().id.to_string()} class="accordions">
                {for ctx.props().children.iter().map(|child| {
                    html! {child.clone()}
                })}
            </section>
        }
    }

    fn rendered(&mut self, ctx: &Context<Self>, first_render: bool) {
        if first_render {
            let window = web_sys::window().expect("no global `window` exists");
            let document = window.document().expect("should have a document on window");

            let element = document
                .get_element_by_id(ctx.props().id.to_string().as_str())
                .unwrap_or_else(|| panic!("should have #{} on the page", ctx.props().id));

            setup_accordion(&element);
        }
    }

    fn destroy(&mut self, ctx: &Context<Self>) {
        detach_accordion(&JsValue::from_str(&ctx.props().id));
    }
}

#[wasm_bindgen(inline_js = r#"
let accordionInstances  = null;
export function setup_accordion(element) {
    // console.log('Setting up accordion ID:' + element.id);
    if (accordionInstances === null) {
      accordionInstances = bulmaAccordion.attach('#' + element.id);
      return;
    }

    // Check if the accordion is already attached
    for (let i = 0; i < accordionInstances.length; i++) {
        if (accordionInstances[i].element && accordionInstances[i].element.id === element.id) {
            // console.log('Accordion already attached to #id=' + element.id);
            return;
        }
    }

    // If not attached, attach it
    let newAccordion = bulmaAccordion.attach('#' + element.id);
    accordionInstances.push(newAccordion);
    // console.log('Accordion successfully attached to #id=' + element.id);

}

export function detach_accordion(id) {
    for (let i = 0; i < accordionInstances.length; i++) {
        if (accordionInstances[i] && accordionInstances[i].element && accordionInstances[i].element.id === id) {
            // console.log('Detaching accordion #id='+id+'!');
            accordionInstances[i].destroy();
            accordionInstances.splice(i, 1);
            // console.log(accordionInstances); // Log the accordionInstances array
            break;
        }
    }

    if (accordionInstances.length === 0) {
        accordionInstances = null;
         // console.log('Detached accordion from all!');
    }
}


"#)]
extern "C" {
    fn setup_accordion(element: &Element);
    fn detach_accordion(id: &JsValue);
}
