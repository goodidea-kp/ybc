use yew::prelude::*;

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct ProgressProps {
    #[prop_or_default]
    pub classes: Classes,
    /// The maximum amount of progress; the 100% value.
    #[prop_or_else(|| 1.0)]
    pub max: f32,
    /// The amount of progress which has been made. `-1.0` means "unknown": the
    /// bar is then indeterminate (an animated stripe) and has no `value`.
    #[prop_or_else(|| 0.0)]
    pub value: f32,
    /// The bar's accessible name (`aria-label`). A bare `<progress>` has none,
    /// so a screen reader says only "progress bar" unless a visible label is
    /// associated with it. Absent by default: nothing is rendered.
    #[prop_or_default]
    pub aria_label: Option<AttrValue>,
}

/// A native HTML progress bar.
///
/// [https://bulma.io/documentation/elements/progress/](https://bulma.io/documentation/elements/progress/)
#[component(Progress)]
pub fn progress(props: &ProgressProps) -> Html {
    let class = classes!("progress", props.classes.clone());
    let max = props.max.to_string();
    let value = props.value.to_string();
    let value_txt = format!("{}%", value);
    let aria_label = props.aria_label.clone();
    if props.value == -1.0 {
        html! {
            <progress {class} {max} aria-label={aria_label}/>
        }
    } else {
        html! {
            <progress {class} {max} {value} aria-label={aria_label}>
                {value_txt}
            </progress>
        }
    }
}
