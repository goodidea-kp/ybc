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
    /// `data-testid` of the reason's paragraph.
    #[prop_or_default]
    pub reason_testid: Option<AttrValue>,
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
    let reason_id = format!("{}-why-not", props.id);
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
            if let Some(reason) = &props.disabled_reason {
                <p class="help" id={reason_id} data-testid={props.reason_testid.clone()}>{ reason.clone() }</p>
            }
        </>
    }
}
