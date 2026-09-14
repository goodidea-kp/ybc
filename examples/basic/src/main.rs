#![recursion_limit = "1024"]

use console_error_panic_hook::set_once as set_panic_hook;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use ybc::Calendar;
use ybc::TileCtx::{Ancestor, Child, Parent};
use yew::prelude::*;

use ybc::NavBurgerCloserState;
use ybc::PieSegment;
use yew_router::prelude::*;

#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/")]
    Home,
    #[at("/language/:key")]
    Language { key: String },
    #[not_found]
    #[at("/404")]
    NotFound,
}

#[component(App)]
pub fn app() -> Html {
    html! {
        <BrowserRouter>
            <Switch<Route> render={switch} />
        </BrowserRouter>
    }
}

fn switch(route: Route) -> Html {
    match route {
        Route::Home | Route::NotFound => html! { <Home /> },
        Route::Language { key } => html! { <LanguagePage language_key={key} /> },
    }
}

/// A language in the demo codebase: key, name, lines of code, and a blurb for its page.
struct Language {
    key: &'static str,
    name: &'static str,
    lines_of_code: f64,
    blurb: &'static str,
}

const LANGUAGES: [Language; 8] = [
    Language {
        key: "rust",
        name: "Rust",
        lines_of_code: 48_210.0,
        blurb: "Core services and the WASM front end.",
    },
    Language {
        key: "typescript",
        name: "TypeScript",
        lines_of_code: 21_930.0,
        blurb: "Browser glue and the design-system playground.",
    },
    Language {
        key: "go",
        name: "Go",
        lines_of_code: 12_400.0,
        blurb: "Deployment tooling and the CLI.",
    },
    Language {
        key: "python",
        name: "Python",
        lines_of_code: 7_850.0,
        blurb: "Data pipelines and one-off analysis.",
    },
    Language {
        key: "sql",
        name: "SQL",
        lines_of_code: 3_120.0,
        blurb: "Migrations and reporting views.",
    },
    Language {
        key: "shell",
        name: "Shell",
        lines_of_code: 1_480.0,
        blurb: "CI scripts.",
    },
    Language {
        key: "yaml",
        name: "YAML",
        lines_of_code: 960.0,
        blurb: "Pipeline and Kubernetes manifests.",
    },
    Language {
        key: "css",
        name: "CSS",
        lines_of_code: 410.0,
        blurb: "Overrides on top of Bulma.",
    },
];

fn language_segments() -> Vec<PieSegment> {
    LANGUAGES
        .iter()
        .map(|language| PieSegment::new(language.key, language.name, language.lines_of_code).with_href(format!("/language/{}", language.key)))
        .collect()
}

#[derive(Clone, Properties, PartialEq)]
struct LanguagePageProps {
    language_key: String,
}

/// The detail page a chart segment routes to.
#[component(LanguagePage)]
fn language_page(props: &LanguagePageProps) -> Html {
    let language = LANGUAGES.iter().find(|language| language.key == props.language_key);
    let total: f64 = LANGUAGES.iter().map(|language| language.lines_of_code).sum();
    let body = match language {
        Some(language) => {
            let share = format!(
                "{:.0} lines of code, {:.0}% of the codebase",
                language.lines_of_code,
                language.lines_of_code / total * 100.0
            );
            html! {
                <>
                    <ybc::Title><>{language.name}</></ybc::Title>
                    <ybc::Subtitle><>{share}</></ybc::Subtitle>
                    <p>{language.blurb}</p>
                </>
            }
        }
        None => html! {
            <>
                <ybc::Title>{"Unknown language"}</ybc::Title>
                <p>{format!("Nothing is recorded for '{}'.", props.language_key)}</p>
            </>
        },
    };
    html! {
        <ybc::Section>
            <ybc::Container>
                <ybc::Box>
                    {body}
                    <Link<Route> to={Route::Home} classes={classes!("button", "is-link", "is-light", "mt-4")}>{"Back to the chart"}</Link<Route>>
                </ybc::Box>
            </ybc::Container>
        </ybc::Section>
    }
}

