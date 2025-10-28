use yew::prelude::*;
use yew_router::prelude::Link;
use ybc::{Alignment, Size};
use ybc::components::pagination::{Pagination, PaginationItem, PaginationItemType, PaginationEllipsis};
use crate::router::Route;

#[function_component(PaginationExamplePage)]
pub fn pagination_example_page() -> Html {
    let prev_link = |to: Route| html! { <Link<Route> to={to} classes={classes!("pagination-previous")}>{ "Previous" }</Link<Route>> };
    let next_link = |to: Route| html! { <Link<Route> to={to} classes={classes!("pagination-next")}>{ "Next page" }</Link<Route>> };

    html! {
        <ybc::Section>
            <ybc::Container classes={classes!("content")}>
                <h3>{ "Pagination" }</h3>

                <h5>{ "Default" }</h5>
                <Pagination previous={prev_link(Route::Home)} next={next_link(Route::Home)}>
                    <PaginationItem item_type={PaginationItemType::Link} label={"Go to page 1"} current=true>{ "1" }</PaginationItem>
                    <PaginationItem item_type={PaginationItemType::Link} label={"Go to page 2"}>{ "2" }</PaginationItem>
                    <PaginationItem item_type={PaginationItemType::Link} label={"Go to page 3"}>{ "3" }</PaginationItem>
                    <PaginationEllipsis />
                    <PaginationItem item_type={PaginationItemType::Link} label={"Go to page 8"}>{ "8" }</PaginationItem>
                    <PaginationItem item_type={PaginationItemType::Link} label={"Go to page 9"}>{ "9" }</PaginationItem>
                </Pagination>

                <h5>{ "Centered, rounded, small" }</h5>
                <Pagination alignment={Some(Alignment::Centered)} size={Some(Size::Small)} rounded=true
                            previous={prev_link(Route::Breadcrumb)} next={next_link(Route::Breadcrumb)}>
                    <PaginationItem item_type={PaginationItemType::Link} label={"Go to page 1"}>{ "1" }</PaginationItem>
                    <PaginationItem item_type={PaginationItemType::Link} label={"Go to page 2"} current=true>{ "2" }</PaginationItem>
                    <PaginationItem item_type={PaginationItemType::Link} label={"Go to page 3"}>{ "3" }</PaginationItem>
                    <PaginationEllipsis />
                    <PaginationItem item_type={PaginationItemType::Link} label={"Go to page 6"}>{ "6" }</PaginationItem>
                </Pagination>

                <h5>{ "Right, large" }</h5>
                <Pagination alignment={Some(Alignment::Right)} size={Some(Size::Large)}
                            previous={prev_link(Route::Panel)} next={next_link(Route::Panel)}>
                    <PaginationItem item_type={PaginationItemType::Link} label={"Go to page 9"}>{ "9" }</PaginationItem>
                    <PaginationItem item_type={PaginationItemType::Link} label={"Go to page 10"} current=true>{ "10" }</PaginationItem>
                    <PaginationItem item_type={PaginationItemType::Link} label={"Go to page 11"}>{ "11" }</PaginationItem>
                </Pagination>
            </ybc::Container>
        </ybc::Section>
    }
}
