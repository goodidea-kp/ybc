use yew::events::MouseEvent;
use yew::prelude::*;

use crate::{Alignment, Size};

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct IconProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub classes: Classes,
    /// The click handler to use for this component.
    #[prop_or_default]
    pub onclick: Callback<MouseEvent>,
    /// The size of this component; to help prevent page "jumps" during load.
    #[prop_or_default]
    pub size: Option<Size>,
    /// The alignment of this icon, often used within form controls.
    #[prop_or_default]
    pub alignment: Option<Alignment>,
}

/// A container for any type of icon font.
///
/// [https://bulma.io/documentation/elements/icon/](https://bulma.io/documentation/elements/icon/)
#[component(Icon)]
pub fn icon(props: &IconProps) -> Html {
    let class = classes!(
        "icon",
        props.classes.clone(),
        props.size.as_ref().map(|size| size.to_string()),
        props.alignment.as_ref().map(|alignment| alignment.to_string()),
    );
    html! {
        <span {class} onclick={props.onclick.clone()}>
            {props.children.clone()}
        </span>
    }
}

//////////////////////////////////////////////////////////////////////////////
//////////////////////////////////////////////////////////////////////////////

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct FaIconProps {
    /// Font Awesome classes applied to the inner `<i>` element.
    ///
    /// Example: `classes!("fa-solid", "fa-user")`.
    pub icon_classes: Classes,
    /// Optional extra classes for the wrapper `.icon`.
    #[prop_or_default]
    pub classes: Classes,
    /// The click handler to use for this component.
    #[prop_or_default]
    pub onclick: Callback<MouseEvent>,
    /// The size of this component; to help prevent page "jumps" during load.
    #[prop_or_default]
    pub size: Option<Size>,
    /// The alignment of this icon, often used within form controls.
    #[prop_or_default]
    pub alignment: Option<Alignment>,
    /// Optional accessible label for non-decorative icons.
    #[prop_or_default]
    pub aria_label: AttrValue,
    /// Use fixed-width icon alignment (`fa-fw`) for better visual rhythm.
    #[prop_or(true)]
    pub fixed_width: bool,
}

/// A Font Awesome icon wrapper with Bulma-compatible spacing/alignment defaults.
///
/// Best practices encoded by default:
/// - decorative icons are hidden from assistive tech (`aria-hidden="true"`),
/// - labeled icons are exposed with `role="img"` and `aria-label`,
/// - `fa-fw` is enabled for consistent alignment.
#[component(FaIcon)]
pub fn fa_icon(props: &FaIconProps) -> Html {
    let is_labeled = !props.aria_label.is_empty();
    let role = is_labeled.then_some("img");
    let aria_label = is_labeled.then_some(props.aria_label.clone());
    let aria_hidden = (!is_labeled).then_some("true");

    let icon_classes = classes!(props.icon_classes.clone(), props.fixed_width.then_some("fa-fw"));
    html! {
        <Icon
            classes={props.classes.clone()}
            onclick={props.onclick.clone()}
            size={props.size.clone()}
            alignment={props.alignment.clone()}
        >
            <i class={icon_classes} aria-hidden={aria_hidden} role={role} aria-label={aria_label}></i>
        </Icon>
    }
}
