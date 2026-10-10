//! What a paged, sortable list shares: the column header that sorts it
//! ([`SortHeader`]), the page links under it ([`Pager`]) and the filter, range
//! and page-size bar above it ([`ListToolbar`]).
//!
//! The components hold no list state. The page owns the query (page, size,
//! filter, sort) and the server's answer; these only draw them and report what
//! the operator asked for. Every control is a real `<button>`, `<input>` or
//! `<select>`, reachable by keyboard, and every button is at least 24×24 CSS
//! pixels (WCAG 2.2, 2.5.8).

use std::sync::atomic::{AtomicUsize, Ordering};

mod pager;
mod sort_header;
mod toolbar;

pub use pager::{Pager, PagerProps, page_links};
pub use sort_header::{SortDirection, SortHeader, SortHeaderProps};
pub use toolbar::{ListToolbar, ListToolbarProps, PAGE_SIZES};

/// The minimum target size every list button gets, WCAG 2.2 success
/// criterion 2.5.8: 24×24 CSS pixels whatever Bulma modifier shrinks it.
pub const MIN_TARGET_STYLE: &str = "min-width: 24px; min-height: 24px";

static LIST_AUTO_ID: AtomicUsize = AtomicUsize::new(1);

/// A page-unique id for a control whose caller gave no `testid` to derive one from.
fn next_list_id(prefix: &str) -> String {
    format!("{prefix}-{}", LIST_AUTO_ID.fetch_add(1, Ordering::Relaxed))
}
