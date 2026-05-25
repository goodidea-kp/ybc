use yew::prelude::*;

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct DeleteProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub classes: Classes,
    /// The HTML tag to use for this component.
    #[prop_or_else(|| "button".into())]
    pub tag: String,
    /// The click handler to use for this component.
    #[prop_or_default]
    pub onclick: Callback<MouseEvent>,
    /// Optional `aria-label` attribute.
    #[prop_or_default]
    pub aria_label: AttrValue,
}

/// A versatile delete cross.
///
/// [https://bulma.io/documentation/elements/delete/](https://bulma.io/documentation/elements/delete/)
#[component(Delete)]
pub fn delete(props: &DeleteProps) -> Html {
    let class = classes!("delete", props.classes.clone());
    let aria_label = (!props.aria_label.is_empty()).then_some(props.aria_label.clone());
    html! {
        <@{props.tag.clone()} {class} aria-label={aria_label} onclick={props.onclick.clone()}>
            {props.children.clone()}
        </@>
    }
}
