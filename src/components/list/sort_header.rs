use yew::prelude::*;

use super::MIN_TARGET_STYLE;

/// Which way a sorted column runs.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum SortDirection {
    Ascending,
    Descending,
}

impl SortDirection {
    /// The `aria-sort` value of a header sorted this way.
    pub fn aria_sort(self) -> &'static str {
        match self {
            SortDirection::Ascending => "ascending",
            SortDirection::Descending => "descending",
        }
    }

    /// Font Awesome 6/7 solid icon shown beside the label.
    pub fn icon(self) -> &'static str {
        match self {
            SortDirection::Ascending => "fa-sort-up",
            SortDirection::Descending => "fa-sort-down",
        }
    }

    /// The other way round: what a second click on a sorted header asks for.
    pub fn reversed(self) -> SortDirection {
        match self {
            SortDirection::Ascending => SortDirection::Descending,
            SortDirection::Descending => SortDirection::Ascending,
        }
    }
}

/// The `aria-sort` value of a header: `none` when the list is not sorted by it.
fn aria_sort(direction: Option<SortDirection>) -> &'static str {
    direction.map_or("none", SortDirection::aria_sort)
}

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct SortHeaderProps<C: Clone + PartialEq + 'static> {
    /// The column this header sorts by; [`on_sort`](Self::on_sort) gets it back.
    pub column: C,
    /// The visible column name.
    pub label: AttrValue,
    /// `Some` when the list is sorted by this column, and which way.
    #[prop_or_default]
    pub direction: Option<SortDirection>,
    /// Called with [`column`](Self::column) on a click (or Enter / Space).
    /// Which direction follows is the page's decision.
    pub on_sort: Callback<C>,
    /// `data-testid` of the button.
    #[prop_or_default]
    pub testid: Option<AttrValue>,
    /// Extra classes on the `<th>`.
    #[prop_or_default]
    pub classes: Classes,
}

/// A `<th>` whose `<button>` sorts the list by its column; `aria-sort` on the
/// header says whether and which way the list is sorted by it.
///
/// The button is a real button (keyboard and screen reader get it for free)
/// and at least 24×24 CSS pixels, although it keeps the compact look of a
/// table header.
#[component(SortHeader)]
pub fn sort_header<C: Clone + PartialEq + 'static>(props: &SortHeaderProps<C>) -> Html {
    let onclick = {
        let (column, on_sort) = (props.column.clone(), props.on_sort.clone());
        Callback::from(move |_: MouseEvent| on_sort.emit(column.clone()))
    };
    let icon = props.direction.map_or("fa-sort", SortDirection::icon);
    html! {
        <th class={props.classes.clone()} aria-sort={aria_sort(props.direction)}>
            <button type="button" class="button is-text is-small px-1 py-0" style={MIN_TARGET_STYLE}
                    data-testid={props.testid.clone()} {onclick}>
                <span>{ props.label.clone() }</span>
                <span class="icon is-small"><i class={classes!("fas", icon)} aria-hidden="true"></i></span>
            </button>
        </th>
    }
}
