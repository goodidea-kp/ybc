use crate::router::{Route, SwitchRoutes};
use crate::ui::CatalogList;
use std::rc::Rc;
use ybc::{FaIcon, NavBurgerCloserState, Navbar, NavbarItem};
use yew::prelude::*;
use yew_router::prelude::*;

#[component(App)]
pub fn app() -> Html {
    let state = Rc::new(NavBurgerCloserState { total_clicks: 0 });
    html! {
        <ContextProvider<Rc<NavBurgerCloserState>> context={state}>
            <BrowserRouter>
                <Navbar
                    navbrand={html!{
                        <NavbarItem>
                            <ybc::Title classes={classes!("has-text-white", "is-flex", "is-align-items-center")} size={ybc::HeaderSize::Is4}>
                                <FaIcon classes={classes!("mr-2")} icon_classes={classes!("fa-solid", "fa-cubes")} />
                                {"Trunk | Yew | YBC"}
                            </ybc::Title>
                        </NavbarItem>
                    }}
                    navstart={html!{
                        <>
                            <NavbarItem>
                                <Link<Route> to={Route::Home}>{"Home"}</Link<Route>>
                            </NavbarItem>
                            <NavbarItem>
                                <a href="https://crates.io/crates/ybc" target="_blank">{"YBC crate"}</a>
                            </NavbarItem>
                            <NavbarItem>
                                <a href="https://bulma.io/documentation/" target="_blank">{"Bulma docs"}</a>
                            </NavbarItem>
                        </>
                    }}
                >
                </Navbar>

                <SwitchRoutes/>
            </BrowserRouter>
        </ContextProvider<Rc<NavBurgerCloserState>>>
    }
}

#[component(HomePage)]
pub fn home_page() -> Html {
    html! {
        <>
            <section class="hero is-primary">
                <div class="hero-body">
                    <p class="title is-flex is-align-items-center">
                        <FaIcon classes={classes!("mr-2")} icon_classes={classes!("fa-solid", "fa-book-open")} />
                        { "YBC Catalog" }
                    </p>
                    <p class="subtitle">{ "All examples are live below. Open devtools to inspect HTML." }</p>
                </div>
            </section>

            <ybc::Section>
                <ybc::Container>
                    <CatalogList/>
                </ybc::Container>
            </ybc::Section>
        </>
    }
}
