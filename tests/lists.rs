//! The list controls, rendered to HTML on the host: what a keyboard and a
//! screen reader get from the sort headers, the pager and the toolbar
//! (Bastion BASTION-105, target sizes BASTION-104).

use ybc::{ListToolbar, ListToolbarProps, MIN_TARGET_STYLE, PAGE_SIZES, Pager, PagerProps, SortDirection, SortHeader, SortHeaderProps, page_links};
use yew::LocalServerRenderer;
use yew::prelude::*;

async fn render<C>(props: C::Properties) -> String
where
    C: BaseComponent,
{
    LocalServerRenderer::<C>::with_props(props).hydratable(false).render().await
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum Column {
    Name,
}

fn sort_header(direction: Option<SortDirection>) -> SortHeaderProps<Column> {
    SortHeaderProps {
        column: Column::Name,
        label: "Name".into(),
        direction,
        on_sort: Callback::noop(),
        testid: Some("domains-sort-name".into()),
        classes: Classes::new(),
    }
}

#[tokio::test]
async fn a_sort_header_is_a_th_with_a_real_button() {
    let html = render::<SortHeader<Column>>(sort_header(None)).await;

    assert!(html.starts_with("<th"), "{html}");
    assert!(html.contains(r#"<button type="button""#), "{html}");
    assert!(!html.contains("tabindex"), "the button keeps its natural tab stop: {html}");
    assert!(html.contains("<span>Name</span>"), "{html}");
    assert!(html.contains(r#"data-testid="domains-sort-name""#), "{html}");
}

#[tokio::test]
async fn aria_sort_says_whether_and_which_way_the_list_is_sorted() {
    let unsorted = render::<SortHeader<Column>>(sort_header(None)).await;
    let ascending = render::<SortHeader<Column>>(sort_header(Some(SortDirection::Ascending))).await;
    let descending = render::<SortHeader<Column>>(sort_header(Some(SortDirection::Descending))).await;

    assert!(unsorted.contains(r#"aria-sort="none""#) && unsorted.contains("fa-sort\""), "{unsorted}");
    assert!(
        ascending.contains(r#"aria-sort="ascending""#) && ascending.contains("fa-sort-up"),
        "{ascending}"
    );
    assert!(
        descending.contains(r#"aria-sort="descending""#) && descending.contains("fa-sort-down"),
        "{descending}"
    );
}

#[tokio::test]
async fn the_sort_icon_is_hidden_and_the_button_is_at_least_24px() {
    let html = render::<SortHeader<Column>>(sort_header(Some(SortDirection::Ascending))).await;

    assert!(html.contains(r#"aria-hidden="true""#), "{html}");
    assert!(html.contains(&format!(r#"style="{MIN_TARGET_STYLE}""#)), "{html}");
    assert_eq!(MIN_TARGET_STYLE, "min-width: 24px; min-height: 24px");
}

#[test]
fn a_second_click_reverses_the_direction() {
    assert_eq!(SortDirection::Ascending.reversed(), SortDirection::Descending);
    assert_eq!(SortDirection::Descending.reversed(), SortDirection::Ascending);
}

fn pager(current: u64, last: u64, testid: Option<&str>) -> PagerProps {
    PagerProps {
        current,
        last,
        on_page: Callback::noop(),
        aria_label: "Domain pages".into(),
        testid: testid.map(AttrValue::from),
        classes: Classes::new(),
    }
}

#[tokio::test]
async fn a_single_page_draws_no_pager() {
    let html = render::<Pager>(pager(0, 0, Some("domains"))).await;

    assert_eq!(html, "", "{html}");
}

#[tokio::test]
async fn the_pager_is_a_named_navigation_landmark_of_buttons() {
    let html = render::<Pager>(pager(1, 2, Some("domains"))).await;

    assert!(html.starts_with("<nav ") && html.contains(r#"class="pagination is-small""#), "{html}");
    assert!(html.contains(r#"role="navigation" aria-label="Domain pages""#), "{html}");
    assert!(html.contains(r#"data-testid="domains-pagination""#), "{html}");
    assert_eq!(
        html.matches(r#"<button type="button""#).count(),
        5,
        "Previous, Next and three pages: {html}"
    );
    assert!(!html.contains("<a "), "{html}");
}

#[tokio::test]
async fn only_the_current_page_has_aria_current() {
    let html = render::<Pager>(pager(1, 2, Some("domains"))).await;

    assert_eq!(html.matches(r#"aria-current="page""#).count(), 1, "{html}");
    let current = html.find(r#"aria-current="page""#).expect("a current page");
    let page_two = html.find(r#"data-testid="domains-page-2""#).expect("page 2");
    let page_three = html.find(r#"data-testid="domains-page-3""#).expect("page 3");
    assert!(current < page_two && page_two < page_three, "page 2 is the current one: {html}");
    assert!(html.contains(r#"aria-label="Page 2""#), "{html}");
}

#[tokio::test]
async fn every_pager_button_is_at_least_24px() {
    let html = render::<Pager>(pager(1, 2, Some("domains"))).await;

    assert_eq!(html.matches(MIN_TARGET_STYLE).count(), 5, "{html}");
}

#[tokio::test]
async fn at_the_first_page_previous_stays_focusable_and_says_why() {
    let html = render::<Pager>(pager(0, 4, Some("domains"))).await;

    assert!(
        html.contains(r#"id="domains-page-edge""#) && html.contains("This is the first page."),
        "{html}"
    );
    assert!(html.contains(r#"aria-describedby="domains-page-edge""#), "{html}");
    assert_eq!(html.matches(r#"aria-disabled="true""#).count(), 1, "only Previous waits: {html}");
    assert!(!html.contains(" disabled"), "no native disabled, which leaves the tab order: {html}");
    assert!(
        html.contains(&format!("{MIN_TARGET_STYLE}; opacity: 0.55")),
        "the target size survives the disabled look: {html}"
    );
}

#[tokio::test]
async fn at_the_last_page_next_says_why() {
    let html = render::<Pager>(pager(4, 4, Some("domains"))).await;

    assert!(html.contains("This is the last page."), "{html}");
    assert_eq!(html.matches(r#"aria-disabled="true""#).count(), 1, "{html}");
}

#[tokio::test]
async fn in_the_middle_nothing_waits_and_there_is_no_edge_line() {
    let html = render::<Pager>(pager(2, 4, Some("domains"))).await;

    assert!(!html.contains("aria-disabled"), "{html}");
    assert!(!html.contains("page-edge"), "{html}");
}

#[tokio::test]
async fn without_a_testid_the_pager_has_generated_ids_and_no_test_ids() {
    let html = render::<Pager>(pager(0, 3, None)).await;

    assert!(!html.contains("data-testid"), "{html}");
    assert!(html.contains(r#"id="ybc-pager-"#), "{html}");
}

#[test]
fn up_to_seven_pages_every_page_gets_a_link() {
    assert_eq!(page_links(0, 1), vec![Some(0)]);
    assert_eq!(page_links(3, 7), (0..7).map(Some).collect::<Vec<_>>());
}

#[test]
fn past_seven_pages_the_ends_and_the_neighbours_remain_with_ellipses() {
    assert_eq!(page_links(5, 20), vec![Some(0), None, Some(4), Some(5), Some(6), None, Some(19)]);
    assert_eq!(page_links(0, 20), vec![Some(0), Some(1), None, Some(19)]);
    assert_eq!(page_links(19, 20), vec![Some(0), None, Some(18), Some(19)]);
    assert_eq!(page_links(1, 20), vec![Some(0), Some(1), Some(2), None, Some(19)]);
}

fn toolbar(filter: &str, range_text: Option<&str>, testid: Option<&str>) -> ListToolbarProps {
    ListToolbarProps {
        filter: AttrValue::from(filter.to_string()),
        on_filter: Callback::noop(),
        on_clear: None,
        filter_label: "Filter domains by name".into(),
        placeholder: Some("Filter by name".into()),
        size: 50,
        sizes: &PAGE_SIZES,
        on_size: Callback::noop(),
        range_text: range_text.map(|text| AttrValue::from(text.to_string())),
        testid: testid.map(AttrValue::from),
    }
}

#[tokio::test]
async fn the_filter_box_is_a_named_search_input() {
    let html = render::<ListToolbar>(toolbar("", Some("1–25 of 112 domains"), Some("domains"))).await;

    assert!(html.contains(r#"type="search""#), "{html}");
    assert!(html.contains(r#"aria-label="Filter domains by name""#), "{html}");
    assert!(html.contains(r#"placeholder="Filter by name""#), "{html}");
    assert!(html.contains(r#"data-testid="domains-filter-name""#), "{html}");
}

#[tokio::test]
async fn clear_appears_only_while_the_box_has_text() {
    let empty = render::<ListToolbar>(toolbar("", Some("x"), Some("domains"))).await;
    let typed = render::<ListToolbar>(toolbar("sca", Some("x"), Some("domains"))).await;

    assert!(!empty.contains("filter-clear"), "{empty}");
    assert!(typed.contains(r#"data-testid="domains-filter-clear""#), "{typed}");
    assert!(typed.contains(r#"aria-label="Clear the filter""#), "{typed}");
    assert!(typed.contains(MIN_TARGET_STYLE), "{typed}");
}

#[tokio::test]
async fn the_range_is_a_polite_live_region() {
    let html = render::<ListToolbar>(toolbar("", Some("1–25 of 112 domains"), Some("domains"))).await;

    assert!(html.contains(r#"data-testid="domains-range" aria-live="polite""#), "{html}");
    assert!(html.contains("1–25 of 112 domains"), "{html}");
    assert!(!html.contains("aria-busy"), "{html}");
}

#[tokio::test]
async fn while_loading_the_range_is_a_busy_skeleton_that_still_says_loading() {
    let html = render::<ListToolbar>(toolbar("", None, Some("domains"))).await;

    assert!(html.contains(r#"aria-busy="true""#), "{html}");
    assert!(html.contains(r#"<span class="is-skeleton">Loading…</span>"#), "{html}");
}

#[tokio::test]
async fn the_page_size_select_is_labelled_and_marks_the_current_size() {
    let html = render::<ListToolbar>(toolbar("", Some("x"), Some("domains"))).await;

    assert!(
        html.contains(r#"<label for="domains-page-size""#) && html.contains(">Rows per page</label>"),
        "{html}"
    );
    assert!(html.contains(r#"<select id="domains-page-size""#), "{html}");
    assert_eq!(html.matches("<option").count(), PAGE_SIZES.len(), "{html}");
    assert_eq!(html.matches(r#"selected="selected""#).count(), 1, "{html}");
    let selected = html.find(r#"selected="selected""#).expect("one selected option");
    let fifty = html.find(r#"value="50""#).expect("a 50 option");
    let hundred = html.find(r#"value="100""#).expect("a 100 option");
    assert!(fifty < selected && selected < hundred, "50 is selected: {html}");
}

#[tokio::test]
async fn without_a_testid_the_select_still_has_a_label_for_its_generated_id() {
    let html = render::<ListToolbar>(toolbar("", Some("x"), None)).await;

    assert!(!html.contains("data-testid"), "{html}");
    let id_start = html.find(r#"<select id=""#).expect("a select") + r#"<select id=""#.len();
    let id = &html[id_start..id_start + html[id_start..].find('"').expect("closing quote")];
    assert!(html.contains(&format!(r#"for="{id}""#)), "{html}");
}
