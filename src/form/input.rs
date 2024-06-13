use derive_more::Display;
use wasm_bindgen::UnwrapThrowExt;
use web_sys::HtmlInputElement;
use yew::prelude::*;

use crate::Size;

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct InputProps {
    /// The `name` attribute for this form element.
    pub name: String,
    /// The controlled value of this form element.
    pub value: String,
    /// The callback to be used for propagating changes to this element's value.
    pub update: Callback<String>,

    #[prop_or_default]
    pub classes: Classes,
    /// The input type of this component.
    #[prop_or_else(|| InputType::Text)]
    pub r#type: InputType,
    /// The placeholder value for this component.
    #[prop_or_default]
    pub placeholder: String,
    /// The size of this component.
    #[prop_or_default]
    pub size: Option<Size>,
    /// Use rounded appearance.
    #[prop_or_default]
    pub rounded: bool,
    /// Display a loading spinner within this component.
    #[prop_or_default]
    pub loading: bool,
    /// Disable this component.
    #[prop_or_default]
    pub disabled: bool,
    /// Make this component read-only.
    #[prop_or_default]
    pub readonly: bool,
    /// Make this component static.
    #[prop_or_default]
    pub r#static: bool,

    #[prop_or_default]
    pub step: f32,
}

/// A text input element.
///
/// [https://bulma.io/documentation/form/input/](https://bulma.io/documentation/form/input/)
///
/// All YBC form components are controlled components. This means that the value of the field must
/// be provided from a parent component, and changes to this component are propagated to the parent
/// component via callback.
#[function_component(Input)]
pub fn input(props: &InputProps) -> Html {
    let class = classes!(
        "input",
        props.classes.clone(),
        props.size.as_ref().map(|size| size.to_string()),
        props.rounded.then_some("is-rounded"),
        props.loading.then_some("is-loading"),
        props.r#static.then_some("is-static"),
    );
    let oninput_text = props.update.reform(|ev: web_sys::InputEvent| {
        let input: HtmlInputElement = ev.target_dyn_into().expect_throw("event target should be an input");
        input.value()
    });


    let input_ref = use_node_ref();

    let oninput_number =props.update.reform({
        let input_ref = input_ref.clone();
        move |_| {
            if let Some(input) = input_ref.cast::<HtmlInputElement>() {
                input.set_custom_validity("");
                input.check_validity();
                return input.value();
            };
            "".to_string()
        }
    });

    let oninvalid = Callback::from({
        let input_ref = input_ref.clone();
        move |_| {
            if let Some(input) = input_ref.cast::<HtmlInputElement>() {
                if input.value() == "" {
                    input.set_custom_validity("");
                } else {
                    input.set_custom_validity("Please enter a number with up to two decimal places.");
                }
            }
        }
    });

    html! {
        if props.r#type == InputType::Number {
            <input
                name={props.name.clone()}
                value={props.value.clone()}
                {class}
                type={props.r#type.to_string()}
                ref={input_ref.clone()}
                oninput={oninput_number}
                oninvalid={oninvalid}
                placeholder={props.placeholder.clone()}
                disabled={props.disabled}
                readonly={props.readonly}
                step={props.step.to_string()}
                pattern="[0-9]+([.][0-9]{0,2})?"
                />
        } else {
            <input
                name={props.name.clone()}
                value={props.value.clone()}
                oninput={oninput_text}
                {class}
                type={props.r#type.to_string()}
                placeholder={props.placeholder.clone()}
                disabled={props.disabled}
                readonly={props.readonly}
                />
        }
    }
}

/// The 4 allowed types for an input component.
///
/// https://bulma.io/documentation/form/input/
#[derive(Clone, Debug, Display, PartialEq, Eq)]
pub enum InputType {
    #[display(fmt = "text")]
    Text,
    #[display(fmt = "password")]
    Password,
    #[display(fmt = "email")]
    Email,
    #[display(fmt = "tel")]
    Tel,
    #[display(fmt = "number")]
    Number,
}
