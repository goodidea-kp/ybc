use std::sync::atomic::{AtomicUsize, Ordering};

use derive_more::Display;
use yew::prelude::*;

static FIELD_AUTO_ID: AtomicUsize = AtomicUsize::new(1);

fn next_field_id() -> String {
    format!("field-{}", FIELD_AUTO_ID.fetch_add(1, Ordering::Relaxed))
}

/// Values a form control inherits from its enclosing [`Field`].
///
/// `Field` provides this through a Yew context. The built-in controls (`Input`, `Select`,
/// `TextArea`, `File`, ...) read it automatically, so a plain `Field` + control pair is labelled and
/// described without any ids being written by hand. Custom controls can consume it with
/// `use_context::<FieldContext>()`.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct FieldContext {
    /// The id the field's label points at, for a control that has no explicit `id` of its own.
    ///
    /// `None` for `grouped` and `addons` fields, which hold several controls and would otherwise
    /// end up with duplicate ids.
    pub control_id: Option<AttrValue>,
    /// The id of the field's help paragraph, for `aria-describedby`. `None` without help text.
    pub help_id: Option<AttrValue>,
    /// Whether the field is showing an error (`help_has_error`). Controls render `aria-invalid`.
    pub has_error: bool,
}

/// Read the enclosing [`Field`]'s context, or a neutral default outside any field.
#[hook]
pub(crate) fn use_field_context() -> FieldContext {
    use_context::<FieldContext>().unwrap_or_default()
}

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct FieldProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub classes: Classes,
    /// A text label for the field.
    #[prop_or_default]
    pub label: Option<String>,
    /// The `id` of the form control this field's label describes.
    ///
    /// Rendered as `<label for="...">`. When omitted, the field generates an id and hands it to
    /// its control through [`FieldContext`], so you only need this when the control has an
    /// explicit `id` of its own or when the field is `grouped`/`addons`.
    #[prop_or_default]
    pub label_for: Option<AttrValue>,
    /// Extra classes for the label container.
    #[prop_or_default]
    pub label_classes: Classes,
    /// A help message for the field. Rendered as `<p class="help">` and linked to the control
    /// through `aria-describedby`.
    #[prop_or_default]
    pub help: Option<String>,
    /// Extra classes for the help message container.
    #[prop_or_default]
    pub help_classes: Classes,
    /// A convenience bool to add the `is-danger` class to the help classes when `true`.
    /// The field's control also receives `aria-invalid="true"` and the `is-danger` class.
    #[prop_or_default]
    pub help_has_error: bool,
    /// Has icons on the left of the field's controls.
    #[prop_or_default]
    pub icons_left: bool,
    /// Has icons on the right of the field's controls.
    #[prop_or_default]
    pub icons_right: bool,
    /// Allow addons to the field's controls.
    #[prop_or_default]
    pub addons: bool,
    /// Alignment for the field's addons.
    #[prop_or_default]
    pub addons_align: Option<AddonsAlign>,
    /// All controls in this field should be grouped.
    #[prop_or_default]
    pub grouped: bool,
    /// Alignment for grouped controls.
    #[prop_or_default]
    pub grouped_align: Option<GroupedAlign>,
    /// Allow the grouped controls to span multiple lines.
    #[prop_or_default]
    pub multiline: bool,
    /// Make this a horizontal field.
    #[prop_or_default]
    pub horizontal: bool,
}

/// A container for form controls.
///
/// [https://bulma.io/documentation/form/general/](https://bulma.io/documentation/form/general/)
///
/// A field with a `label` and a single control is accessible by default: the field mints an id,
/// points its `<label for>` at it, and the control picks the id up through [`FieldContext`].
/// Help text is linked to the control with `aria-describedby`, and `help_has_error` marks the
/// control `aria-invalid`.
#[component(Field)]
pub fn field(props: &FieldProps) -> Html {
    let class = classes!(
        "field",
        props.classes.clone(),
        props.icons_left.then_some("has-icons-left"),
        props.icons_right.then_some("has-icons-right"),
        props.addons.then_some("has-addons"),
        props.grouped.then_some("is-grouped"),
        props.multiline.then_some("is-multiline"),
        props.addons_align.as_ref().map(|align| align.to_string()),
        props.grouped_align.as_ref().map(|align| align.to_string()),
    );

    let auto_id = use_state(|| AttrValue::from(next_field_id()));
    let control_id = props.label_for.clone().unwrap_or_else(|| (*auto_id).clone());
    let holds_single_control = !props.grouped && !props.addons;
    let label_for = if holds_single_control {
        Some(control_id.clone())
    } else {
        props.label_for.clone()
    };
    let help_id = props.help.as_ref().map(|_| AttrValue::from(format!("{control_id}-help")));

    let context = FieldContext {
        control_id: holds_single_control.then(|| control_id.clone()),
        help_id: help_id.clone(),
        has_error: props.help_has_error,
    };

    let body = if props.horizontal {
        html! {<div class="field-body">{props.children.clone()}</div>}
    } else {
        html! {<>{props.children.clone()}</>}
    };

    html! {
        <ContextProvider<FieldContext> {context}>
            <div {class}>
                {render_label(props, label_for)}
                {body}
                {render_help(props, help_id)}
            </div>
        </ContextProvider<FieldContext>>
    }
}

fn render_label(props: &FieldProps, label_for: Option<AttrValue>) -> Html {
    let Some(label_content) = &props.label else {
        return html! {};
    };
    let label = html! {
        <label class={classes!("label", props.label_classes.clone())} for={label_for}>
            {label_content.clone()}
        </label>
    };
    if props.horizontal {
        html! {<div class="field-label">{label}</div>}
    } else {
        label
    }
}

fn render_help(props: &FieldProps, help_id: Option<AttrValue>) -> Html {
    let Some(help_content) = &props.help else {
        return html! {};
    };
    let class = classes!("help", props.help_classes.clone(), props.help_has_error.then_some("is-danger"));
    html! {<p id={help_id} {class}>{help_content.clone()}</p>}
}

/// The two alignment options available for field addons.
///
/// https://bulma.io/documentation/form/general/
#[derive(Clone, Debug, Display, PartialEq, Eq)]
#[display("has-addons-{}")]
pub enum AddonsAlign {
    #[display("centered")]
    Centered,
    #[display("right")]
    Right,
}

/// The two alignment options available for grouped field controls.
///
/// https://bulma.io/documentation/form/general/
#[derive(Clone, Debug, Display, PartialEq, Eq)]
#[display("is-grouped-{}")]
pub enum GroupedAlign {
    #[display("centered")]
    Centered,
    #[display("right")]
    Right,
}

/// The three sizes available for horizontal field labels.
///
/// https://bulma.io/documentation/form/general/#horizontal-form
#[derive(Clone, Debug, Display, PartialEq, Eq)]
#[display("is-{}")]
pub enum LabelSize {
    #[display("small")]
    Small,
    #[display("medium")]
    Medium,
    #[display("large")]
    Large,
}
