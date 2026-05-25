use derive_more::Display;
use yew::events::{Event, KeyboardEvent, MouseEvent};
use yew::prelude::*;

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct ButtonsProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub classes: Classes,
    /// The size for all buttons within this group.
    #[prop_or_default]
    pub size: Option<ButtonGroupSize>,
}

/// A container for a group of buttons.
///
/// [https://bulma.io/documentation/elements/button/](https://bulma.io/documentation/elements/button/)
#[component(Buttons)]
pub fn buttons(props: &ButtonsProps) -> Html {
    let class = classes!("buttons", props.classes.clone(), props.size.as_ref().map(ToString::to_string));
    html! {
        <div {class}>
            {props.children.clone()}
        </div>
    }
}

/// The 3 sizes available for a button group.
///
/// https://bulma.io/documentation/elements/button/#sizes
#[derive(Clone, Debug, Display, PartialEq, Eq)]
#[display("are-{}")]
pub enum ButtonGroupSize {
    #[display("small")]
    Small,
    #[display("medium")]
    Medium,
    #[display("large")]
    Large,
}

//////////////////////////////////////////////////////////////////////////////
//////////////////////////////////////////////////////////////////////////////

/// The available HTML `type` values for a `<button>`.
#[derive(Clone, Debug, Display, PartialEq, Eq)]
pub enum ButtonType {
    #[display("button")]
    Button,
    #[display("submit")]
    Submit,
    #[display("reset")]
    Reset,
}

//////////////////////////////////////////////////////////////////////////////
//////////////////////////////////////////////////////////////////////////////

/// Semantic colors for Bulma buttons.
#[derive(Clone, Debug, Display, PartialEq, Eq)]
pub enum ButtonColor {
    #[display("is-white")]
    White,
    #[display("is-light")]
    Light,
    #[display("is-dark")]
    Dark,
    #[display("is-black")]
    Black,
    #[display("is-text")]
    Text,
    #[display("is-ghost")]
    Ghost,
    #[display("is-primary")]
    Primary,
    #[display("is-link")]
    Link,
    #[display("is-info")]
    Info,
    #[display("is-success")]
    Success,
    #[display("is-warning")]
    Warning,
    #[display("is-danger")]
    Danger,
}

/// Available Bulma button sizes.
#[derive(Clone, Debug, Display, PartialEq, Eq)]
pub enum ButtonSize {
    #[display("is-small")]
    Small,
    #[display("is-medium")]
    Medium,
    #[display("is-large")]
    Large,
}

/// Optional button appearance variant.
#[derive(Clone, Debug, Display, PartialEq, Eq)]
pub enum ButtonVariant {
    #[display("is-outlined")]
    Outlined,
    #[display("is-inverted")]
    Inverted,
    #[display("is-light")]
    Light,
    #[display("is-text")]
    Text,
    #[display("is-ghost")]
    Ghost,
}

//////////////////////////////////////////////////////////////////////////////
//////////////////////////////////////////////////////////////////////////////

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct ButtonProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub classes: Classes,
    /// The click handler to use for this component.
    #[prop_or_default]
    pub onclick: Callback<MouseEvent>,
    /// The keyboard handler to use for this component.
    #[prop_or_default]
    pub onkeydown: Callback<KeyboardEvent>,
    /// Render a loading spinner within this component.
    #[prop_or_default]
    pub loading: bool,
    /// Make this component static.
    #[prop_or_default]
    pub r#static: bool,
    /// Disable this component.
    #[prop_or_default]
    pub disabled: bool,
    /// Optional semantic color.
    #[prop_or_default]
    pub color: Option<ButtonColor>,
    /// Optional size class.
    #[prop_or_default]
    pub size: Option<ButtonSize>,
    /// Optional appearance variant.
    #[prop_or_default]
    pub variant: Option<ButtonVariant>,
    /// Make this button rounded.
    #[prop_or_default]
    pub rounded: bool,
    /// Make this button full width.
    #[prop_or_default]
    pub fullwidth: bool,
    /// The HTML `type` attribute.
    #[prop_or(ButtonType::Button)]
    pub button_type: ButtonType,
    /// Optional `aria-label` attribute.
    #[prop_or_default]
    pub aria_label: AttrValue,
    /// Optional `role` attribute.
    #[prop_or_default]
    pub role: AttrValue,
    /// Optional `tabindex` attribute.
    #[prop_or_default]
    pub tabindex: Option<i32>,
    /// Optional `aria-controls` attribute.
    #[prop_or_default]
    pub aria_controls: AttrValue,
    /// Optional `aria-haspopup` attribute.
    #[prop_or_default]
    pub aria_haspopup: AttrValue,
    /// Optional `aria-expanded` attribute.
    #[prop_or_default]
    pub aria_expanded: Option<bool>,
    /// Optional `aria-selected` attribute.
    #[prop_or_default]
    pub aria_selected: Option<bool>,
    /// Optional `aria-pressed` attribute.
    #[prop_or_default]
    pub aria_pressed: Option<bool>,
    /// Skip adding the default `button` Bulma class.
    #[prop_or_default]
    pub no_button_class: bool,
    #[prop_or_default]
    pub id: String,
}

