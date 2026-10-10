use wasm_bindgen::JsCast;
use web_sys::HtmlDetailsElement;
use yew::prelude::*;

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct DisclosureProps {
    /// The always-visible line that opens and closes the rest. Plain text or
    /// inline content; never a button or a link (it is one already).
    pub summary: Html,
    /// Open on first render. The operator's own toggling wins afterwards.
    #[prop_or_default]
    pub open: bool,
    /// Called with the new state after the operator opens or closes it.
    #[prop_or_default]
    pub on_toggle: Option<Callback<bool>>,
    #[prop_or_default]
    pub id: Option<AttrValue>,
    /// `data-testid` of the `<details>`; the `<summary>` gets `<testid>-summary`.
    #[prop_or_default]
    pub testid: Option<AttrValue>,
    #[prop_or_default]
    pub classes: Classes,
    /// What is hidden until opened: the advanced options, the raw log.
    #[prop_or_default]
    pub children: Children,
}

/// Content that stays out of the way until asked for, over the native
/// `<details>` / `<summary>`.
///
/// The browser gives it the keyboard (Enter and Space on the summary), the
/// expanded state to a screen reader and find-in-page, with no script. The
/// summary is at least 24 CSS pixels high (WCAG 2.5.8).
#[component(Disclosure)]
pub fn disclosure(props: &DisclosureProps) -> Html {
    let ontoggle = props.on_toggle.clone().map(|on_toggle| {
        Callback::from(move |event: Event| {
            let open = event
                .target()
                .and_then(|target| target.dyn_into::<HtmlDetailsElement>().ok())
                .map(|details| details.open());
            if let Some(open) = open {
                on_toggle.emit(open);
            }
        })
    });
    let summary_testid = props.testid.as_ref().map(|testid| format!("{testid}-summary"));
    html! {
        <details class={classes!("ybc-disclosure", props.classes.clone())} id={props.id.clone()} open={props.open}
                 data-testid={props.testid.clone()} {ontoggle}>
            <summary style="min-height: 24px; cursor: pointer" data-testid={summary_testid}>{ props.summary.clone() }</summary>
            <div class="mt-2">{ props.children.clone() }</div>
        </details>
    }
}
