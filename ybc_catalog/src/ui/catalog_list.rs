use crate::router::Route;
use yew::prelude::*;
use yew_router::prelude::*;

#[function_component(CatalogList)]
pub fn catalog_list() -> Html {
    let items: Vec<(&'static str, Route, &'static str)> = vec![
        ("Button", Route::Button, "Primary/Link/Button groups, click handlers"),
        ("Navbar", Route::Navbar, "Brand/start/end slots; burger toggle handled in Yew"),
        ("Card", Route::Card, "Header, image, content, footer"),
        ("Form Controls", Route::Form, "Input, Textarea, Select, Checkbox, Radio, Calendar"),
        ("Columns", Route::Columns, "Responsive layout with Columns/Column"),
        ("Table", Route::Table, "Striped, hoverable, bordered"),
        ("Modal", Route::Modal, "Controlled active state; close callbacks"),
        ("Tabs", Route::Tabs, "Controlled active index; content switch"),
        ("Tag & Notification", Route::TagNotification, "Color tags and notification types"),
        ("Image & Icon", Route::ImageIcon, "Bulma Image sizes and Font Awesome icons"),
        ("Breadcrumb", Route::Breadcrumb, "Alignment, size, and separator variations"),
        ("Message", Route::Message, "Header and body; colors and sizes"),
        ("Panel", Route::Panel, "Compact controls with tabs and blocks"),
        ("Pagination", Route::Pagination, "Sizes, alignment, rounded, and ellipsis"),
        ("Accordion", Route::Accordion, "Accessible details/summary sections"),
        ("Autocomplete", Route::Autocomplete, "Tags input with static source"),
        ("Calendar", Route::Calendar, "Date/time picker powered by bulma-calendar"),
    ];

    html! {
      <div class="content">
        <h3>{"Components"}</h3>
        <ul>
          { for items.into_iter().map(|(name, route, desc)| html!{
              <li>
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
