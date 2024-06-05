use std::collections::HashSet;
use std::rc::Rc;
use wasm_bindgen::JsCast;

use yew::prelude::*;

use yew_agent::worker::{HandlerId, Worker, WorkerScope};

use yew_agent::prelude::*;

/// Modal actions.
pub enum ModalMsg {
    Open,
    Close,
    CloseFromAgent(ModalCloseMsg),
}

pub type ModalCloserContext = UseReducerHandle<ModalCloseMsg>;

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct ModalProps {
    /// The ID of this modal, used for triggering close events from other parts of the app.
    pub id: String,
    /// The content of the `"modal-content"` element.
    #[prop_or_default]
    pub children: Children,
    /// The contents of the modal trigger, typically a button or the like.
    #[prop_or_default]
    pub trigger: Html,
    #[prop_or_default]
    pub classes: Classes,
}

/// A classic modal overlay, in which you can include any content you want.
///
/// [https://bulma.io/documentation/components/modal/](https://bulma.io/documentation/components/modal/)
///
/// See the docs on the `ModalCloser` agent to be able to close your modal instance from anywhere
/// in your app for maximum flexibility.
#[function_component(Modal)]
pub fn modal(props: &ModalProps) -> Html {
    let is_active = use_state(|| false);
    let id = props.id.clone();
    let closer_ctx = use_context::<ModalCloserContext>().expect("Modal closer in context");
    let mut class = Classes::from("modal");

    class.push(props.classes.clone());
    let (_id, closed) = match closer_ctx.0.contains("-") {
        true => {
            let result = closer_ctx.0.split("-").collect::<Vec<&str>>();
            (result[0], result[1] == "close")
        }
        false => (closer_ctx.0.as_str(), false),
    };

    let (opencb, closecb) = if _id == id && *is_active {
        class.push("is-active");

        let is_active = is_active.clone();

        (Callback::noop(), Callback::from(move |_| is_active.set(false)))
    } else if _id == id {
        let is_active = is_active.clone();

        (Callback::from(move |_| is_active.set(true)), Callback::noop())
    } else {
        (Callback::noop(), Callback::noop())
    };

    html! {
        <>
        <div onclick={opencb}>
            {props.trigger.clone()}
        </div>
        <div id={props.id.clone()} {class}>
            <div class="modal-background" onclick={closecb.clone()}></div>
            <div class="modal-content">
                {props.children.clone()}
            </div>
            <button class="modal-close is-large" aria-label="close" onclick={closecb}></button>
        </div>
        </>
    }
}

//////////////////////////////////////////////////////////////////////////////

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct ModalCardProps {
    /// The ID of this modal, used for triggering close events from other parts of the app.
    pub id: AttrValue,
    /// The title of this modal.
    pub title: AttrValue,
    /// The content to be placed in the `modal-card-body` not including the modal-card-header /
    /// modal-card-title, which is handled by the `modal_title` prop.
    #[prop_or_default]
    pub body: Html,
    /// The content to be placed in the `modal-card-footer`.
    #[prop_or_default]
    pub footer: Html,
    /// The contents of the modal trigger, typically a button or the like.
    #[prop_or_default]
    pub trigger: Html,
    #[prop_or_default]
    pub classes: Classes,
}

