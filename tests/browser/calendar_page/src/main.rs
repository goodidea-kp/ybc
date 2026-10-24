//! Two date fields for the browser test: one on the page with a `<label for>`,
//! one inside a ModalCard. Each shows the value it last emitted, and the modal
//! says whether it is open, so the test sees what an Esc closed.

use ybc::{Calendar, ModalCard, TestAttr};
use yew::prelude::*;

#[function_component(PageField)]
fn page_field() -> Html {
    let value = use_state(String::new);
    let on_date_changed = {
        let value = value.clone();
        Callback::from(move |date: String| value.set(date))
    };
    html! {
        <section class="section">
            <label class="label" for="plain">{ "Start date" }</label>
            <Calendar id="plain" date={Some((*value).clone())} {on_date_changed} test_attr={TestAttr::test_id("plain")} />
            <output data-testid="plain-value">{ (*value).clone() }</output>
        </section>
    }
}

#[function_component(ModalField)]
fn modal_field() -> Html {
    let value = use_state(|| "10/24/2026".to_owned());
    let open = use_state(|| false);
    let on_date_changed = {
        let value = value.clone();
        Callback::from(move |date: String| value.set(date))
    };
    let set_open = {
        let open = open.clone();
        Callback::from(move |next: bool| open.set(next))
    };
    let body = html! {
        <>
            <Calendar id="in-modal" date_format="mm/dd/yyyy" date={Some((*value).clone())} {on_date_changed}
                      aria_label="Not after" test_attr={TestAttr::test_id("in-modal")} />
            <output data-testid="in-modal-value">{ (*value).clone() }</output>
        </>
    };
    let open_modal = {
        let open = open.clone();
        Callback::from(move |_| open.set(true))
    };
    html! {
        <section class="section">
            <button type="button" class="button" data-testid="open-modal" onclick={open_modal}>{ "Open the form" }</button>
            <p data-testid="modal-state">{ if *open { "open" } else { "closed" } }</p>
            <ModalCard id="form" title="A form" {body} open={Some(*open)} set_open={Some(set_open)} />
        </section>
    }
}

#[function_component(App)]
fn app() -> Html {
    html! { <><PageField /><ModalField /></> }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
