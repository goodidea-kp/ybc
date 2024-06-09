use wasm_bindgen::JsValue;
use wasm_bindgen::prelude::wasm_bindgen;
use web_sys::Element;
use yew::prelude::*;


#[function_component(AccordionItem)]
pub fn accordion_item(props: &AccordionItemProps) -> Html {
    let accordion_classes = if props.open {
        "accordion is-active"
    } else {
        "accordion"
    };
    html! {
                        <article class={classes!(accordion_classes)}>
                            <div class="accordion-header toggle" onclick={props.on_toggle.clone()}>
                                <p>{&props.title}</p>
                            </div>
                            <div class="accordion-body">
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
    pub id: String,
}

pub struct Accordions {
    props: AccordionsProps,
    id: String,
}

#[derive(Properties, Clone, PartialEq)]
pub struct AccordionItemProps {
    pub title: String,
    pub children: Children,
    #[prop_or_default]
    pub open: bool,
    #[prop_or_else(Callback::noop)]
    pub on_toggle: Callback<MouseEvent>,
    #[prop_or_default]
    pub id: String,
}

impl Component for Accordions {
    type Message = ();
    type Properties = AccordionsProps;

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            props: ctx.props().clone(),
            id: format!("accordion-{}", ctx.props().id),
        }
    }

    fn update(&mut self, ctx: &Context<Self>, _msg: Self::Message) -> bool {
        self.props = ctx.props().clone();
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <section id={self.id.clone()} class="accordions">
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
                .get_element_by_id(self.id.as_str())
                .expect(format!("should have #{} on the page", self.id.as_str()).as_str());

            setup_accordion(&element);
        }
    }


    fn destroy(&mut self, ctx: &Context<Self>) {
        detach_accordion(&JsValue::from(self.id.as_str()));
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