/// A classic modal with a header, body, and footer section.
///
/// [https://bulma.io/documentation/components/modal/](https://bulma.io/documentation/components/modal/)
///
/// See the docs on the `ModalCloser` agent to be able to close your modal instance from anywhere
/// in your app for maximum flexibility.
#[function_component(ModalCard)]
pub fn modal_card(props: &ModalCardProps) -> Html {
    let id = props.id.clone();
    let closer_ctx = use_context::<ModalCloserContext>().expect("Modal closer in context");

    // gloo_console::log!("closer_ctx: full ID {}", closer_ctx.0.as_str());

    let (_id, closed) = match closer_ctx.0.contains("-") {
        true => {
            let result = closer_ctx.0.split("-").collect::<Vec<&str>>();
            (result[0], result[1] == "close")
        }
        false => (closer_ctx.0.as_str(), false),
    };
    let is_active = use_state(|| false);

    // gloo_console::log!("closer_ctx: {:?} id:{:?} is closed:", _id.clone().into(), id.clone().into(), closed);

    if _id == id && closed {
        is_active.set(false);
        closer_ctx.dispatch(id.clone());
        // gloo_console::log!("closed!");
    }

    let mut class = Classes::from("modal");
    class.push(props.classes.clone());

    let (opencb, closecb) = if _id == id && *is_active {

        class.push("is-active");
        // gloo_console::log!("is_active=true call");

        let is_active = is_active.clone();

        (Callback::noop(), Callback::from(move |e:MouseEvent| {
            let target = e.target();
            // gloo_console::log!("Close event from modal-card: {:?}");
            // Check if the target is an element that you want to ignore
            if let Some(target) = target {
                let target_element = target.dyn_into::<web_sys::Element>().unwrap();
                if target_element.id().starts_with("modal-ignore-") {
                    // If the target is an element to ignore, stop the event propagation
                    e.stop_propagation();
                    // gloo_console::log!("Ignoring event");
                    return;
                }
            }
            is_active.set(false)
        }))
    } else if _id == id {
        let is_active = is_active.clone();
        // gloo_console::log!("is_active=false call");
        (Callback::from(move |_| is_active.set(true)), Callback::noop())
    } else {
        // gloo_console::log!("NOOP call");
        (Callback::noop(), Callback::noop())
    };

    html! {
    <>
        <div onclick={opencb}>
            {props.trigger.clone()}
        </div>
        <div id={props.id.clone()} {class}>
            <div class="modal-background" onclick={closecb.clone()}></div>
            <div class="modal-card">
                <header class="modal-card-head">
                    <p class="modal-card-title">{props.title.clone()}</p>
                    <button class="delete" aria-label="close" onclick={closecb.clone()}></button>
                </header>
                <section class="modal-card-body">
                    {props.body.clone()}
                </section>
                <footer class="modal-card-foot">
                    {props.footer.clone()}
                </footer>
            </div>
            <button class="modal-close is-large" aria-label="close" onclick={closecb}></button>
        </div>
    </>
    }
}

#[function_component(ModalCard2)]
pub fn modal_card2(props: &ModalCardProps) -> Html {
    let id = props.id.clone();
    let closer_ctx = use_context::<ModalCloserContext>().expect("Modal closer in context");

    // gloo_console::log!("closer_ctx: full ID {}", closer_ctx.0.as_str());
    let action = closer_ctx.0.as_str();
    let (_id, closed) = match action.contains("-") {
        true => {
            let result = action.split("-").collect::<Vec<&str>>();
            (result[0], result[1] == "close")
        }
        false => (action, false),
    };
    let is_active = use_state(|| false);

    // gloo_console::log!("closer_ctx: {:?} id:{:?} is closed:", _id.clone().into(), id.clone().into(), closed);

    if _id == id && closed {
        is_active.set(false);
        closer_ctx.dispatch(id.clone());
        // gloo_console::log!("closed!");
    }

    let mut class = Classes::from("modal");
    class.push(props.classes.clone());

    let (opencb, closecb) = if _id == id && *is_active {
        class.push("is-active");
        // gloo_console::log!("is_active=true call");

        let is_active = is_active.clone();

        (Callback::noop(), Callback::from(move |_| is_active.set(false)))
    } else if _id == id {
        let is_active = is_active.clone();
        // gloo_console::log!("is_active=false call");
        (Callback::from(move |_| is_active.set(true)), Callback::noop())
    } else {
        // gloo_console::log!("NOOP call");
        (Callback::noop(), Callback::noop())
    };

    html! {
    <>
        <div onclick={opencb}>
            {props.trigger.clone()}
        </div>
        <div id={props.id.clone()} {class}>
            <div class="modal-background" onclick={closecb.clone()}></div>
            <div class="modal-card">
                <header class="modal-card-head">
                    <p class="modal-card-title">{props.title.clone()}</p>
                    <button class="delete" aria-label="close" onclick={closecb.clone()}></button>
                </header>
                <section class="modal-card-body">
                    {props.body.clone()}
                </section>
                <footer class="modal-card-foot">
                    {props.footer.clone()}
                </footer>
            </div>
            <button class="modal-close is-large" aria-label="close" onclick={closecb}></button>
        </div>
    </>
    }
}

