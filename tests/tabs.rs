//! Tabs from the keyboard (Bastion BASTION-230, WCAG 2.1.1): only the selected
//! tab is in the Tab order, so the arrow keys must reach the others.

use ybc::{TabItem, Tabs, TabsProps, sibling_tab_index};
use yew::LocalServerRenderer;
use yew::prelude::*;

#[test]
fn the_arrow_keys_step_through_the_tabs_and_wrap_round() {
    assert_eq!(sibling_tab_index("ArrowRight", 0, 3), Some(1));
    assert_eq!(sibling_tab_index("ArrowRight", 2, 3), Some(0), "past the last comes the first");
    assert_eq!(sibling_tab_index("ArrowLeft", 1, 3), Some(0));
    assert_eq!(sibling_tab_index("ArrowLeft", 0, 3), Some(2), "before the first comes the last");
}

#[test]
fn home_and_end_go_to_the_first_and_the_last_tab() {
    assert_eq!(sibling_tab_index("Home", 2, 4), Some(0));
    assert_eq!(sibling_tab_index("End", 0, 4), Some(3));
}

#[test]
fn other_keys_and_an_empty_list_move_nothing() {
    assert_eq!(sibling_tab_index("Tab", 0, 3), None);
    assert_eq!(sibling_tab_index("a", 0, 3), None);
    assert_eq!(sibling_tab_index("ArrowRight", 0, 0), None);
    assert_eq!(sibling_tab_index("ArrowRight", 0, 1), Some(0), "a lone tab stays put");
}

#[function_component(ThreeTabs)]
fn three_tabs() -> Html {
    let props = TabsProps {
        children: Children::new(vec![
            html! { <TabItem index={0usize} id={AttrValue::from("first")}>{"First"}</TabItem> },
            html! { <TabItem index={1usize} id={AttrValue::from("second")}>{"Second"}</TabItem> },
            html! { <TabItem index={2usize} id={AttrValue::from("third")} disabled=true>{"Third"}</TabItem> },
        ]),
        classes: Classes::new(),
        alignment: None,
        size: None,
        boxed: false,
        toggle: false,
        rounded: false,
        fullwidth: false,
        id: None,
        aria_label: AttrValue::from("Sections"),
        active: None,
        set_active: None,
        default_active: 0,
        on_change: Callback::noop(),
    };
    html! { <Tabs ..props/> }
}

#[tokio::test]
async fn only_the_selected_tab_is_in_the_tab_order_and_a_disabled_one_says_so() {
    let html = LocalServerRenderer::<ThreeTabs>::new().hydratable(false).render().await;
    assert!(html.contains(r#"role="tablist""#), "{html}");
    assert!(html.contains(r#"id="first" role="tab" aria-selected="true" tabindex="0""#), "{html}");
    assert!(
        html.contains(r#"tabindex="0""#) && html.matches(r#"tabindex="-1""#).count() == 2,
        "{html}"
    );
    assert!(html.contains(r#"aria-disabled="true""#), "{html}");
}