/// A button element.
///
/// [https://bulma.io/documentation/elements/button/](https://bulma.io/documentation/elements/button/)
#[component(Button)]
pub fn button(props: &ButtonProps) -> Html {
    let aria_label = (!props.aria_label.is_empty()).then_some(props.aria_label.clone());
    let role = (!props.role.is_empty()).then_some(props.role.clone());
    let tabindex = props.tabindex.map(|value| value.to_string());
    let aria_controls = (!props.aria_controls.is_empty()).then_some(props.aria_controls.clone());
    let aria_haspopup = (!props.aria_haspopup.is_empty()).then_some(props.aria_haspopup.clone());
    let aria_expanded = props.aria_expanded.map(|value| if value { "true" } else { "false" });
    let aria_selected = props.aria_selected.map(|value| if value { "true" } else { "false" });
    let aria_pressed = props.aria_pressed.map(|value| if value { "true" } else { "false" });
    let class = classes!(
        (!props.no_button_class).then_some("button"),
        props.classes.clone(),
        props.loading.then_some("is-loading"),
        props.r#static.then_some("is-static"),
        props.color.as_ref().map(ToString::to_string),
        props.size.as_ref().map(ToString::to_string),
        props.variant.as_ref().map(ToString::to_string),
        props.rounded.then_some("is-rounded"),
        props.fullwidth.then_some("is-fullwidth")
    );
    let _id = props.id.clone();
    html! {
        <button
            id={_id}
            {class}
            type={props.button_type.to_string()}
            aria-label={aria_label}
            role={role}
            tabindex={tabindex}
            aria-controls={aria_controls}
            aria-haspopup={aria_haspopup}
            aria-expanded={aria_expanded}
            aria-selected={aria_selected}
            aria-pressed={aria_pressed}
            onclick={props.onclick.clone()}
            onkeydown={props.onkeydown.clone()}
            disabled={props.disabled}
        >
            {props.children.clone()}
        </button>
    }
}

//////////////////////////////////////////////////////////////////////////////
//////////////////////////////////////////////////////////////////////////////

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct IconButtonProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub classes: Classes,
    /// The click handler to use for this component.
    #[prop_or_default]
    pub onclick: Callback<MouseEvent>,
    /// Render a loading spinner within this component.
    #[prop_or_default]
    pub loading: bool,
    /// Make this component static.
    #[prop_or_default]
    pub r#static: bool,
    /// Disable this component.
    #[prop_or_default]
    pub disabled: bool,
    /// Optional semantic color.
    #[prop_or_default]
    pub color: Option<ButtonColor>,
    /// Optional size class.
    #[prop_or_default]
    pub size: Option<ButtonSize>,
    /// Optional appearance variant.
    #[prop_or_default]
    pub variant: Option<ButtonVariant>,
    /// Make this button rounded.
    #[prop_or_default]
    pub rounded: bool,
    /// Make this button full width.
    #[prop_or_default]
    pub fullwidth: bool,
    /// The HTML `type` attribute.
    #[prop_or(ButtonType::Button)]
    pub button_type: ButtonType,
    /// Optional `aria-label` attribute.
    #[prop_or_default]
    pub aria_label: AttrValue,
    #[prop_or_default]
    pub id: String,
    /// Optional icon slot rendered before text.
    #[prop_or_default]
    pub icon_left: Option<Html>,
    /// Optional icon slot rendered after text.
    #[prop_or_default]
    pub icon_right: Option<Html>,
}

/// A button with icon slots and sensible accessibility defaults.
#[component(IconButton)]
pub fn icon_button(props: &IconButtonProps) -> Html {
    let is_icon_only = props.children.is_empty();
    let mut aria_label = props.aria_label.clone();
    if is_icon_only && aria_label.is_empty() {
        aria_label = AttrValue::from("icon button");
    }

    html! {
        <Button
            classes={props.classes.clone()}
            onclick={props.onclick.clone()}
            loading={props.loading}
            r#static={props.r#static}
            disabled={props.disabled}
            color={props.color.clone()}
            size={props.size.clone()}
            variant={props.variant.clone()}
            rounded={props.rounded}
            fullwidth={props.fullwidth}
            button_type={props.button_type.clone()}
            aria_label={aria_label}
            id={props.id.clone()}
        >
            {
                props.icon_left
                    .as_ref()
                    .map(|icon| html! { <span class="icon" aria-hidden="true">{icon.clone()}</span> })
                    .unwrap_or_default()
            }
            {
                if !props.children.is_empty() {
                    html! { <span>{props.children.clone()}</span> }
                } else {
                    Html::default()
                }
            }
            {
                props.icon_right
                    .as_ref()
                    .map(|icon| html! { <span class="icon" aria-hidden="true">{icon.clone()}</span> })
                    .unwrap_or_default()
            }
        </Button>
    }
}

