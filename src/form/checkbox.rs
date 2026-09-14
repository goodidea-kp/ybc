use yew::prelude::*;

use crate::form::field::use_field_context;

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct CheckboxProps {
    /// The `id` attribute for the inner `input` element.
    ///
    /// The checkbox is already wrapped in its own `<label>`, so this is mainly for tests and
    /// `aria-describedby`. Unlike single-control components it does not inherit the enclosing
    /// `Field`'s id, because several checkboxes commonly share one field.
    #[prop_or_default]
    pub id: Option<AttrValue>,
    /// The `name` attribute for this form element.
    pub name: String,
    /// The controlled value of this form element.
    pub checked: bool,
    /// The callback to be used for propagating changes to this element's value.
    pub update: Callback<bool>,
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub classes: Classes,
    /// Disable this component.
    #[prop_or_default]
    pub disabled: bool,
    /// Mark this component as required for form submission.
    #[prop_or_default]
    pub required: bool,
    /// Mark this component as invalid (`aria-invalid="true"`).
    /// Also set automatically when the enclosing `Field` has `help_has_error`.
    #[prop_or_default]
    pub invalid: bool,
}

/// The 2-state checkbox in its native format.
///
/// [https://bulma.io/documentation/form/checkbox/](https://bulma.io/documentation/form/checkbox/)
///
/// All YBC form components are controlled components. This means that the value of the field must
/// be provided from a parent component, and changes to this component are propagated to the parent
/// component via callback.
#[component(Checkbox)]
pub fn checkbox(props: &CheckboxProps) -> Html {
    let field = use_field_context();
    let invalid = props.invalid || field.has_error;
    let class = classes!("checkbox", props.classes.clone());
    let checked = props.checked;
    html! {
        <label {class} disabled={props.disabled}>
            <input
                id={props.id.clone()}
                type="checkbox"
                checked={props.checked}
                name={props.name.clone()}
                onclick={props.update.reform(move |_| !checked)}
                disabled={props.disabled}
                required={props.required}
                aria-invalid={invalid.then_some("true")}
                aria-describedby={field.help_id}
                />
            {props.children.clone()}
        </label>
    }
}
