use yew::prelude::*;

use super::{MIN_TARGET_STYLE, next_list_id};
use crate::ReasonedButton;

/// Up to this many pages every page gets a link; past it, the first, the last
/// and the neighbours of the current page do, with ellipses between.
const ALL_LINKS_UP_TO: u64 = 7;

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct PagerProps {
    /// The page shown, zero-based.
    pub current: u64,
    /// The last page, zero-based: `total_pages - 1`. Zero draws nothing.
    pub last: u64,
    /// Called with the zero-based page asked for.
    pub on_page: Callback<u64>,
    /// The name of the pagination landmark, e.g. "Domain pages". Unique on
    /// the page when there is more than one list.
    pub aria_label: AttrValue,
    /// Prefix of the test ids: `<testid>-pagination`, `-page-prev`,
    /// `-page-next`, `-page-<n>` (one-based) and `-page-edge`. It also seeds
    /// the element ids; without it they are generated.
    #[prop_or_default]
    pub testid: Option<AttrValue>,
    /// Extra classes on the `<nav>`; `is-small` is there already.
    #[prop_or_default]
    pub classes: Classes,
}

/// Previous, Next and the numbered page buttons of one list; nothing at all
/// for a single page.
///
/// The current page has `aria-current="page"`. At either end Previous or Next
/// stays focusable with `aria-disabled` and points to one line under the bar
/// that says why ("This is the last page."). Every button is at least 24×24
/// CSS pixels.
#[component(Pager)]
pub fn pager(props: &PagerProps) -> Html {
    let generated = use_state(|| next_list_id("ybc-pager"));
    if props.last == 0 {
        return html! {};
    }
    let prefix = props.testid.as_ref().map_or_else(|| (*generated).clone(), ToString::to_string);
    let testid = |suffix: &str| props.testid.as_ref().map(|_| AttrValue::from(format!("{prefix}-{suffix}")));
    let current = props.current.min(props.last);
    let go = |page: u64| props.on_page.reform(move |_: MouseEvent| page);
    let links = page_links(current, props.last + 1)
        .into_iter()
        .map(|link| page_link(link, current, &go, &testid))
        .collect::<Html>();
    let edge = page_edge(current, props.last);
    let edge_id = AttrValue::from(format!("{prefix}-page-edge"));
    let at_start = current == 0;
    let at_end = current >= props.last;
    html! {
        <>
            <nav class={classes!("pagination", "is-small", props.classes.clone())} role="navigation"
                 aria-label={props.aria_label.clone()} data-testid={testid("pagination")}>
                <ReasonedButton id={format!("{prefix}-page-prev")} classes={classes!("pagination-previous")}
                        style={AttrValue::from(MIN_TARGET_STYLE)}
                        disabled_reason={at_start.then(|| AttrValue::from(edge.unwrap_or_default()))}
                        reason_elsewhere={at_start.then(|| edge_id.clone())}
                        testid={testid("page-prev")} onclick={go(current.saturating_sub(1))}>{ "Previous" }</ReasonedButton>
                <ReasonedButton id={format!("{prefix}-page-next")} classes={classes!("pagination-next")}
                        style={AttrValue::from(MIN_TARGET_STYLE)}
                        disabled_reason={at_end.then(|| AttrValue::from(edge.unwrap_or_default()))}
                        reason_elsewhere={at_end.then(|| edge_id.clone())}
                        testid={testid("page-next")} onclick={go((current + 1).min(props.last))}>{ "Next" }</ReasonedButton>
                <ul class="pagination-list">{ links }</ul>
            </nav>
            if let Some(edge) = edge {
                <p class="help" id={edge_id.clone()} data-testid={testid("page-edge")}>{ edge }</p>
            }
        </>
    }
}

/// One `<li>`: a page button, or an ellipsis for `None`.
fn page_link(link: Option<u64>, current: u64, go: &dyn Fn(u64) -> Callback<MouseEvent>, testid: &dyn Fn(&str) -> Option<AttrValue>) -> Html {
    let Some(page) = link else {
        return html! { <li><span class="pagination-ellipsis">{ "…" }</span></li> };
    };
    let is_current = page == current;
    let number = page + 1;
    html! {
        <li>
            <button type="button" class={classes!("pagination-link", is_current.then_some("is-current"))}
                    style={MIN_TARGET_STYLE} aria-label={format!("Page {number}")}
                    aria-current={is_current.then_some("page")}
                    data-testid={testid(&format!("page-{number}"))} onclick={go(page)}>
                { number }
            </button>
        </li>
    }
}

/// Which page buttons to draw for `total_pages` pages with `current` shown
/// (both zero-based); `None` is an ellipsis. Every page up to seven; past
/// that the first, the last, the current one and its neighbours.
pub fn page_links(current: u64, total_pages: u64) -> Vec<Option<u64>> {
    if total_pages <= ALL_LINKS_UP_TO {
        return (0..total_pages).map(Some).collect();
    }
    let mut wanted = vec![0, total_pages - 1, current, current.saturating_sub(1), (current + 1).min(total_pages - 1)];
    wanted.sort_unstable();
    wanted.dedup();
    let mut links = Vec::with_capacity(wanted.len() + 2);
    let mut previous: Option<u64> = None;
    for page in wanted {
        if previous.is_some_and(|last_drawn| page > last_drawn + 1) {
            links.push(None);
        }
        links.push(Some(page));
        previous = Some(page);
    }
    links
}

/// Why Previous or Next waits: the page shown is at an end of the list.
fn page_edge(current: u64, last: u64) -> Option<&'static str> {
    match (current == 0, current >= last) {
        (true, true) => Some("This is the only page."),
        (true, false) => Some("This is the first page."),
        (false, true) => Some("This is the last page."),
        (false, false) => None,
    }
}
