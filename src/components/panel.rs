use yew::events::{KeyboardEvent, MouseEvent};
use yew::prelude::*;

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct PanelProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub classes: Classes,
    /// The HTML content of this panel's heading; it is automatically wrapped in a `p.panel-heading`.
    #[prop_or_default]
    pub heading: Html,
    /// Optional accessible label for this panel navigation.
    #[prop_or_default]
    pub aria_label: AttrValue,
}

/// A composable panel, for compact controls.
///
/// [https://bulma.io/documentation/components/panel/](https://bulma.io/documentation/components/panel/)
#[component(Panel)]
pub fn panel(props: &PanelProps) -> Html {
    let class = classes!("panel", props.classes.clone());
    let aria_label = (!props.aria_label.is_empty()).then_some(props.aria_label.clone());
    html! {
        <nav {class} role="navigation" aria-label={aria_label}>
            <p class="panel-heading">{props.heading.clone()}</p>
            {props.children.clone()}
        </nav>
    }
}

//////////////////////////////////////////////////////////////////////////////
//////////////////////////////////////////////////////////////////////////////

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct PanelTabsProps {
    #[prop_or_default]
    pub children: Children,
    /// Optional accessible label for the tabs collection.
    #[prop_or_default]
    pub aria_label: AttrValue,
}

/// A container for the navigation tabs of a panel.
///
/// [https://bulma.io/documentation/components/panel/](https://bulma.io/documentation/components/panel/)
#[component(PanelTabs)]
pub fn panel_tabs(props: &PanelTabsProps) -> Html {
    let aria_label = (!props.aria_label.is_empty()).then_some(props.aria_label.clone());
    // Bulma `.panel-tabs` is filter navigation, not a WAI-ARIA tablist widget
    // (no roving tabindex / arrow-key contract, no linked tabpanels). Claiming
    // role="tablist" requires role="tab" children and trips axe
    // `aria-required-children`; expose it as a labelled navigation region instead.
    // The accessible tablist widget lives in `tabs.rs` (`Tabs`/`TabItem`/`TabPanel`).
    html! { <p class="panel-tabs" role="navigation" aria-label={aria_label}>{props.children.clone()}</p> }
}

//////////////////////////////////////////////////////////////////////////////
//////////////////////////////////////////////////////////////////////////////

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct PanelBlockProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub classes: Classes,
    /// The HTML tag to use for this component.
    #[prop_or_else(|| "div".into())]
    pub tag: String,
    /// Make this element the active / highlighted element.
    #[prop_or_default]
    pub active: bool,
    /// The click handler for this element.
    #[prop_or_default]
    pub onclick: Callback<MouseEvent>,
    /// Callback used for keyboard activation on non-native interactive tags.
    #[prop_or_default]
    pub onactivate: Callback<()>,
    /// Mark this element as keyboard interactive when using non-interactive tags.
    #[prop_or_default]
    pub interactive: bool,
    /// Optional aria label.
    #[prop_or_default]
    pub aria_label: AttrValue,
    /// Optional aria-controls target id.
    #[prop_or_default]
    pub aria_controls: AttrValue,
    /// Optional aria-expanded state.
    #[prop_or_default]
    pub aria_expanded: Option<bool>,
    /// Optional aria-pressed state.
    #[prop_or_default]
    pub aria_pressed: Option<bool>,
}

/// An individual element of the panel.
///
/// [https://bulma.io/documentation/components/panel/](https://bulma.io/documentation/components/panel/)
#[component(PanelBlock)]
pub fn panel_block(props: &PanelBlockProps) -> Html {
    let class = classes!("panel-block", props.classes.clone(), props.active.then_some("is-active"));
    let aria_label = (!props.aria_label.is_empty()).then_some(props.aria_label.clone());
    let aria_controls = (!props.aria_controls.is_empty()).then_some(props.aria_controls.clone());
    let aria_expanded = props.aria_expanded.map(|value| if value { "true" } else { "false" });
    let aria_pressed = props.aria_pressed.map(|value| if value { "true" } else { "false" });
    let keyboard_interactive = props.interactive && props.tag != "button" && props.tag != "a";
    let role = keyboard_interactive.then_some("button");
    let tabindex = keyboard_interactive.then_some("0");
    let onkeydown = {
        let onactivate = props.onactivate.clone();
        Callback::from(move |event: KeyboardEvent| {
            let key = event.key();
            if key == "Enter" || key == " " {
                event.prevent_default();
                onactivate.emit(());
            }
        })
    };
    html! {
        <@{props.tag.clone()}
            {class}
            role={role}
            tabindex={tabindex}
            aria-label={aria_label}
            aria-controls={aria_controls}
            aria-expanded={aria_expanded}
            aria-pressed={aria_pressed}
            onclick={props.onclick.clone()}
            onkeydown={onkeydown}
        >
            {props.children.clone()}
        </@>
    }
}
