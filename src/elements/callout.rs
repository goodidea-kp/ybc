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

/// The class every [`Callout`] carries, for a page's own rules about what sits
/// inside one (secondary text, links) in each colour scheme.
pub const CALLOUT_CLASS: &str = "ybc-callout";

/// Layout plus the colour scheme, as inline style.
///
/// Bulma's `.notification.is-light` pins the background at 90% lightness in
/// both schemes, so in the dark scheme every callout was a light block, the
/// brightest thing on the screen (Bastion UX review S1-F14). Bulma's `soft`
/// and `bold` lightness tokens swap with the scheme -- 90%/20% light, 20%/90%
/// dark, under `prefers-color-scheme` and under `data-theme` alike -- so a
/// tint at `soft` with text at `bold` follows whichever scheme is shown, with
/// no media query and no stylesheet. Inline, it wins over the severity's own
/// rule. Contrast of the text (and the icon, drawn in the same colour) on the
/// tint, Bulma 1.0.4 colours: info 8.3:1, success 8.2:1, warning 7.5:1,
/// danger 9.4:1, the same in both schemes (WCAG 1.4.3 AA asks 4.5:1).
const CALLOUT_STYLE: &str = "display: flex; gap: 0.75rem; align-items: flex-start; \
    --bulma-notification-background-l: var(--bulma-soft-l); \
    --bulma-notification-color-l: var(--bulma-bold-l)";

/// A message that announces itself: a tinted Bulma notification with the role
/// its severity calls for and an icon that says the same as its colour. The
/// tint and its text follow the light or dark colour scheme.
///
/// Replaces hand-written `notification is-*` blocks, most of which had no role
/// and relied on colour alone.
#[component(Callout)]
pub fn callout(props: &CalloutProps) -> Html {
    let class = classes!(
        "notification",
        "is-light",
        CALLOUT_CLASS,
        props.severity.to_string(),
        props.compact.then_some("py-2 px-3"),
        props.classes.clone()
    );
    html! {
        <div {class} id={props.id.clone()} role={(!props.quiet).then_some(props.severity.role())} data-testid={props.testid.clone()}
             style={CALLOUT_STYLE}>
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