#[component(Home)]
pub fn home() -> Html {
    let navigator = use_navigator();
    let go_to_language = Callback::from(move |segment: PieSegment| match &navigator {
        Some(navigator) => navigator.push(&Route::Language {
            key: segment.key.to_string(),
        }),
        None => gloo_console::warn!("no router in scope; cannot open", segment.key.to_string()),
    });
    let language_segments = language_segments();
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
    let cb_on_text_update = Callback::from(|tag: String| {
        gloo_console::log!("Tex updated: {}", tag);
    });
    let items: UseStateHandle<Vec<String>> = use_state(|| vec!["Apple".to_string(), "Banana".to_string(), "Cherry".to_string()]);

    let fruit = use_state(String::new);
    let on_fruit_change = {
        let fruit = fruit.clone();
        Callback::from(move |value: String| fruit.set(value))
    };
    let email = use_state(String::new);
    let on_email_change = {
        let email = email.clone();
        Callback::from(move |value: String| email.set(value))
    };
    let email_is_invalid = !email.is_empty() && !email.contains('@');
    let shipping = use_state(|| None::<String>);
    let on_shipping_change = {
        let shipping = shipping.clone();
        Callback::from(move |value: String| shipping.set(Some(value)))
    };

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
                    <ybc::ButtonAnchor classes={classes!("is-black", "is-outlined")} rel={String::from("noopener noreferrer")} target={String::from("_blank")} href="https://github.com/goodidea-kp/ybc">
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
                                <ybc::Accordions id={Rc::from("accordions")}>
                                    <ybc::AccordionItem title={Rc::from("Accordion 1")} open={true} id={Rc::from("accordion1")}>
                                        <p>{"This is the content of the first accordion."}</p>
                                    </ybc::AccordionItem>
                                    <ybc::AccordionItem title={Rc::from("Accordion 2")} open={false} id={Rc::from("accordion2")}>
                                        <p>{"This is the content of the second accordion."}</p>
                                    </ybc::AccordionItem>
                                </ybc::Accordions>
                                <ModalControllerProvider>
                                    <MyModal1/>
                                    <MyModal2/>
                                </ModalControllerProvider>
                            </ybc::Tile>
                        </ybc::Tile>
                        <ybc::Tile ctx={Parent}>
                            <ybc::Tile ctx={Child} classes={classes!("notification", "is-light")}>
                                <ybc::Subtitle size={ybc::HeaderSize::Is3} classes={classes!("has-text-dark")}>{"Tabs"}</ybc::Subtitle>
                                <ybc::TabsProvider>
                                    <ybc::Tabs boxed=true>
                                        <ybc::TabItem index={0}>
                                            <span>{"Pictures"}</span>
                                        </ybc::TabItem>
                                        <ybc::TabItem index={1}>
                                            <span>{"Music"}</span>
                                        </ybc::TabItem>
                                        <ybc::TabItem index={2}>
                                            <span>{"Videos"}</span>
                                        </ybc::TabItem>
                                    </ybc::Tabs>
                                    <ybc::TabPanel index={0}>
                                        <ybc::Box>
                                            <p class="has-text-black">{"Pictures Content: Standard Bulma tabs component demonstration."}</p>
                                        </ybc::Box>
                                    </ybc::TabPanel>
                                    <ybc::TabPanel index={1}>
                                        <ybc::Box>
                                            <p class="has-text-black">{"Music Content: Standard Bulma tabs component demonstration."}</p>
                                        </ybc::Box>
                                    </ybc::TabPanel>
                                    <ybc::TabPanel index={2}>
                                        <ybc::Box>
                                            <p class="has-text-black">{"Videos Content: Standard Bulma tabs component demonstration."}</p>
                                        </ybc::Box>
                                    </ybc::TabPanel>
                                </ybc::TabsProvider>
                            </ybc::Tile>
                        </ybc::Tile>
                        <ybc::Tile ctx={Parent}>
                            <ybc::Tile ctx={Child} classes={classes!("box")}>
                                <ybc::Subtitle size={ybc::HeaderSize::Is4}>{"Donut chart"}</ybc::Subtitle>
                                <p class="mb-4">{"Click or focus a segment, or its legend entry, to open that language's page. Segments past the sixth fold into \"Other\"."}</p>
                                <ybc::PieChart
                                    title={"Lines of code by language"}
                                    total_label={"lines total"}
                                    segments={language_segments.clone()}
                                    follow_href={false}
                                    on_select={go_to_language.clone()}
                                />
                            </ybc::Tile>
                            <ybc::Tile ctx={Child} classes={classes!("box")}>
                                <ybc::Subtitle size={ybc::HeaderSize::Is4}>{"Pie variant"}</ybc::Subtitle>
                                <p class="mb-4">{"The same data with hole={0.0} and max_segments={4}; the readout moves beside the chart."}</p>
                                <ybc::PieChart
                                    segments={language_segments}
                                    hole={0.0}
                                    max_segments={4}
                                    size={"11rem"}
                                    follow_href={false}
                                    on_select={go_to_language}
                                />
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
                                <ybc::Progress value={-1.0} max={100.0} classes={classes!("is-primary")} />
                                <ybc::Field label={Some("Prompt")} help={"The label is bound to the textarea through the field's auto-generated id"}>
                                    <ybc::Control>
                                        <ybc::TextArea update={cb_on_text_update} rows={3} name={String::from("textarea")} value={String::from("Hello, ChatGpt!")} placeholder={String::from("Enter some text")}
                                           is_genai={true}
                                        />
                                    </ybc::Control>
                                </ybc::Field>
                                <ybc::Field label={Some("Favourite fruit")} help={"Required; the placeholder option cannot be re-selected"}>
                                    <ybc::Control>
                                        <ybc::Select name="fruit" value={(*fruit).clone()} update={on_fruit_change} placeholder={"Choose a fruit"} required={true}>
                                            <option value="apple">{"Apple"}</option>
                                            <option value="banana">{"Banana"}</option>
                                            <option value="cherry">{"Cherry"}</option>
                                        </ybc::Select>
                                    </ybc::Control>
                                </ybc::Field>
                                <ybc::Field
                                    label={Some("Email")}
                                    help={if email_is_invalid { "Enter an address containing '@'" } else { "Autofill and the email keyboard are enabled" }}
                                    help_has_error={email_is_invalid}
                                >
                                    <ybc::Control>
                                        <ybc::Input
                                            name="email"
                                            r#type={ybc::InputType::Email}
                                            value={(*email).clone()}
                                            update={on_email_change}
                                            placeholder={String::from("you@example.com")}
                                            autocomplete={"email"}
                                            inputmode={ybc::InputMode::Email}
                                            required={true}
                                        />
                                    </ybc::Control>
                                </ybc::Field>
                                <ybc::RadioGroup legend={"Shipping speed"}>
                                    <ybc::Radio name="shipping" value="standard" checked_value={(*shipping).clone()} update={on_shipping_change.clone()} required={true}>{" Standard"}</ybc::Radio>
                                    <ybc::Radio name="shipping" value="express" checked_value={(*shipping).clone()} update={on_shipping_change}>{" Express"}</ybc::Radio>
                                </ybc::RadioGroup>
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

use ybc::ModalControllerContext;
use ybc::ModalControllerProvider;

#[component]
pub fn MyModal1() -> Html {
    let controller = use_context::<ModalControllerContext>().unwrap();
    let onclick = {
        let controller = controller.clone();
        Callback::from(move |_| controller.close("id0"))
    };
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

#[component(MyModal2)]
pub fn my_modal2() -> Html {
    let controller = use_context::<ModalControllerContext>().unwrap();
    let onclick = {
        let controller = controller.clone();
        Callback::from(move |_| controller.close("id2"))
    };
    let onsave = {
        let controller = controller.clone();
        Callback::from(move |_| controller.close("id2"))
    };
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