//////////////////////////////////////////////////////////////////////////////

/// A request to close a modal instance by ID.
///
/// The ID provided in this message must match the ID of the modal which is to be closed, else
/// the message will be ignored.
#[derive(Clone, Debug, PartialEq)]
pub struct ModalCloseMsg(pub AttrValue);

impl Reducible for ModalCloseMsg {
    type Action = AttrValue;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        ModalCloseMsg { 0: action }.into()
    }
}

/// An agent used for being able to close `Modal` & `ModalCard` instances by ID.
///
/// If custom modal closing functionality is need for your modal instance, the following
/// pattern is recommended.
///
/// First, in your component which is using this modal, configure a `ModalCloser` dispatcher.
/// ```rust
/// use yew::agent::Dispatcher;
/// use yew::prelude::*;
/// // .. snip ..
/// fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
///     let bridge = ModalCloser::dispatcher();
///     Self { link, props, bridge }
/// }
/// ```
///
/// Next, in your component's `view` method, setup a callback to handle your component's close
/// event. ```rust
/// let closer = self.link.callback(|_| ModalCloseMsg("modal-0".into()));
/// // ... snip ...
/// <ModalCard
///     id="modal-0"
///     // ... snip ...
///     footer=html!{
///         <Button onclick=Some(closer)>{"Close"}</Button>
///     }
/// />
/// ```
///
/// Finally, in your component's `update` method, send the `ModalCloseMsg` over to the agent which
/// will forward the message to the modal to cause it to close.
/// ```rust
/// fn update(&mut self, msg: Self::Message) -> ShouldRender {
///     self.bridge.send(msg);
///     true
/// }
/// ```
///
/// This pattern allows you to communicate with a modal by its given ID, allowing
/// you to close the modal from anywhere in your application.
pub struct ModalCloser {
    link: WorkerScope<Self>,
    subscribers: HashSet<HandlerId>,
}

impl Worker for ModalCloser {
    type Input = ModalCloseMsg;
    type Message = ();
    // The agent receives requests to close modals by ID.
    type Output = ModalCloseMsg;

    // The agent forwards the input to all registered modals.

    fn create(link: &WorkerScope<Self>) -> Self {
        Self { link: link.clone(), subscribers: HashSet::new() }
    }

    fn update(&mut self, scope: &WorkerScope<Self>, _: Self::Message) {}

    fn connected(&mut self, scope: &WorkerScope<Self>, id: HandlerId) {
        self.subscribers.insert(id);
    }

    fn disconnected(&mut self, scope: &WorkerScope<Self>, id: HandlerId) {
        self.subscribers.remove(&id);
    }

    fn received(&mut self, scope: &WorkerScope<Self>, msg: Self::Input, id: HandlerId) {
        for cmp in self.subscribers.iter() {
            self.link.respond(*cmp, msg.clone());
        }
    }
}
#[derive(Properties, Debug, PartialEq)]
pub struct ModalCloserProviderProps {
    #[prop_or_default]
    pub children: Html,
    pub id: String,
}

#[function_component]
pub fn ModalCloserProvider(props: &ModalCloserProviderProps) -> Html {
    let msg = use_reducer(|| ModalCloseMsg { 0: props.id.clone().into() });
    html! {
        <ContextProvider<ModalCloserContext> context={ msg }>
         { props.children.clone() }
        </ContextProvider<ModalCloserContext >>
    }
}
