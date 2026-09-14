use wasm_bindgen::UnwrapThrowExt;
use web_sys::HtmlSelectElement;
use yew::prelude::*;

use crate::Size;
use crate::form::field::use_field_context;

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct SelectProps {
    /// The `id` attribute for the inner `select` element.
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

    /// The `option` & `optgroup` tags of this select component.
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub classes: Classes,

    /// A prompt shown while nothing is chosen, e.g. `"Choose a country"`.
    ///
    /// Rendered as a disabled first option with an empty value. It is selected while `value` is
    /// empty and cannot be re-chosen, so combined with `required` the browser rejects the form
    /// until the user picks a real option.
    #[prop_or_default]
    pub placeholder: Option<AttrValue>,
    /// The size of this component.
    #[prop_or_default]
    pub size: Option<Size>,
    /// Display a loading spinner within this component.
    #[prop_or_default]
    pub loading: bool,
    /// Disable this component.
    #[prop_or_default]
    pub disabled: bool,
    /// Mark this component as required for form submission.
    #[prop_or_default]
    pub required: bool,
    /// Mark this component as invalid: adds `is-danger` and `aria-invalid="true"`.
    /// Also set automatically when the enclosing `Field` has `help_has_error`.
    #[prop_or_default]
    pub invalid: bool,
}

/// A wrapper around an HTML `select` tag.
///
/// [https://bulma.io/documentation/form/select/](https://bulma.io/documentation/form/select/)
///
/// All YBC form components are controlled components. This means that the value of the field must
/// be provided from a parent component, and changes to this component are propagated to the parent
/// component via callback.
///
/// **NOTE WELL:** not all browsers will honor the value of the select element's value on initial
/// load. So if you have an initial `value` set for this component, ensure that the corresponding
/// option element also has the `selected=true` attribute. When `placeholder` is set and `value`
/// is empty, the placeholder option is selected for you.
#[component(Select)]
pub fn select(props: &SelectProps) -> Html {
    let field = use_field_context();
    let id = props.id.clone().or(field.control_id);
    let invalid = props.invalid || field.has_error;
    let class = classes!(
        "select",
        props.classes.clone(),
        props.size.as_ref().map(|size| size.to_string()),
        props.loading.then_some("is-loading"),
        invalid.then_some("is-danger"),
    );
    let onchange = props.update.reform(|ev: web_sys::Event| {
        let select: HtmlSelectElement = ev.target_dyn_into().expect_throw("event target should be a select");
        select.value()
    });
    let placeholder_option = props.placeholder.as_ref().map(|placeholder| {
        html! {
            <option value="" disabled=true selected={props.value.is_empty()}>{placeholder.clone()}</option>
        }
    });
    html! {
        <div {class}>
            <select
                {id}
                name={props.name.clone()}
                value={props.value.clone()}
                disabled={props.disabled}
                required={props.required}
                aria-invalid={invalid.then_some("true")}
                aria-describedby={field.help_id}
                {onchange}
            >
                {placeholder_option}
                {props.children.clone()}
            </select>
        </div>
    }
}

//////////////////////////////////////////////////////////////////////////////
//////////////////////////////////////////////////////////////////////////////

#[derive(Properties, Clone, PartialEq)]
pub struct MultiSelectProps {
    /// The `id` attribute for the inner `select` element.
    ///
    /// Defaults to the id minted by the enclosing `Field`, so a labelled field needs no explicit
    /// id. Set it to bind a label by hand or to give tests a stable target.
    #[prop_or_default]
    pub id: Option<AttrValue>,
    /// The `name` attribute for this form element.
    pub name: String,
    /// The controlled value of this form element.
    pub value: Vec<String>,
    /// The callback to be used for propagating changes to this element's value.
    pub update: Callback<Vec<String>>,

    /// The `option` & `optgroup` tags of this select component.
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub classes: Classes,

    /// The size of this component.
    #[prop_or_default]
    pub size: Option<Size>,
    /// Size of the list to display.
    #[prop_or_else(|| 4)]
    pub list_size: u32,
    /// Display a loading spinner within this component.
    #[prop_or_default]
    pub loading: bool,
    /// Disable this component.
    #[prop_or_default]
    pub disabled: bool,
    /// Mark this component as required for form submission.
    #[prop_or_default]
    pub required: bool,
    /// Mark this component as invalid: adds `is-danger` and `aria-invalid="true"`.
    /// Also set automatically when the enclosing `Field` has `help_has_error`.
    #[prop_or_default]
    pub invalid: bool,
}

/// A wrapper around an HTML `select` tag with the `multiple=true` attribute.
///
/// [https://bulma.io/documentation/form/select/](https://bulma.io/documentation/form/select/)
///
/// All YBC form components are controlled components. This means that the value of the field must
/// be provided from a parent component, and changes to this component are propagated to the parent
/// component via callback.
///
/// **NOTE WELL:** not all browsers will honor the value of the select element's value on initial
/// load. So if you have an initial `value` set for this component, ensure that the corresponding
/// option element also has the `selected=true` attribute.
#[component(MultiSelect)]
pub fn multi_select(props: &MultiSelectProps) -> Html {
    let field = use_field_context();
    let id = props.id.clone().or(field.control_id);
    let invalid = props.invalid || field.has_error;
    let class = classes!(
        "select",
        "is-multiple",
        props.classes.clone(),
        props.size.as_ref().map(|size| size.to_string()),
        props.loading.then_some("is-loading"),
        invalid.then_some("is-danger"),
    );
    let size = props.list_size.to_string();
    let onchange = props.update.reform(|ev: web_sys::Event| {
        let select: HtmlSelectElement = ev.target_dyn_into().expect_throw("event target should be a select");
        let opts = select.selected_options();
        (0..opts.length())
            .filter_map(|idx| opts.item(idx))
            .filter_map(|elem| elem.get_attribute("value").or_else(|| elem.text_content()))
            .collect::<Vec<_>>()
    });
    html! {
        <div {class}>
            <select
                {id}
                multiple=true
                size={size}
                name={props.name.clone()}
                value={props.value.join(",")}
                disabled={props.disabled}
                required={props.required}
                aria-invalid={invalid.then_some("true")}
                aria-describedby={field.help_id}
                {onchange}
            >
                {props.children.clone()}
            </select>
        </div>
    }
}
