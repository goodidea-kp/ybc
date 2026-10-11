//! WCAG 2.2 at the library level (Bastion BASTION-104): a password input
//! carries its autocomplete token and never blocks paste (3.3.8 Accessible
//! Authentication).

use ybc::{Input, InputProps, InputType};
use yew::LocalServerRenderer;
use yew::prelude::*;

async fn password(autocomplete: Option<&str>) -> String {
    let props = yew::props!(InputProps {
        name: "password",
        value: String::new(),
        update: Callback::noop(),
        r#type: InputType::Password,
        autocomplete: autocomplete.map(AttrValue::from),
    });
    LocalServerRenderer::<Input>::with_props(props).hydratable(false).render().await
}

#[tokio::test]
async fn a_password_input_carries_its_autocomplete_token() {
    let current = password(Some("current-password")).await;
    let new = password(Some("new-password")).await;

    assert!(current.contains(r#"type="password""#), "{current}");
    assert!(current.contains(r#"autocomplete="current-password""#), "{current}");
    assert!(new.contains(r#"autocomplete="new-password""#), "{new}");
}

#[tokio::test]
async fn without_a_token_the_attribute_is_left_to_the_browser_and_paste_is_never_blocked() {
    let html = password(None).await;

    assert!(!html.contains("autocomplete"), "{html}");
    assert!(!html.contains("onpaste"), "{html}");
}
