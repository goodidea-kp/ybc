use derive_more::Display;
use yew::prelude::*;

use crate::Alignment;

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct BreadcrumbProps {
    /// The `li` child elements of this breadcrumb.
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub classes: Classes,
    /// The size of this component.
    #[prop_or_default]
    pub size: Option<BreadcrumbSize>,
    /// The alignment of this component.
    #[prop_or_default]
    pub alignment: Option<Alignment>,
    /// The separator type to use between breadcrumb segments.
    #[prop_or_default]
    pub separator: Option<BreadcrumbSeparator>,
}

/// A simple breadcrumb component to improve your navigation experience.
///
/// [https://bulma.io/documentation/components/breadcrumb/](https://bulma.io/documentation/components/breadcrumb/)
#[function_component(Breadcrumb)]
pub fn breadcrumb(props: &BreadcrumbProps) -> Html {
    let class = classes!(
        "breadcrumb",
        props.classes.clone(),
        props.size.as_ref().map(|size| size.to_string()),
        props.alignment.as_ref().map(|alignment| alignment.to_string()),
        props.separator.as_ref().map(|separator| separator.to_string()),
    );
    html! {
        <nav {class} aria-label="breadcrumbs">
            <ul>
                {props.children.clone()}
            </ul>
        </nav>
    }
}

/// The 3 sizes available for a breadcrumb.
///
/// https://bulma.io/documentation/components/breadcrumb/#sizes
#[derive(Clone, Debug, Display, PartialEq, Eq)]
pub enum BreadcrumbSize {
    #[display("are-small")]
    Small,
    #[display("are-medium")]
    Medium,
    #[display("are-large")]
    Large,
}

/// The 4 additional separators for a breadcrump.
///
/// https://bulma.io/documentation/components/breadcrumb/#alternative-separators
#[derive(Clone, Debug, Display, PartialEq, Eq)]
pub enum BreadcrumbSeparator {
    #[display("has-arrow-separator")]
    Arrow,
    #[display("has-bullet-separator")]
    Bullet,
    #[display("has-dot-separator")]
    Dot,
    #[display("has-succeeds-separator")]
    Succeeds,
}
