//! Minimal repro of a suspected Tabs/Level desync bug in bastion/ui-client's
//! keys_dashboard: a class component whose CardHeader-style toolbar renders a
//! DIFFERENT NUMBER of ybc::Button children depending on the active tab, kept
//! in sync with the URL via a location listener, and reached through a
//! NESTED Switch (outer wildcard route -> inner exact-path route) exactly
//! like `AppRoute::DashboardKeys` (`/dk/:k`) wraps `Switch<DKRoute>`.

use std::rc::Rc;
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Clone, PartialEq)]
pub struct AppState {
    pub event: String,
}

#[derive(Properties, PartialEq)]
pub struct ModalProps {
    pub is_active: bool,
}

#[function_component(ModalA)]
fn modal_a(props: &ModalProps) -> Html {
    if !props.is_active {
        return html! {};
    }
    html! { <div class="modal is-active" data-testid="modal-a"></div> }
}

#[function_component(ModalB)]
fn modal_b(props: &ModalProps) -> Html {
    if !props.is_active {
        return html! {};
    }
    html! { <div class="modal is-active" data-testid="modal-b"></div> }
}

#[derive(Properties, PartialEq)]
pub struct ListProps {
    pub refresh_token: u64,
}

pub enum ListMsg {
    Loaded,
}

pub struct ListA;
impl Component for ListA {
    type Message = ListMsg;
    type Properties = ListProps;
    fn create(ctx: &Context<Self>) -> Self {
        ctx.link().send_future(async { ListMsg::Loaded });
        Self
    }
    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
        true
    }
    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! { <p data-testid="content">{"Content A (list component)"}</p> }
    }
}

pub struct ListB;
impl Component for ListB {
    type Message = ListMsg;
    type Properties = ListProps;
    fn create(ctx: &Context<Self>) -> Self {
        ctx.link().send_future(async { ListMsg::Loaded });
        Self
    }
    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
        true
    }
    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! { <p data-testid="content">{"Content B (list component)"}</p> }
    }
}

#[derive(Clone, Routable, PartialEq, Debug)]
pub enum ReproRoute {
    #[at("/repro/a")]
    A,
    #[not_found]
    #[at("/repro/b")]
    B,
}

pub(crate) fn repro_switch(r: ReproRoute) -> Html {
    let active_tab = match r {
        ReproRoute::B => "/repro/b".to_string(),
        ReproRoute::A => "/repro/a".to_string(),
    };
    let state = Rc::new(AppState { event: String::new() });
    html! {
        <ContextProvider<Rc<AppState>> context={state}>
            <ReproPage {active_tab} />
        </ContextProvider<Rc<AppState>>>
    }
}

#[derive(Clone, PartialEq, Debug)]
pub enum TabValues {
    A,
    B,
}

pub enum Msg {
    OnTabAction(TabValues),
    RouteChange(Location),
    ContextChanged(Rc<AppState>),
}

#[derive(Clone, PartialEq, Properties)]
pub struct ReproProps {
    pub active_tab: String,
}

pub struct ReproPage {
    active_tab: TabValues,
    _history_listener: LocationHandle,
    _ctx_listener: ContextHandle<Rc<AppState>>,
    modal_active: bool,
    refresh_token: u64,
}

fn tab_for_path(path: &str) -> TabValues {
    match path {
        "/repro/b" => TabValues::B,
        _ => TabValues::A,
    }
}

impl Component for ReproPage {
    type Message = Msg;
    type Properties = ReproProps;

    fn create(ctx: &Context<Self>) -> Self {
        let listener = ctx
            .link()
            .add_location_listener(ctx.link().callback(Msg::RouteChange))
            .unwrap();
        let (_state, ctx_listener) = ctx
            .link()
            .context::<Rc<AppState>>(ctx.link().callback(Msg::ContextChanged))
            .expect("context to be set");
        let active_tab = tab_for_path(&ctx.props().active_tab);
        // Mirror KeysDashboard::create exactly: it unconditionally
        // `n.replace()`s to the concrete route matching the resolved tab,
        // even when the URL already matches.
        let navigator = ctx.link().navigator().unwrap();
        navigator.replace(&match active_tab {
            TabValues::A => ReproRoute::A,
            TabValues::B => ReproRoute::B,
        });
        Self {
            active_tab,
            _history_listener: listener,
            _ctx_listener: ctx_listener,
            modal_active: false,
            refresh_token: 0,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::OnTabAction(t) => {
                self.active_tab = t.clone();
                let navigator = ctx.link().navigator().unwrap();
                navigator.push(&match t {
                    TabValues::A => ReproRoute::A,
                    TabValues::B => ReproRoute::B,
                });
                true
            }
            Msg::RouteChange(loc) => {
                self.active_tab = tab_for_path(loc.path());
                true
            }
            Msg::ContextChanged(_state) => true,
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let active_index = match &self.active_tab {
            TabValues::A => 0usize,
            TabValues::B => 1usize,
        };
        html! {
            <ybc::Section>
                <ybc::Container>
                    <h1 class="title">{"Level/Tabs desync repro (nested Switch)"}</h1>
                    <p>{format!("active_tab = {:?}", self.active_tab)}</p>
                    <ybc::Tabs boxed=true active={Some(active_index)}
                               set_active={ctx.link().callback(|idx: usize| Msg::OnTabAction(if idx == 1 { TabValues::B } else { TabValues::A }))}>
                        <ybc::TabItem index={0usize} on_select={ctx.link().callback(|_: usize| Msg::OnTabAction(TabValues::A))}>{"A (5 buttons)"}</ybc::TabItem>
                        <ybc::TabItem index={1usize} on_select={ctx.link().callback(|_: usize| Msg::OnTabAction(TabValues::B))}>{"B (3 buttons)"}</ybc::TabItem>
                    </ybc::Tabs>
                    <ybc::Card>
                        <ybc::CardHeader>
                        {
                            match &self.active_tab {
                                TabValues::A => html! {
                                    <ybc::Level>
                                        <ybc::Button>{"Add"}</ybc::Button>
                                        <ybc::Button>{"Edit"}</ybc::Button>
                                        <ybc::Button>{"Delete"}</ybc::Button>
                                        <ybc::Button>{"Rotate"}</ybc::Button>
                                        <ybc::Button>{"Destroy"}</ybc::Button>
                                        <ModalA is_active={self.modal_active} />
                                    </ybc::Level>
                                },
                                TabValues::B => html! {
                                    <ybc::Level>
                                        <ybc::Button>{"Add"}</ybc::Button>
                                        <ybc::Button>{"Edit"}</ybc::Button>
                                        <ybc::Button>{"Delete"}</ybc::Button>
                                        <ModalB is_active={self.modal_active} />
                                    </ybc::Level>
                                },
                            }
                        }
                        </ybc::CardHeader>
                        <ybc::CardContent>
                        {
                            match &self.active_tab {
                                TabValues::A => html! { <ListA refresh_token={self.refresh_token} /> },
                                TabValues::B => html! { <ListB refresh_token={self.refresh_token} /> },
                            }
                        }
                        </ybc::CardContent>
                    </ybc::Card>
                </ybc::Container>
            </ybc::Section>
        }
    }
}
