use crate::{Icon, Size};
use wasm_bindgen::UnwrapThrowExt;
use web_sys::HtmlTextAreaElement;
use yew::prelude::*;

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct TextAreaProps {
    /// The `name` attribute for this form element.
    pub name: String,
    /// The controlled value of this form element.
    pub value: String,
    /// The callback to be used for propagating changes to this element's value.
    pub update: Callback<String>,

    #[prop_or_default]
    pub classes: Classes,
    /// The placeholder value for this component.
    #[prop_or_default]
    pub placeholder: String,
    /// The number of rows to which this component will be locked.
    #[prop_or_default]
    pub rows: u32,

    /// The size of this component.
    #[prop_or_default]
    pub size: Option<Size>,
    /// Fix the size of this component.
    #[prop_or_default]
    pub fixed_size: bool,
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
    pub is_genai: bool,
}

/// A multiline textarea component.
///
/// [https://bulma.io/documentation/form/textarea/](https://bulma.io/documentation/form/textarea/)
///
/// All YBC form components are controlled components. This means that the value of the field must
/// be provided from a parent component, and changes to this component are propagated to the parent
/// component via callback.
#[function_component(TextArea)]
pub fn text_area(props: &TextAreaProps) -> Html {
    let class = classes!(
        "textarea",
        props.classes.clone(),
        props.size.as_ref().map(|size| size.to_string()),
        props.loading.then_some("is-loading"),
        props.r#static.then_some("is-static"),
        props.fixed_size.then_some("has-fixed-size"),
    );
    let genai = use_state(|| props.is_genai);
    let value = use_state(|| props.value.clone());
    let input_ref = use_node_ref();
    let oninput = {
        let value = value.clone();
        let update = props.update.clone();
        let _genai = genai.clone();
        Callback::from(move |ev: InputEvent| {
            let input: HtmlTextAreaElement = ev.target_dyn_into().expect_throw("event target should be a text area");
            _genai.set(false);
            let input_value = input.value();
            value.set(input_value.clone()); // Update the local state
            update.emit(input_value); // Emit the new value to the parent component
        })
    };
    {
        let value = value.clone();
        use_effect_with(props.value.clone(), move |value_prop| {
            value.set(value_prop.clone());
            || ()
        });
    }
    {
        let gen1 = genai.clone();
        use_effect_with(props.is_genai, move |value_prop| {
            gen1.set(value_prop.clone());
            || ()
        });
    }

    html! {
        if props.is_genai {
            <div id="context" style="position:relative">
                if *genai {
                    <Icon size={Size::Small} classes={classes!("is-pulled-right","ribbon")}>
                        <img src="chatgpt.svg"/>
                    </Icon>
                }
                <textarea
                    name={props.name.clone()}
                    value={(*value).clone()}
                    {oninput}
                    {class}
                    rows={props.rows.to_string()}
                    placeholder={props.placeholder.clone()}
                    disabled={props.disabled}
                    readonly={props.readonly}
                    ref={input_ref}
                    />
            </div>
        } else {
            <textarea
                name={props.name.clone()}
                value={props.value.clone()}
                {oninput}
                {class}
                rows={props.rows.to_string()}
                placeholder={props.placeholder.clone()}
                disabled={props.disabled}
                readonly={props.readonly}
                ref={input_ref}
                />
        }
    }
}
