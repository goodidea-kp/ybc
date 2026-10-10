use derive_more::Display;
use yew::prelude::*;

use crate::Size;

/// What a [`StatusTag`] says about its subject: it picks the colour, and the
/// icon shown when the tag has neither text nor an icon of its own.
#[derive(Clone, Copy, Debug, Display, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum StatusTone {
    /// No colour: a plain fact.
    #[display("")]
    Neutral,
    #[display("is-info")]
    Info,
    #[display("is-success")]
    Success,
    #[display("is-warning")]
    Warning,
    #[display("is-danger")]
    Danger,
    #[display("is-primary")]
    Primary,
    #[display("is-link")]
    Link,
    #[display("is-dark")]
    Dark,
}

impl StatusTone {
    /// Font Awesome 6/7 solid icon used when a tag has no text and no icon of
    /// its own, so colour is never the only signal (WCAG 1.4.1).
    pub fn icon(self) -> &'static str {
        match self {
            StatusTone::Info | StatusTone::Primary | StatusTone::Link => "fas fa-circle-info",
            StatusTone::Success => "fas fa-circle-check",
            StatusTone::Warning => "fas fa-triangle-exclamation",
            StatusTone::Danger => "fas fa-circle-exclamation",
            StatusTone::Neutral | StatusTone::Dark => "fas fa-circle",
        }
    }

    /// The tone's word, the accessible name of an icon-only tag without a `title`.
    pub fn label(self) -> &'static str {
        match self {
            StatusTone::Neutral => "Status",
            StatusTone::Info | StatusTone::Primary | StatusTone::Link => "Information",
            StatusTone::Success => "Success",
            StatusTone::Warning => "Warning",
            StatusTone::Danger => "Danger",
            StatusTone::Dark => "Status",
        }
    }

    fn class(self) -> Option<String> {
        let class = self.to_string();
        (!class.is_empty()).then_some(class)
    }
}

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct StatusTagProps {
    pub tone: StatusTone,
    /// Font Awesome classes of an icon before the text, e.g. `"fas fa-lock"`.
    /// Always `aria-hidden` beside text.
    #[prop_or_default]
    pub icon: Option<AttrValue>,
    /// A tooltip with more detail. For a tag without text it is also the
    /// accessible name.
    #[prop_or_default]
    pub title: Option<AttrValue>,
    /// Bulma's pale variant (`is-light`).
    #[prop_or_default]
    pub light: bool,
    #[prop_or_default]
    pub size: Option<Size>,
    #[prop_or_default]
    pub id: Option<AttrValue>,
    #[prop_or_default]
    pub testid: Option<AttrValue>,
    /// Extra classes, e.g. spacing.
    #[prop_or_default]
    pub classes: Classes,
    /// The text: the state in the backend's own word (`ACTIVE`, `EXPIRED`).
    #[prop_or_default]
    pub children: Children,
}

/// A status chip that never relies on colour alone: it carries text, an icon,
/// or both.
///
/// With text, the icon (if any) is `aria-hidden` and the text is what a screen
/// reader hears. Without text, the tag shows its own icon -- or the tone's when
/// it has none -- as `role="img"` named by `title`, or by the tone's word when
/// there is no title either.
#[component(StatusTag)]
pub fn status_tag(props: &StatusTagProps) -> Html {
    let class = classes!(
        "tag",
        props.tone.class(),
        props.light.then_some("is-light"),
        props.size.as_ref().map(ToString::to_string),
        props.classes.clone()
    );
    if props.children.is_empty() {
        let icon = props.icon.clone().unwrap_or_else(|| AttrValue::from(props.tone.icon()));
        let name = props.title.clone().unwrap_or_else(|| AttrValue::from(props.tone.label()));
        return html! {
            <span {class} id={props.id.clone()} role="img" aria-label={name} title={props.title.clone()} data-testid={props.testid.clone()}>
                <span class="icon"><i class={icon} aria-hidden="true"></i></span>
            </span>
        };
    }
    html! {
        <span {class} id={props.id.clone()} title={props.title.clone()} data-testid={props.testid.clone()}>
            if let Some(icon) = &props.icon {
                <span class="icon"><i class={icon.clone()} aria-hidden="true"></i></span>
            }
            <span>{ props.children.clone() }</span>
        </span>
    }
}
