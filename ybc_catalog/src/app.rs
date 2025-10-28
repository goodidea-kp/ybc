use crate::router::{Route, SwitchRoutes};
use crate::ui::CatalogList;
use std::rc::Rc;
use ybc::{NavBurgerCloserState, Navbar, NavbarItem};
use yew::prelude::*;
use yew_router::prelude::*;

#[function_component(App)]
pub fn app() -> Html {
    let state = Rc::new(NavBurgerCloserState { total_clicks: 0 });
    html! {
        <ContextProvider<Rc<NavBurgerCloserState>> context={state}>
            <BrowserRouter>
                <Navbar
                    navbrand={html!{
                        <NavbarItem>
                            <ybc::Title classes={classes!("has-text-white")} size={ybc::HeaderSize::Is4}>
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

#[function_component(HomePage)]
pub fn home_page() -> Html {
    html! {
        <>
            <section class="hero is-primary">
                <div class="hero-body">
                    <p class="title">{ "YBC Catalog" }</p>
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
