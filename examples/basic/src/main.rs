#![recursion_limit = "1024"]

use console_error_panic_hook::set_once as set_panic_hook;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use ybc::Calendar;
use ybc::TileCtx::{Ancestor, Child, Parent};
use yew::prelude::*;

use ybc::ModalCloserContext;
use ybc::ModalCloserProvider;
use ybc::NavBurgerCloserState;

#[function_component(App)]
pub fn app() -> Html {
    let state = Rc::new(NavBurgerCloserState { total_clicks: 0 });
    let cb_date_changed = Callback::from(|date: String| {
        gloo_console::log!("Date changed: {}", date);
    });

    let cb_on_update = Callback::from(|tag: String| {
        gloo_console::log!("Tag updated: {}", tag);
    });
    let cb_on_remove = Callback::from(|tag: String| {
        gloo_console::log!("Tag removed: {}", tag);
    });
    let calendar_departure_date = html! {
       <Calendar id="my-calendar" date={"2030-01-01 01:02"} on_date_changed={cb_date_changed.clone()} class={vec!["input".to_string()]} />
    };
    let items: UseStateHandle<Vec<String>> = use_state(|| vec!["Apple".to_string(), "Banana".to_string(), "Cherry".to_string()]);

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
                                <ybc::Accordions id="accordions">
                                    <ybc::AccordionItem title={"Accordion 1"} open={true} id={"accordion1"}>
                                        <p>{"This is the content of the first accordion."}</p>
                                    </ybc::AccordionItem>
                                    <ybc::AccordionItem title={"Accordion 2"} open={false} id={"accordion2"}>
                                        <p>{"This is the content of the second accordion."}</p>
                                    </ybc::AccordionItem>
                                </ybc::Accordions>
                                <ModalCloserProvider id="id0">
                                       <MyModal1/>
                                </ModalCloserProvider>
                                <ModalCloserProvider id="id2">
                                    <MyModal2/>
                                </ModalCloserProvider>
                            </ybc::Tile>
                        </ybc::Tile>
                        <ybc::Tile>
                            <ybc::Tile>
                                <ybc::Field label={Some("Departure to Mars")} help={"Enter desired departure date"}>
                                      <ybc::Control>
                                          {calendar_departure_date}
                                      </ybc::Control>
                                </ybc::Field>
                            </ybc::Tile>
                       </ybc::Tile>
                       <ybc::Tile>
                            <ybc::Tile>
                                <ybc::Field label={Some("Select country code")} help={"Enter country code. data taken from 'https://restcountries.com/v3.1/name/'"}>
                                    <ybc::Control>
                                        <ybc::AutoComplete
                                          classes={classes!("is-primary")}
                                          current_selector={Rc::from("FR")}
                                          id={Rc::from("tags".to_string().as_str())}
                                          data_item_text={Rc::from("cca2")}
                                          data_item_value={Rc::from("cca2")}
                                          max_items={1}
                                          url_for_fetch={Rc::from("https://restcountries.com/v3.1/name/")}
                                          on_update={cb_on_update.clone()}
                                          on_remove={cb_on_remove.clone()}
                                          case_sensitive={false}
                                          placeholder={Rc::from("Enter country code")}
                                        />
                                    </ybc::Control>
                                </ybc::Field>
                                <ybc::Field label={Some("Tags as free text")} help={"Enter some data"}>
                                    <ybc::Control>
                                        <ybc::AutoComplete
                                          classes={classes!("is-primary")}
                                          current_selector={Rc::from("Apple")}
                                          id={Rc::from("tags2")}
                                          on_update={cb_on_update.clone()}
                                          on_remove={cb_on_remove.clone()}
                                        />
                                    </ybc::Control>
                                </ybc::Field>
                               <ybc::Field label={Some("Tags as fixed list")} help={"Enter some data"}>
                                    <ybc::Control>
                                        <ybc::AutoComplete
                                          classes={classes!("is-primary")}
                                          current_selector={Rc::from("Apple")}
                                          id={Rc::from("tags3".to_string().as_str())}
                                          on_update={cb_on_update.clone()}
                                          on_remove={cb_on_remove.clone()}
                                          items={(*items).clone()}
                                        />
                                    </ybc::Control>
                                </ybc::Field>
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

#[function_component(MyModal1)]
pub fn my_modal1() -> Html {
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

#[function_component(MyModal2)]
pub fn my_modal2() -> Html {
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