//////////////////////////////////////////////////////////////////////////////
//////////////////////////////////////////////////////////////////////////////

#[cfg(feature = "router")]
mod router {
    use super::*;
    use serde::Serialize;
    use yew_router::Routable;
    use yew_router::components::Link;

    #[derive(Clone, Properties, PartialEq)]
    pub struct ButtonRouterProps<R: Routable + Clone + PartialEq + 'static> {
        /// The Switched item representing the route.
        pub route: R,
        /// Html inside the component.
        #[prop_or_default]
        pub children: Children,
        /// Classes to be added to component.
        #[prop_or_default]
        pub classes: Classes,
        /// Render a loading spinner within this component.
        #[prop_or_default]
        pub loading: bool,
        /// Make this component static.
        #[prop_or_default]
        pub r#static: bool,
        /// Disable this component.
        #[prop_or_default]
        pub disabled: bool,
        /// Optional semantic color.
        #[prop_or_default]
        pub color: Option<ButtonColor>,
        /// Optional size class.
        #[prop_or_default]
        pub size: Option<ButtonSize>,
        /// Optional appearance variant.
        #[prop_or_default]
        pub variant: Option<ButtonVariant>,
        /// Make this button rounded.
        #[prop_or_default]
        pub rounded: bool,
        /// Make this button full width.
        #[prop_or_default]
        pub fullwidth: bool,
    }

    /// A Yew Router button element with Bulma styling.
    pub struct ButtonRouter<R: Routable + Clone + PartialEq + 'static, Q: Clone + PartialEq + Serialize + 'static = ()> {
        _route: std::marker::PhantomData<R>,
        _query: std::marker::PhantomData<Q>,
    }

    impl<R: Routable + Clone + PartialEq + 'static, Q: Clone + PartialEq + Serialize + 'static> Component for ButtonRouter<R, Q> {
        type Message = ();
        type Properties = ButtonRouterProps<R>;

        fn create(_ctx: &Context<Self>) -> Self {
            Self {
                _route: std::marker::PhantomData,
                _query: std::marker::PhantomData,
            }
        }

        fn view(&self, ctx: &Context<Self>) -> Html {
            let classes = classes!(
                ctx.props().classes.clone(),
                "button",
                ctx.props().loading.then_some("is-loading"),
                ctx.props().color.as_ref().map(ToString::to_string),
                ctx.props().size.as_ref().map(ToString::to_string),
                ctx.props().variant.as_ref().map(ToString::to_string),
                ctx.props().rounded.then_some("is-rounded"),
                ctx.props().fullwidth.then_some("is-fullwidth"),
            );
            html! {
                <Link<R, Q>
                    to={ctx.props().route.clone()}
                    disabled={ctx.props().disabled}
                    {classes}
                    children={ctx.props().children.clone()}
                />
            }
        }
    }

    /// A Yew Router anchor button element with Bulma styling.
    pub struct ButtonAnchorRouter<R: Routable + Clone + PartialEq + 'static, Q: Clone + PartialEq + Serialize + 'static = ()> {
        _route: std::marker::PhantomData<R>,
        _query: std::marker::PhantomData<Q>,
    }

    impl<R: Routable + Clone + PartialEq + 'static, Q: Clone + PartialEq + Serialize + 'static> Component for ButtonAnchorRouter<R, Q> {
        type Message = ();
        type Properties = ButtonRouterProps<R>;

        fn create(_ctx: &Context<Self>) -> Self {
            Self {
                _route: std::marker::PhantomData,
                _query: std::marker::PhantomData,
            }
        }

        fn view(&self, ctx: &Context<Self>) -> Html {
            let classes = classes!(
                ctx.props().classes.clone(),
                "button",
                ctx.props().loading.then_some("is-loading"),
                ctx.props().color.as_ref().map(ToString::to_string),
                ctx.props().size.as_ref().map(ToString::to_string),
                ctx.props().variant.as_ref().map(ToString::to_string),
                ctx.props().rounded.then_some("is-rounded"),
                ctx.props().fullwidth.then_some("is-fullwidth"),
            );
            html! {
                <Link<R, Q>
                    to={ctx.props().route.clone()}
                    disabled={ctx.props().disabled}
                    {classes}
                    children={ctx.props().children.clone()}
                />
            }
        }
    }
}

