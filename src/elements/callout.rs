use derive_more::Display;
use yew::prelude::*;

/// How much a [`Callout`] matters: it picks the colour, the icon and the ARIA role.
#[derive(Clone, Copy, Debug, Display, PartialEq, Eq)]
pub enum CalloutSeverity {
    #[display("is-info")]
    Info,
    #[display("is-success")]
    Success,
    #[display("is-warning")]
    Warning,
    #[display("is-danger")]
    Danger,
}

impl CalloutSeverity {
    /// `alert` interrupts a screen reader; only a danger deserves it (WCAG 4.1.3).
    pub fn role(self) -> &'static str {
        match self {
            CalloutSeverity::Danger => "alert",
            _ => "status",
        }
    }

    /// Font Awesome 6/7 solid icon: colour is never the only signal (WCAG 1.4.1).
    pub fn icon(self) -> &'static str {
        match self {
            CalloutSeverity::Info => "fa-circle-info",
            CalloutSeverity::Success => "fa-circle-check",
            CalloutSeverity::Warning => "fa-triangle-exclamation",
            CalloutSeverity::Danger => "fa-circle-exclamation",
        }
    }
}

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct CalloutProps {
    pub severity: CalloutSeverity,
    /// A short bold line above the body, when the body needs one.
    #[prop_or_default]
    pub title: Option<AttrValue>,
    /// Tighter padding, for a callout inside a form or a card.
    #[prop_or_default]
    pub compact: bool,
    #[prop_or_default]
    pub testid: Option<AttrValue>,
    /// Inside a live region that already announces it (`aria-live` on a wrapper
    /// that stays on the page): no role of its own, so it is not read twice.
    #[prop_or_default]
    pub quiet: bool,
    /// For `aria-describedby` from elsewhere, e.g. a [`crate::ReasonedButton`]'s `reason_elsewhere`.
    #[prop_or_default]
    pub id: Option<AttrValue>,
    #[prop_or_default]
    pub classes: Classes,
    #[prop_or_default]
    pub children: Children,
}

/// A message that announces itself: a Bulma light notification with the role
/// its severity calls for and an icon that says the same as its colour.
///
/// Replaces hand-written `notification is-*` blocks, most of which had no role
/// and relied on colour alone.
#[component(Callout)]
pub fn callout(props: &CalloutProps) -> Html {
    let class = classes!(
        "notification",
        "is-light",
        props.severity.to_string(),
        props.compact.then_some("py-2 px-3"),
        props.classes.clone()
    );
    html! {
        <div {class} id={props.id.clone()} role={(!props.quiet).then_some(props.severity.role())} data-testid={props.testid.clone()}
             style="display: flex; gap: 0.75rem; align-items: flex-start">
            <span class="icon" style="flex: none">
                <i class={classes!("fas", props.severity.icon())} aria-hidden="true"></i>
            </span>
            <div style="min-width: 0; flex: 1 1 auto">
                if let Some(title) = &props.title {
                    <p class="has-text-weight-semibold">{ title.clone() }</p>
                }
                { props.children.clone() }
            </div>
        </div>
    }
}
