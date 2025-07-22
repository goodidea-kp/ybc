#![recursion_limit = "1024"]

use console_error_panic_hook::set_once as set_panic_hook;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use ybc::TileCtx::{Ancestor, Child, Parent};
use yew::prelude::*;

use ybc::NavBurgerCloserState;

#[function_component(App)]
pub fn app() -> Html {
    let state = Rc::new(NavBurgerCloserState { total_clicks: 0 });
    html! {
        <>
        <ContextProvider<Rc<NavBurgerCloserState>> context={state}>
        <ybc::Navbar
            classes={classes!("is-success")}
            padded=true
            navbrand={html!{
                <ybc::NavbarItem>
                    <ybc::Title classes={classes!("has-text-white")} size={ybc::HeaderSize::Is4}>{"Trunk | Yew | YBC"}</ybc::Title>
                </ybc::NavbarItem>
            }}
            navstart={html!{}}
            navend={html!{
                <>
                <ybc::NavbarItem>
                    <ybc::ButtonAnchor classes={classes!("is-black", "is-outlined")} rel={String::from("noopener noreferrer")} target={String::from("_blank")} href="https://github.com/thedodd/trunk">
                        {"Trunk"}
                    </ybc::ButtonAnchor>
                </ybc::NavbarItem>
                <ybc::NavbarItem>
                    <ybc::ButtonAnchor classes={classes!("is-black", "is-outlined")} rel={String::from("noopener noreferrer")} target={String::from("_blank")} href="https://yew.rs">
                        {"Yew"}
                    </ybc::ButtonAnchor>
                </ybc::NavbarItem>
                <ybc::NavbarItem>
                    <ybc::ButtonAnchor classes={classes!("is-black", "is-outlined")} rel={String::from("noopener noreferrer")} target={String::from("_blank")} href="https://github.com/goodidea-kp/ybc">
                        {"YBC"}
                    </ybc::ButtonAnchor>
                </ybc::NavbarItem>
                </>
            }}
        />
        </ContextProvider<Rc<NavBurgerCloserState>>>

        <ybc::Hero
            classes={classes!("is-light")}
            size={ybc::HeroSize::FullheightWithNavbar}
            body={html!{
                <ybc::Container classes={classes!("is-centered")}>
                <ybc::Tile ctx={Ancestor}>
                    <ybc::Tile ctx={Parent} size={ybc::TileSize::Twelve}>
                        <ybc::Tile ctx={Parent}>
                            <ybc::Tile ctx={Child} classes={classes!("notification", "is-success")}>
                                <ybc::Subtitle size={ybc::HeaderSize::Is3} classes={classes!("has-text-white")}>{"Trunk"}</ybc::Subtitle>
                                <p>{"Trunk is a WASM web application bundler for Rust."}</p>
                            </ybc::Tile>
                        </ybc::Tile>
                        <ybc::Tile ctx={Parent}>
                            <ybc::Tile ctx={Child} classes={classes!("notification", "is-success")}>
                                <ybc::Icon size={ybc::Size::Large} classes={classes!("is-pulled-right")}><img src="yew.svg"/></ybc::Icon>
                                <ybc::Subtitle size={ybc::HeaderSize::Is3} classes={classes!("has-text-white")}>
                                    {"Yew"}
                                </ybc::Subtitle>
                                <p>{"Yew is a modern Rust framework for creating multi-threaded front-end web apps with WebAssembly."}</p>
                            </ybc::Tile>
                        </ybc::Tile>
                        <ybc::Tile ctx={Parent}>
                            <ybc::Tile ctx={Child} classes={classes!("notification", "is-success")}>
                                <ybc::Subtitle size={ybc::HeaderSize::Is3} classes={classes!("has-text-white")}>{"YBC"}</ybc::Subtitle>
                                <p>{"A Yew component library based on the Bulma CSS framework."}</p>
                                <ModalCloserProvider id="id0">
                                       <MyModal1/>
                                </ModalCloserProvider>
                                <ModalCloserProvider id="id2">
                                    <MyModal2/>
                                </ModalCloserProvider>
                            </ybc::Tile>
                        </ybc::Tile>
                    </ybc::Tile>
                </ybc::Tile>
                </ybc::Container>
            }}>
        </ybc::Hero>
        </>
    }
}

#[wasm_bindgen(inline_js = "export function snippetTest() { console.log('Hello from JS FFI!'); }")]
extern "C" {
    fn snippetTest();
}

fn main() {
    set_panic_hook();
    snippetTest();

    // Show off some feature flag enabling patterns.
    #[cfg(feature = "demo-abc")]
    {
        gloo_console::log!("feature `demo-abc` enabled");
    }
    #[cfg(feature = "demo-xyz")]
    {
        gloo_console::log!("feature `demo-xyz` enabled");
    }

    yew::Renderer::<App>::new().render();
}

use ybc::ModalCloserContext;
use ybc::ModalCloserProvider;

#[function_component]
pub fn MyModal1() -> Html {
    let msg_ctx = use_context::<ModalCloserContext>().unwrap();
    let onclick = { Callback::from(move |e: MouseEvent| msg_ctx.dispatch("id0-close".to_string().parse().unwrap())) };
    let on_click_cb = Callback::from(move |e: AttrValue| {
        gloo_console::log!("Button clicked!");
    });
    html! {
            <ybc::ModalCard
                classes={classes!("")}
                id={String::from("id0")}
                title={"Modal"}
                trigger={html!{
                    <ybc::Button classes={classes!("is-success")}>
                        {"Open Modal"}
                    </ybc::Button>
                }}
         // on_clicked={on_click_cb}
                body={
                    html!{
                    <ybc::Content>
                        <p classes="has-text-green">{"This is the body of the modal."}</p>
                    </ybc::Content>
                    }
                }
                footer={html!(
                      <>
                        <ybc::Button classes={classes!("is-success")}>
                            {"Save changes"}
                        </ybc::Button>
                        <ybc::Button classes={classes!("is-danger")} onclick={onclick}>
                            {"Close"}
                        </ybc::Button>
                    </>
                )}
            />
    }
}

#[function_component]
pub fn MyModal2() -> Html {
    let msg_ctx = use_context::<ModalCloserContext>().unwrap();
    let onclick = { Callback::from(move |e: MouseEvent| msg_ctx.dispatch("id2-close".to_string().parse().unwrap())) };
    let msg_ctx2 = use_context::<ModalCloserContext>().unwrap();
    let onsave = { Callback::from(move |e: MouseEvent| msg_ctx2.dispatch("id2-close".to_string().parse().unwrap())) };
    let on_click_cb = Callback::from(move |e: AttrValue| {
        gloo_console::log!("Button clicked!");
    });
    html! {
            <ybc::ModalCard
                classes={classes!("")}
                id={String::from("id2")}
                title={"Modal2"}
                trigger={html!{
                    <ybc::Button classes={classes!("is-danger")}>
                        {"Open Modal"}
                    </ybc::Button>
                }}
                // on_clicked={on_click_cb}
                body={
                    html!{
                    <ybc::Content>
                        <p classes="has-text-green">{"This is the body of the modal2."}</p>
                    </ybc::Content>
                    }
                }
                footer={html!(
                      <>
                        <ybc::Button classes={classes!("is-success")} onclick={onsave}>
                            {"Save changes"}
                        </ybc::Button>
                        <ybc::Button classes={classes!("is-danger")} onclick={onclick}>
                            {"Close"}
                        </ybc::Button>
                    </>
                )}
            />
    }
}