#[cfg(feature = "router")]
pub use router::{ButtonAnchorRouter, ButtonRouter, ButtonRouterProps};

//////////////////////////////////////////////////////////////////////////////
//////////////////////////////////////////////////////////////////////////////

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct ButtonAnchorProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub classes: Classes,
    /// The `href` attribute value to use for this component.
    #[prop_or_default]
    pub href: String,
    /// The click handler to use for this component.
    #[prop_or_default]
    pub onclick: Callback<MouseEvent>,
    /// Render a loading spinner within this component.
    #[prop_or_default]
    pub loading: bool,
    /// Make this component static.
    #[prop_or_default]
    pub r#static: bool,
    /// Disable this component.
    #[prop_or_default]
    pub disabled: bool,
    /// Optional semantic color.
    #[prop_or_default]
    pub color: Option<ButtonColor>,
    /// Optional size class.
    #[prop_or_default]
    pub size: Option<ButtonSize>,
    /// Optional appearance variant.
    #[prop_or_default]
    pub variant: Option<ButtonVariant>,
    /// Make this button rounded.
    #[prop_or_default]
    pub rounded: bool,
    /// Make this button full width.
    #[prop_or_default]
    pub fullwidth: bool,
    /// An optional `rel` for when this element is using the `a` tag.
    #[prop_or_default]
    pub rel: Option<String>,
    /// An optional `target` for when this element is using the `a` tag.
    #[prop_or_default]
    pub target: Option<String>,
}

/// An anchor element styled as a button.
///
/// [https://bulma.io/documentation/elements/button/](https://bulma.io/documentation/elements/button/)
#[component(ButtonAnchor)]
pub fn button_anchor(props: &ButtonAnchorProps) -> Html {
    let class = classes!(
        "button",
        props.classes.clone(),
        props.loading.then_some("is-loading"),
        props.r#static.then_some("is-static"),
        props.color.as_ref().map(ToString::to_string),
        props.size.as_ref().map(ToString::to_string),
        props.variant.as_ref().map(ToString::to_string),
        props.rounded.then_some("is-rounded"),
        props.fullwidth.then_some("is-fullwidth")
    );
    html! {
        <a
            {class}
            onclick={props.onclick.clone()}
            href={props.href.clone()}
            rel={props.rel.clone().unwrap_or_default()}
            target={props.target.clone().unwrap_or_default()}
            disabled={props.disabled}
        >
            {props.children.clone()}
        </a>
    }
}

//////////////////////////////////////////////////////////////////////////////
//////////////////////////////////////////////////////////////////////////////

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct ButtonInputSubmitProps {
    #[prop_or_default]
    pub classes: Classes,
    /// The submit handler to use for this component.
    #[prop_or_default]
    pub onsubmit: Callback<SubmitEvent>,
    /// Render a loading spinner within this component.
    #[prop_or_default]
    pub loading: bool,
    /// Make this component static.
    #[prop_or_default]
    pub r#static: bool,
    /// Disable this component.
    #[prop_or_default]
    pub disabled: bool,
}

/// An input element with `type="submit"` styled as a button.
///
/// [https://bulma.io/documentation/elements/button/](https://bulma.io/documentation/elements/button/)
#[component(ButtonInputSubmit)]
pub fn button_input_submit(props: &ButtonInputSubmitProps) -> Html {
    let class = classes!(
        "button",
        props.classes.clone(),
        props.loading.then_some("is-loading"),
        props.r#static.then_some("is-static"),
    );
    html! {
        <input type="submit" {class} onsubmit={props.onsubmit.clone()} disabled={props.disabled} />
    }
}

//////////////////////////////////////////////////////////////////////////////
//////////////////////////////////////////////////////////////////////////////

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct ButtonInputResetProps {
    #[prop_or_default]
    pub classes: Classes,
    /// The reset handler to use for this component.
    #[prop_or_default]
    pub onreset: Callback<Event>,
    /// Render a loading spinner within this component.
    #[prop_or_default]
    pub loading: bool,
    /// Make this component static.
    #[prop_or_default]
    pub r#static: bool,
    /// Disable this component.
    #[prop_or_default]
    pub disabled: bool,
}

/// An input element with `type="reset"` styled as a button.
///
/// [https://bulma.io/documentation/elements/button/](https://bulma.io/documentation/elements/button/)
#[component(ButtonInputReset)]
pub fn button_input_reset(props: &ButtonInputResetProps) -> Html {
    let class = classes!(
        "button",
        props.classes.clone(),
        props.loading.then_some("is-loading"),
        props.r#static.then_some("is-static"),
    );
    html! {
        <input type="reset" {class} onreset={props.onreset.clone()} disabled={props.disabled} />
    }
}
