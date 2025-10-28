use yew::prelude::*;
use yew_router::prelude::Link;
use ybc::components::panel::{Panel, PanelTabs, PanelBlock};
use crate::router::Route;

#[function_component(PanelExamplePage)]
pub fn panel_example_page() -> Html {
    let on_click_refresh = Callback::from(|_| web_sys::console::log_1(&"refresh clicked".into()));

    html! {
        <ybc::Section>
            <ybc::Container classes={classes!("content")}>
                <h3>{ "Panel" }</h3>

                <Panel heading={html!{ "Repositories" }}>
                    <PanelTabs>
                        <Link<Route> to={Route::Panel} classes={classes!("is-active")}>{ "All" }</Link<Route>>
                        <Link<Route> to={Route::Navbar}>{ "Navbar" }</Link<Route>>
                        <Link<Route> to={Route::Message}>{ "Message" }</Link<Route>>
                    </PanelTabs>

                    <PanelBlock>
                        <p class="control has-icons-left">
                            <input class="input" type="text" placeholder="Search"/>
                            <span class="icon is-left"><i class="fas fa-search" aria-hidden="true"></i></span>
                        </p>
                    </PanelBlock>

                    <PanelBlock active={true}>
                        <Link<Route> to={Route::Message}>{ "Message example" }</Link<Route>>
                    </PanelBlock>

                    <PanelBlock>
                        <Link<Route> to={Route::Modal}>{ "Modal example" }</Link<Route>>
                    </PanelBlock>

                    <PanelBlock tag={"button".to_string()} onclick={on_click_refresh}>
                        { "Refresh" }
                    </PanelBlock>
                </Panel>
            </ybc::Container>
        </ybc::Section>
    }
}
