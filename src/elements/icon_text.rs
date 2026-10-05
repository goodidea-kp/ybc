use yew::prelude::*;

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct IconTextProps {
    /// The icon's classes, e.g. `"fas fa-plus"`.
    pub icon: AttrValue,
    #[prop_or_default]
    pub classes: Classes,
    #[prop_or_default]
    pub children: Children,
}

/// An icon beside its label (Bulma `.icon-text`). The icon is always
/// `aria-hidden`: the label already says it, and a screen reader that reads
/// both says it twice.
#[component(IconText)]
pub fn icon_text(props: &IconTextProps) -> Html {
    html! {
        <span class={classes!("icon-text", props.classes.clone())}>
            <span class="icon"><i class={props.icon.clone()} aria-hidden="true"></i></span>
            <span>{ props.children.clone() }</span>
        </span>
    }
}
