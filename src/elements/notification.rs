use yew::prelude::*;

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct NotificationProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub classes: Classes,
    /// `status` or `alert`, so assistive technology announces it; see also [`crate::Callout`].
    #[prop_or_default]
    pub role: Option<AttrValue>,
}

/// Bold notification blocks, to alert your users of something.
///
/// [https://bulma.io/documentation/elements/notification/](https://bulma.io/documentation/elements/notification/)
#[component(Notification)]
pub fn notification(props: &NotificationProps) -> Html {
    let class = classes!("notification", props.classes.clone());
    html! {
        <div {class} role={props.role.clone()}>
            {props.children.clone()}
        </div>
    }
}
