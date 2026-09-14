use derive_more::Display;
use wasm_bindgen::UnwrapThrowExt;
use web_sys::HtmlInputElement;
use yew::prelude::*;

use crate::Size;
use crate::form::field::use_field_context;

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct InputProps {
    /// The `id` attribute for this form element.
    ///
    /// Defaults to the id minted by the enclosing `Field`, so a labelled field needs no explicit
    /// id. Set it to bind a label by hand or to give tests a stable target.
    #[prop_or_default]
    pub id: Option<AttrValue>,
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
    /// The `autocomplete` token, e.g. `"email"`, `"username"`, `"current-password"`,
    /// `"one-time-code"`. Drives browser autofill; `None` leaves it to the browser.
    #[prop_or_default]
    pub autocomplete: Option<AttrValue>,
    /// The virtual keyboard to show on touch devices. `None` leaves it to the browser.
    #[prop_or_default]
    pub inputmode: Option<InputMode>,
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
    /// Mark this component as required for form submission.
    #[prop_or_default]
    pub required: bool,
    /// Mark this component as invalid: adds `is-danger` and `aria-invalid="true"`.
    /// Also set automatically when the enclosing `Field` has `help_has_error`.
    #[prop_or_default]
    pub invalid: bool,

    #[prop_or_default]
    pub step: f32,
    /// The `maxlength` attribute for this form element. `None` omits the
    /// attribute (no limit); `Some(n)` caps input to `n` characters.
    #[prop_or_default]
    pub maxlength: Option<u32>,
}

/// A text input element.
///
/// [https://bulma.io/documentation/form/input/](https://bulma.io/documentation/form/input/)
///
/// All YBC form components are controlled components. This means that the value of the field must
/// be provided from a parent component, and changes to this component are propagated to the parent
/// component via callback.
#[component(Input)]
pub fn input(props: &InputProps) -> Html {
    let field = use_field_context();
    let id = props.id.clone().or(field.control_id);
    let invalid = props.invalid || field.has_error;
    let class = classes!(
        "input",
        props.classes.clone(),
        props.size.as_ref().map(|size| size.to_string()),
        props.rounded.then_some("is-rounded"),
        props.loading.then_some("is-loading"),
        props.r#static.then_some("is-static"),
        invalid.then_some("is-danger"),
    );
    let inputmode = props.inputmode.as_ref().map(|mode| mode.to_string());
    let oninput_text = props.update.reform(|ev: web_sys::InputEvent| {
        let input: HtmlInputElement = ev.target_dyn_into().expect_throw("event target should be an input");
        input.value()
    });

    let input_ref = use_node_ref();

    let oninput_number = props.update.reform({
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
                {id}
                name={props.name.clone()}
                value={props.value.clone()}
                {class}
                type={props.r#type.to_string()}
                ref={input_ref.clone()}
                oninput={oninput_number}
                oninvalid={oninvalid}
                placeholder={props.placeholder.clone()}
                autocomplete={props.autocomplete.clone()}
                {inputmode}
                disabled={props.disabled}
                readonly={props.readonly}
                required={props.required}
                aria-invalid={invalid.then_some("true")}
                aria-describedby={field.help_id}
                step={props.step.to_string()}
                pattern="[0-9]+([.][0-9]{0,2})?"
                maxlength={props.maxlength.map(|m| m.to_string())}
                />
        } else {
            <input
                {id}
                name={props.name.clone()}
                value={props.value.clone()}
                oninput={oninput_text}
                {class}
                type={props.r#type.to_string()}
                placeholder={props.placeholder.clone()}
                autocomplete={props.autocomplete.clone()}
                {inputmode}
                disabled={props.disabled}
                readonly={props.readonly}
                required={props.required}
                aria-invalid={invalid.then_some("true")}
                aria-describedby={field.help_id}
                maxlength={props.maxlength.map(|m| m.to_string())}
                />
        }
    }
}

/// The 4 allowed types for an input component.
///
/// https://bulma.io/documentation/form/input/
#[derive(Clone, Debug, Display, PartialEq, Eq)]
pub enum InputType {
    #[display("text")]
    Text,
    #[display("password")]
    Password,
    #[display("email")]
    Email,
    #[display("tel")]
    Tel,
    #[display("number")]
    Number,
}

/// The `inputmode` hint: which virtual keyboard a touch device should show.
///
/// https://html.spec.whatwg.org/multipage/interaction.html#attr-inputmode
#[derive(Clone, Debug, Display, PartialEq, Eq)]
pub enum InputMode {
    /// No virtual keyboard; the page draws its own input UI.
    #[display("none")]
    None,
    #[display("text")]
    Text,
    /// Digits plus the locale's decimal separator.
    #[display("decimal")]
    Decimal,
    /// Digits only, e.g. a PIN or one-time code.
    #[display("numeric")]
    Numeric,
    #[display("tel")]
    Tel,
    /// Text keyboard with a "search" action key.
    #[display("search")]
    Search,
    #[display("email")]
    Email,
    #[display("url")]
    Url,
}
