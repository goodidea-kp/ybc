use std::sync::atomic::{AtomicUsize, Ordering};

use yew::events::{KeyboardEvent, MouseEvent};
use yew::prelude::*;

use crate::elements::button::Button;

static DROPDOWN_AUTO_ID: AtomicUsize = AtomicUsize::new(1);

fn next_dropdown_id() -> String {
    format!("dropdown-{}", DROPDOWN_AUTO_ID.fetch_add(1, Ordering::Relaxed))
}

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct DropdownProps {
    /// The content of the dropdown menu.
    ///
    /// This content will be placed directly within the `div.dropdown-content` container.
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub classes: Classes,
    /// Make this dropdown triggerable based on hover.
    #[prop_or_default]
    pub hoverable: bool,
    /// Any additional classes to use for the trigger button.
    #[prop_or_default]
    pub button_classes: Classes,
    /// The content of the trigger button.
    #[prop_or_default]
    pub button_html: Html,
    /// Optional id used as the root element id.
    #[prop_or_default]
    pub id: Option<AttrValue>,
    /// Controlled open state.
    #[prop_or_default]
    pub open: Option<bool>,
    /// Controlled open state setter.
    #[prop_or_default]
    pub set_open: Option<Callback<bool>>,
    /// Called after the menu opens.
    #[prop_or_default]
    pub on_open: Callback<()>,
    /// Called after the menu closes.
    #[prop_or_default]
    pub on_close: Callback<()>,
    /// Allows closing the menu with Escape.
    #[prop_or(true)]
    pub close_on_escape: bool,
    /// Allows closing the menu by clicking outside.
    #[prop_or(true)]
    pub close_on_click_outside: bool,
    /// Optional trigger label for assistive technologies.
    #[prop_or_default]
    pub button_aria_label: AttrValue,
    /// Optional menu label for assistive technologies.
    #[prop_or_default]
    pub menu_aria_label: AttrValue,
}

/// Dropdown actions kept for backwards compatibility.
pub enum DropdownMsg {
    Open,
    Close,
}

/// An interactive dropdown menu for discoverable content.
///
/// [https://bulma.io/documentation/components/dropdown/](https://bulma.io/documentation/components/dropdown/)
#[component(Dropdown)]
pub fn dropdown(props: &DropdownProps) -> Html {
    let internal_open = use_state(|| false);
    let is_controlled = props.open.is_some() && props.set_open.is_some();
    let is_open = props.open.unwrap_or(*internal_open);

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

    let auto_id = use_state(|| AttrValue::from(next_dropdown_id()));
    let root_id = props.id.clone().unwrap_or_else(|| (*auto_id).clone());
    let menu_id = AttrValue::from(format!("{}-menu", root_id));
    let button_aria_label = (!props.button_aria_label.is_empty()).then_some(props.button_aria_label.clone());
    let menu_aria_label = (!props.menu_aria_label.is_empty()).then_some(props.menu_aria_label.clone());

    let on_trigger_click = {
        let set_local_open = set_local_open.clone();
        let hoverable = props.hoverable;
        let is_open = is_open;
        Callback::from(move |event: MouseEvent| {
            if hoverable {
                return;
            }
            event.prevent_default();
            set_local_open.emit(!is_open);
        })
    };

    let on_trigger_keydown = {
        let set_local_open = set_local_open.clone();
        let hoverable = props.hoverable;
        let close_on_escape = props.close_on_escape;
        Callback::from(move |event: KeyboardEvent| {
            if hoverable {
                return;
            }
            match event.key().as_str() {
                "Enter" | " " | "ArrowDown" => {
                    event.prevent_default();
                    set_local_open.emit(true);
                }
                "Escape" if close_on_escape => {
                    event.prevent_default();
                    set_local_open.emit(false);
                }
                _ => {}
            }
        })
    };

    let on_root_keydown = {
        let set_local_open = set_local_open.clone();
        let close_on_escape = props.close_on_escape;
        Callback::from(move |event: KeyboardEvent| {
            if close_on_escape && event.key() == "Escape" {
                event.prevent_default();
                set_local_open.emit(false);
            }
        })
    };

    let on_overlay_click = {
        let set_local_open = set_local_open.clone();
        Callback::from(move |_| set_local_open.emit(false))
    };

    let mut class = classes!("dropdown", props.classes.clone());
    if props.hoverable {
        class.push("is-hoverable");
    } else if is_open {
        class.push("is-active");
    }

    let overlay = if !props.hoverable && is_open && props.close_on_click_outside {
        html! {
            <div
                onclick={on_overlay_click}
                style="z-index:10;background-color:rgba(0,0,0,0);position:fixed;top:0;bottom:0;left:0;right:0;"
            ></div>
        }
    } else {
        Html::default()
    };

    html! {
        <div id={root_id} {class} onkeydown={on_root_keydown}>
            {overlay}
            <div class="dropdown-trigger">
                <Button
                    classes={props.button_classes.clone()}
                    onclick={on_trigger_click}
                    onkeydown={on_trigger_keydown}
                    aria_label={button_aria_label.unwrap_or_default()}
                    aria_controls={menu_id.clone()}
                    aria_haspopup={"menu"}
                    aria_expanded={Some(is_open)}
                >
                    {props.button_html.clone()}
                </Button>
            </div>
            <div id={menu_id} class="dropdown-menu" role="menu" aria-label={menu_aria_label}>
                <div class="dropdown-content">
                    {props.children.clone()}
                </div>
            </div>
        </div>
    }
}
