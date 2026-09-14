use wasm_bindgen::UnwrapThrowExt;
use web_sys::HtmlInputElement;
use yew::prelude::*;

use crate::form::field::use_field_context;

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct RadioProps {
    /// The `id` attribute for the inner `input` element.
    ///
    /// The radio is already wrapped in its own `<label>`, so this is mainly for tests and
    /// `aria-describedby`. It does not inherit the enclosing `Field`'s id, because a radio group
    /// always holds several radios.
    #[prop_or_default]
    pub id: Option<AttrValue>,
    /// The `name` attribute for this form element.
    ///
    /// All members of the same radio group must have the same value for their `name` attribute.
    pub name: String,
    /// The `value` attribute for this form element.
    ///
    /// This is different from other form elements, as this value does not change. It represents
    /// the value to be used for the radio group overall when this element is selected.
    pub value: String,
    /// The value of the currently selected radio of this radio group.
    pub checked_value: Option<String>,
    /// The callback to be used for propagating changes to the selected radio of the radio group.
    pub update: Callback<String>,
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub classes: Classes,
    /// Disable this component.
    #[prop_or_default]
    pub disabled: bool,
    /// Mark this radio group as required: one radio must be chosen for form submission.
    /// Setting it on a single member is enough for the browser to enforce it for the group.
    #[prop_or_default]
    pub required: bool,
    /// Mark this component as invalid (`aria-invalid="true"`).
    /// Also set automatically when the enclosing `Field` has `help_has_error`.
    #[prop_or_default]
    pub invalid: bool,
}

/// The mutually exclusive radio buttons in their native format.
///
/// [https://bulma.io/documentation/form/radio/](https://bulma.io/documentation/form/radio/)
///
/// All YBC form components are controlled components. This means that the value of the field must
/// be provided from a parent component, and changes to this component are propagated to the parent
/// component via callback.
///
/// Wrap the members of a group in a [`RadioGroup`] so the group has an accessible name.
#[component(Radio)]
pub fn radio(props: &RadioProps) -> Html {
    let field = use_field_context();
    let invalid = props.invalid || field.has_error;
    let class = classes!("radio", props.classes.clone());
    let oninput = props.update.reform(|ev: web_sys::InputEvent| {
        let input: HtmlInputElement = ev.target_dyn_into().expect_throw("event target should be an input");
        input.value()
    });
    html! {
        <label {class} disabled={props.disabled}>
            <input
                id={props.id.clone()}
                type="radio"
                name={props.name.clone()}
                value={props.value.clone()}
                checked={props.checked_value.as_ref().map(|val| val == &props.value).unwrap_or(false)}
                {oninput}
                disabled={props.disabled}
                required={props.required}
                aria-invalid={invalid.then_some("true")}
                aria-describedby={field.help_id}
                />
            {props.children.clone()}
        </label>
    }
}

//////////////////////////////////////////////////////////////////////////////
//////////////////////////////////////////////////////////////////////////////

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct RadioGroupProps {
    /// The question the group answers, e.g. `"Shipping speed"`. Rendered as the `<legend>`, which
    /// assistive technology announces together with each radio's own label.
    pub legend: AttrValue,
    /// The `Radio` members of this group.
    #[prop_or_default]
    pub children: Children,
    /// Extra classes for the `fieldset`. It always carries `field`, so groups space like fields.
    #[prop_or_default]
    pub classes: Classes,
    /// Extra classes for the `legend`. It always carries `label`.
    #[prop_or_default]
    pub legend_classes: Classes,
    /// The `id` attribute of the `fieldset`.
    #[prop_or_default]
    pub id: Option<AttrValue>,
    /// Disable every radio in the group.
    #[prop_or_default]
    pub disabled: bool,
}

/// A named group of [`Radio`] buttons.
///
/// Renders a `<fieldset>` with a `<legend>`, which is the HTML way to give a set of radios one
/// shared accessible name. Keyboard users move between the members with the arrow keys as usual;
/// the group only adds the name and lets the whole set be disabled at once.
///
/// ```rust,ignore
/// <ybc::RadioGroup legend="Shipping speed">
///     <ybc::Radio name="speed" value="standard" checked_value={speed.clone()} update={set_speed.clone()}>{"Standard"}</ybc::Radio>
///     <ybc::Radio name="speed" value="express" checked_value={speed.clone()} update={set_speed.clone()}>{"Express"}</ybc::Radio>
/// </ybc::RadioGroup>
/// ```
#[component(RadioGroup)]
pub fn radio_group(props: &RadioGroupProps) -> Html {
    let class = classes!("field", props.classes.clone());
    let legend_class = classes!("label", props.legend_classes.clone());
    html! {
        <fieldset id={props.id.clone()} {class} disabled={props.disabled}>
            <legend class={legend_class}>{props.legend.clone()}</legend>
            {props.children.clone()}
        </fieldset>
    }
}
