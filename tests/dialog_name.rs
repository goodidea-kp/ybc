//! A modal card is named by its title (Bastion BASTION-230, WCAG 4.1.2): a
//! screen reader announces "Add domain, dialog" on opening, not "dialog".

use ybc::{ModalCard, ModalCardProps};
use yew::LocalServerRenderer;
use yew::prelude::*;

#[function_component(AddDomain)]
fn add_domain() -> Html {
    let props = yew::props!(ModalCardProps {
        id: Some(AttrValue::from("add-domain")),
        title: AttrValue::from("Add domain"),
        body: html! { "Name" },
        footer: html! {},
        trigger: html! {},
    });
    html! { <ModalCard ..props/> }
}

#[tokio::test]
async fn the_dialog_is_labelled_by_its_title() {
    let html = LocalServerRenderer::<AddDomain>::new().hydratable(false).render().await;
    assert!(html.contains(r#"aria-labelledby="add-domain-title""#), "{html}");
    assert!(html.contains(r#"id="add-domain-title""#), "{html}");
    let title_at = html.find(r#"id="add-domain-title""#).unwrap_or_default();
    assert!(html[title_at..].contains("Add domain"), "{html}");
}
