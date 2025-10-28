use yew::prelude::*;
use yew_router::prelude::*;

use crate::app::HomePage;
use crate::components::button_example::ButtonExamplePage;
use crate::components::{CardExamplePage, ColumnsExamplePage, FormExamplePage, ImageIconExamplePage, ModalExamplePage, NavbarExamplePage, TableExamplePage, TabsExamplePage, TagNotificationExamplePage,
                        BreadCrumbsExamplePage, MessageExamplePage, PanelExamplePage, PaginationExamplePage, AccordionExamplePage, AutocompleteExamplePage};

#[derive(Routable, Clone, PartialEq, Debug)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/component/button")]
    Button,
    #[at("/component/navbar")]
    Navbar,
    #[at("/component/card")]
    Card,
    #[at("/component/form")]
    Form,
    #[at("/component/tag-notification")]
    TagNotification,
    #[at("/component/tabs_example")]
    Tabs,
    #[at("/component/image-icon")]
    ImageIcon,
    #[at("/component/modal")]
    Modal,
    #[at("/component/table")]
    Table,
    #[at("/component/columns")]
    Columns,
    #[at("/component/breadcrumb")]
    Breadcrumb,
    #[at("/component/message")]
    Message,
    #[at("/component/panel")]
    Panel,
    #[at("/component/pagination")]
    Pagination,
    #[at("/component/accordion")]
    Accordion,
    #[at("/component/autocomplete")]
    Autocomplete,
    #[not_found]
    #[at("/404")]
    NotFound,
}

#[function_component(SwitchRoutes)]
pub fn switch_routes() -> Html {
    html! {
           <Switch<Route> render={move |route| {
            match route {
                Route::Home => html! { <HomePage/> },
                Route::Button => html! { <ButtonExamplePage/> },
                Route::Navbar => html! { <NavbarExamplePage/> },
                Route::Card => html! { <CardExamplePage/> },
                Route::Form => html! { <FormExamplePage/> },
                Route::TagNotification => html! { <TagNotificationExamplePage/> },
                Route::Tabs => html! { <TabsExamplePage/> },
                Route::Modal => html! { <ModalExamplePage/> },
                Route::Columns => html! { <ColumnsExamplePage/> },
                Route::Table => html! { <TableExamplePage/> },
                Route::ImageIcon => html! { <ImageIconExamplePage/> },
                Route::Breadcrumb => html! { <BreadCrumbsExamplePage/> },
                Route::Message => html! { <MessageExamplePage/> },
                Route::Panel => html! { <PanelExamplePage/> },
                Route::Pagination => html! { <PaginationExamplePage/> },
                Route::Accordion => html! { <AccordionExamplePage/> },
                Route::Autocomplete => html! { <AutocompleteExamplePage/> },
                Route::NotFound => html! { <div class="section"><div class="container"><h1 class="title">{"404"}</h1></div></div> },
            }
        }}/>
    }
}
