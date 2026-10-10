//! `Table` rendered to HTML on the host: the attributes a test or a screen
//! reader looks for sit on the `<table>` itself, scrollable or not
//! (Bastion BASTION-145).

use ybc::{Table, TableProps};
use yew::LocalServerRenderer;
use yew::prelude::*;

async fn render(props: TableProps) -> String {
    LocalServerRenderer::<Table>::with_props(props).hydratable(false).render().await
}

fn body() -> Children {
    Children::new(vec![html! {
        <>
            <thead><tr><th scope="col">{"Name"}</th></tr></thead>
            <tbody><tr data-testid="row"><td>{"alpha"}</td></tr></tbody>
        </>
    }])
}

fn labelled(scrollable: bool) -> TableProps {
    TableProps {
        children: body(),
        classes: classes!("is-size-7"),
        bordered: false,
        striped: true,
        narrow: true,
        hoverable: true,
        fullwidth: true,
        scrollable,
        id: Some("keys".into()),
        testid: Some("keys-table".into()),
        aria_label: Some("Keys".into()),
        aria_labelledby: Some("keys-heading".into()),
        aria_describedby: Some("keys-help".into()),
    }
}

fn plain() -> TableProps {
    TableProps {
        children: body(),
        classes: Classes::new(),
        bordered: false,
        striped: false,
        narrow: false,
        hoverable: false,
        fullwidth: false,
        scrollable: false,
        id: None,
        testid: None,
        aria_label: None,
        aria_labelledby: None,
        aria_describedby: None,
    }
}

/// The opening `<table ...>` tag, so an assertion cannot pass on the wrapper.
fn table_tag(html: &str) -> &str {
    let start = html.find("<table").expect("a <table> element");
    let end = start + html[start..].find('>').expect("a closed <table> tag");
    &html[start..=end]
}

const LABEL_ATTRIBUTES: [&str; 5] = [
    r#"id="keys""#,
    r#"data-testid="keys-table""#,
    r#"aria-label="Keys""#,
    r#"aria-labelledby="keys-heading""#,
    r#"aria-describedby="keys-help""#,
];

#[tokio::test]
async fn attributes_sit_on_the_table_when_it_does_not_scroll() {
    let html = render(labelled(false)).await;
    let tag = table_tag(&html);

    for attribute in LABEL_ATTRIBUTES {
        assert!(tag.contains(attribute), "{attribute} missing from {tag}");
    }
    assert!(!html.contains("table-container"), "{html}");
}

#[tokio::test]
async fn attributes_sit_on_the_table_not_the_wrapper_when_it_scrolls() {
    let html = render(labelled(true)).await;
    let tag = table_tag(&html);
    let wrapper = &html[..html.find("<table").expect("a <table> element")];

    assert_eq!(wrapper, r#"<div class="table-container">"#, "{html}");
    for attribute in LABEL_ATTRIBUTES {
        assert!(tag.contains(attribute), "{attribute} missing from {tag}");
    }
    assert_eq!(html.matches("table-container").count(), 1, "one scroll container: {html}");
}

#[tokio::test]
async fn modifiers_and_custom_classes_go_on_the_table() {
    let html = render(labelled(true)).await;
    let tag = table_tag(&html);

    for class in ["table", "is-size-7", "is-striped", "is-narrow", "is-hoverable", "is-fullwidth"] {
        assert!(tag.contains(class), "{class} missing from {tag}");
    }
    assert!(!tag.contains("is-bordered"), "{tag}");
}

#[tokio::test]
async fn bordered_adds_its_modifier() {
    let html = render(TableProps { bordered: true, ..plain() }).await;
    assert!(table_tag(&html).contains("is-bordered"), "{html}");
}

#[tokio::test]
async fn a_plain_table_renders_no_empty_attributes() {
    let html = render(plain()).await;
    let tag = table_tag(&html);

    assert_eq!(tag, r#"<table class="table">"#, "{html}");
}

#[tokio::test]
async fn children_render_inside_the_table_unchanged() {
    let html = render(labelled(true)).await;

    assert!(html.contains(r#"<thead><tr><th scope="col">Name</th></tr></thead>"#), "{html}");
    assert!(html.contains(r#"<tbody><tr data-testid="row"><td>alpha</td></tr></tbody>"#), "{html}");
    assert!(html.ends_with("</table></div>"), "{html}");
}
