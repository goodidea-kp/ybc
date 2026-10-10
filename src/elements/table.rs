use yew::prelude::*;

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct TableProps {
    #[prop_or_default]
    pub children: Children,
    /// Extra classes for the `<table>` element (never for the scroll container).
    #[prop_or_default]
    pub classes: Classes,
    /// Add borders to all the cells.
    #[prop_or_default]
    pub bordered: bool,
    /// Add stripes to the table.
    #[prop_or_default]
    pub striped: bool,
    /// Make the cells narrower.
    #[prop_or_default]
    pub narrow: bool,
    /// Add a hover effect on each row.
    #[prop_or_default]
    pub hoverable: bool,
    /// Make the table fullwidth.
    #[prop_or_default]
    pub fullwidth: bool,
    /// Make the table scrollable, wrapping the table in a `div.table-container`.
    ///
    /// The wrapper is the only scroll container; every attribute below stays on
    /// the `<table>` itself either way.
    #[prop_or_default]
    pub scrollable: bool,
    /// `id` of the `<table>` element.
    #[prop_or_default]
    pub id: Option<AttrValue>,
    /// `data-testid` of the `<table>` element.
    #[prop_or_default]
    pub testid: Option<AttrValue>,
    /// Accessible name, when no `<caption>` or visible heading names the table.
    #[prop_or_default]
    pub aria_label: Option<AttrValue>,
    /// `id` of a visible heading that names the table.
    #[prop_or_default]
    pub aria_labelledby: Option<AttrValue>,
    /// `id` of text that describes the table.
    #[prop_or_default]
    pub aria_describedby: Option<AttrValue>,
}

/// An HTML table component.
///
/// It only styles and labels the table: sorting, paging and selection stay with
/// the caller, which renders `caption`/`thead`/`tbody` as children.
///
/// [https://bulma.io/documentation/elements/table/](https://bulma.io/documentation/elements/table/)
#[component(Table)]
pub fn table(props: &TableProps) -> Html {
    let class = classes!(
        "table",
        props.classes.clone(),
        props.bordered.then_some("is-bordered"),
        props.striped.then_some("is-striped"),
        props.narrow.then_some("is-narrow"),
        props.hoverable.then_some("is-hoverable"),
        props.fullwidth.then_some("is-fullwidth"),
    );
    let table = html! {
        <table {class}
            id={props.id.clone()}
            data-testid={props.testid.clone()}
            aria-label={props.aria_label.clone()}
            aria-labelledby={props.aria_labelledby.clone()}
            aria-describedby={props.aria_describedby.clone()}>
            {props.children.clone()}
        </table>
    };
    if props.scrollable {
        html! { <div class="table-container">{table}</div> }
    } else {
        table
    }
}
