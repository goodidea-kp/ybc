use yew::prelude::*;

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct DemoTitleProps {
    pub title: AttrValue,
    pub icon_classes: Classes,
}

#[component(DemoTitle)]
pub fn demo_title(props: &DemoTitleProps) -> Html {
    html! {
        <h3 class="is-flex is-align-items-center mb-3">
            <ybc::FaIcon
                classes={classes!("mr-2", "has-text-link")}
                icon_classes={props.icon_classes.clone()}
            />
            <span>{props.title.clone()}</span>
        </h3>
    }
}
