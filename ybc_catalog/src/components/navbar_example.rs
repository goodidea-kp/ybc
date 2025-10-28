use crate::router::Route;
use ybc::*;
use yew::prelude::*;
use yew_router::prelude::*;

#[function_component(NavbarExamplePage)]
pub fn navbar_example_page() -> Html {
    html! {
      <div class="section">
        <div class="container">
         <Navbar
                    navbrand={html!{
                        <NavbarItem>
                            <Title classes={classes!("has-text-white")} size={HeaderSize::Is4}>
                                {"Navbar Example"}
                            </Title>
                        </NavbarItem>
                    }}
                    navstart={html!{
                            <NavbarItem>
                                <Link<Route> to={Route::Home}>{"Home"}</Link<Route>>
                            </NavbarItem>
                    }}
                    navend={html!{
                            <NavbarItem>
                                <Link<Route> to={Route::Home}>{"Back to Home"}</Link<Route>>
                            </NavbarItem>
                    }
                    }
                >
                </Navbar>
          <p class="content">{"Burger toggle is handled by Yew state, no extra JS."}</p>
        </div>
      </div>
    }
}
