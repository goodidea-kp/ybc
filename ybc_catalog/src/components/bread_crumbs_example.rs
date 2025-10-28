use yew::prelude::*;
use ybc::Alignment;
use ybc::components::breadcrumb::{Breadcrumb, BreadcrumbSeparator, BreadcrumbSize};
use yew_router::prelude::Link;
use crate::router::Route;

#[function_component(BreadCrumbsExamplePage)]
pub fn bread_crumbs_example_page() -> Html {
    let items = || html! {
        <>
            <li><Link<Route> to={Route::Home}>{ "Bulma" }</Link<Route>></li>
            <li><Link<Route> to={Route::Home}>{ "Docs" }</Link<Route>></li>
            <li><Link<Route> to={Route::Home}>{ "Components" }</Link<Route>></li>
            <li><Link<Route> to={Route::Modal}>{ "Modal" }</Link<Route>></li>
        </>
    };

    html! {
        <div class="section">
            <div class="container content">
                <h3>{ "Breadcrumb" }</h3>

                <h5>{ "Default" }</h5>
                <Breadcrumb>{ items() }</Breadcrumb>

                <h5>{ "Alignment" }</h5>
                <Breadcrumb alignment={Some(Alignment::Centered)}>{ items() }</Breadcrumb>
                <Breadcrumb alignment={Some(Alignment::Right)}>{ items() }</Breadcrumb>

                <h5>{ "Sizes" }</h5>
                <Breadcrumb size={Some(BreadcrumbSize::Small)}>{ items() }</Breadcrumb>
                <Breadcrumb size={Some(BreadcrumbSize::Medium)}>{ items() }</Breadcrumb>
                <Breadcrumb size={Some(BreadcrumbSize::Large)}>{ items() }</Breadcrumb>

                <h5>{ "Separators" }</h5>
                <Breadcrumb separator={Some(BreadcrumbSeparator::Arrow)}>{ items() }</Breadcrumb>
                <Breadcrumb separator={Some(BreadcrumbSeparator::Bullet)}>{ items() }</Breadcrumb>
                <Breadcrumb separator={Some(BreadcrumbSeparator::Dot)}>{ items() }</Breadcrumb>
                <Breadcrumb separator={Some(BreadcrumbSeparator::Succeeds)}>{ items() }</Breadcrumb>
            </div>
        </div>
    }
}
