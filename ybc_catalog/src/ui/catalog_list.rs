use crate::router::Route;
use yew::prelude::*;
use yew_router::prelude::*;

#[component(CatalogList)]
pub fn catalog_list() -> Html {
    let items: Vec<(&'static str, (&'static str, &'static str), Route, &'static str)> = vec![
        (
            "Button",
            ("fa-solid", "fa-hand-pointer"),
            Route::Button,
            "Primary/Link/Button groups, click handlers",
        ),
        (
            "Navbar",
            ("fa-solid", "fa-bars"),
            Route::Navbar,
            "Brand/start/end slots; burger toggle handled in Yew",
        ),
        ("Card", ("fa-solid", "fa-id-card"), Route::Card, "Header, image, content, footer"),
        (
            "Form Controls",
            ("fa-solid", "fa-file-lines"),
            Route::Form,
            "Input, Textarea, Select, Checkbox, Radio, Calendar",
        ),
        (
            "Columns",
            ("fa-solid", "fa-table-columns"),
            Route::Columns,
            "Responsive layout with Columns/Column",
        ),
        ("Table", ("fa-solid", "fa-table"), Route::Table, "Striped, hoverable, bordered"),
        (
            "Modal",
            ("fa-solid", "fa-window-maximize"),
            Route::Modal,
            "Dialog-backed modals with controller/context open-close",
        ),
        (
            "Tabs",
            ("fa-solid", "fa-folder-tree"),
            Route::Tabs,
            "Controlled active index; content switch",
        ),
        (
            "Tag & Notification",
            ("fa-solid", "fa-tags"),
            Route::TagNotification,
            "Color tags and notification types",
        ),
        (
            "Image & Icon",
            ("fa-solid", "fa-icons"),
            Route::ImageIcon,
            "Bulma Image sizes and Font Awesome icons",
        ),
        (
            "Breadcrumb",
            ("fa-solid", "fa-route"),
            Route::Breadcrumb,
            "Alignment, size, and separator variations",
        ),
        (
            "Message",
            ("fa-solid", "fa-envelope-open-text"),
            Route::Message,
            "Header and body; colors and sizes",
        ),
        (
            "Panel",
            ("fa-solid", "fa-table-list"),
            Route::Panel,
            "Compact controls with tabs and blocks",
        ),
        (
            "Pagination",
            ("fa-solid", "fa-book-open"),
            Route::Pagination,
            "Sizes, alignment, rounded, and ellipsis",
        ),
        (
            "Accordion",
            ("fa-solid", "fa-list"),
            Route::Accordion,
            "Accessible details/summary sections",
        ),
        (
            "Autocomplete",
            ("fa-solid", "fa-magnifying-glass"),
            Route::Autocomplete,
            "Tags input with static source",
        ),
        (
            "Calendar",
            ("fa-solid", "fa-calendar-days"),
            Route::Calendar,
            "Date/time picker powered by bulma-calendar",
        ),
    ];

    html! {
      <div class="content">
        <h3>{"Components"}</h3>
        <ul>
          { for items.into_iter().map(|(name, (icon_a, icon_b), route, desc)| html!{
              <li>
                <ybc::FaIcon classes={classes!("mr-1", "has-text-link")} icon_classes={classes!(icon_a, icon_b)} />
                <Link<Route> to={route.clone()}>{name}</Link<Route>>
                {format!(" — {}", desc)}
              </li>
          }) }
        </ul>
        <p class="is-size-7 has-text-grey">
          {"See chatgpt_catalog.yaml for a machine-readable index (kept in sync with routes)."}
        </p>
      </div>
    }
}
