use yew::prelude::*;

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct ReasonedButtonProps {
    /// Why the button does nothing now; `None` when it works.
    #[prop_or_default]
    pub disabled_reason: Option<AttrValue>,
    /// Unique on the page: the reason's id is derived from it.
    pub id: AttrValue,
    #[prop_or_default]
    pub onclick: Callback<MouseEvent>,
    /// Defaults to `button`.
    #[prop_or_default]
    pub classes: Classes,
    #[prop_or_default]
    pub testid: Option<AttrValue>,
    /// `data-testid` of the reason's line.
    #[prop_or_default]
    pub reason_testid: Option<AttrValue>,
    /// The id of an element on the page that already shows the reason -- one
    /// line for a whole toolbar, or a notice beside the button. The button then
    /// points to it instead of repeating the reason under itself.
    #[prop_or_default]
    pub reason_elsewhere: Option<AttrValue>,
    #[prop_or_default]
    pub children: Children,
}

/// A button that says why it does not work, to everyone.
///
/// `disabled` takes a button out of the tab order and a `title` on a wrapper is
/// never seen on a keyboard, a phone or a screen reader. This one stays
/// focusable with `aria-disabled`, shows the reason under it and points to it
/// with `aria-describedby`; a click while disabled does nothing.
#[component(ReasonedButton)]
pub fn reasoned_button(props: &ReasonedButtonProps) -> Html {
    let reason_id = props
        .reason_elsewhere
        .as_ref()
        .map(|id| id.to_string())
        .unwrap_or_else(|| format!("{}-why-not", props.id));
    let shown_here = props.reason_elsewhere.is_none();
    let disabled = props.disabled_reason.is_some();
    let onclick = {
        let onclick = props.onclick.clone();
        Callback::from(move |event: MouseEvent| {
            if disabled {
                event.prevent_default();
            } else {
                onclick.emit(event);
            }
        })
    };
    let classes = if props.classes.is_empty() {
        classes!("button")
    } else {
        props.classes.clone()
    };
    html! {
        <>
            <button type="button" id={props.id.clone()} class={classes} {onclick}
                    aria-disabled={disabled.then_some("true")}
                    aria-describedby={disabled.then(|| reason_id.clone())}
                    style={disabled.then_some("opacity: 0.55; cursor: not-allowed")}
                    data-testid={props.testid.clone()}>
                { props.children.clone() }
            </button>
            if let (Some(reason), true) = (&props.disabled_reason, shown_here) {
                // A <span>, not a <p>: the button often sits in a phrasing-only parent.
                <span class="help" style="display: block" id={reason_id} data-testid={props.reason_testid.clone()}>{ reason.clone() }</span>
            }
        </>
    }
}
