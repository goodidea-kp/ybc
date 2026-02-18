use std::collections::HashSet;
use std::rc::Rc;
use std::sync::atomic::{AtomicUsize, Ordering};

use wasm_bindgen::JsCast;
use web_sys::{Event, HtmlDialogElement, HtmlElement, MouseEvent};
use yew::prelude::*;

/// Modal actions kept for backwards compatibility.
#[derive(Clone, Debug, PartialEq)]
pub enum ModalMsg {
    Open,
    Close,
}

#[derive(Clone, Debug, PartialEq)]
enum ModalControllerAction {
    Open(String),
    Close(String),
    CloseAll,
}

#[derive(Clone, Debug, Default, PartialEq)]
struct ModalControllerState {
    open_ids: HashSet<String>,
}

impl Reducible for ModalControllerState {
    type Action = ModalControllerAction;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        let mut open_ids = self.open_ids.clone();

        match action {
            ModalControllerAction::Open(id) => {
                open_ids.insert(id);
            }
            ModalControllerAction::Close(id) => {
                open_ids.remove(&id);
            }
            ModalControllerAction::CloseAll => {
                open_ids.clear();
            }
        }

        Rc::new(Self { open_ids })
    }
}

/// A controller for opening and closing modals from anywhere in the component tree.
#[derive(Clone, PartialEq)]
pub struct ModalController {
    state: UseReducerHandle<ModalControllerState>,
}

impl ModalController {
    fn new(state: UseReducerHandle<ModalControllerState>) -> Self {
        Self { state }
    }

    /// Returns true if the modal with `id` is currently open.
    pub fn is_open(&self, id: &str) -> bool {
        self.state.open_ids.contains(id)
    }

    /// Open a modal by id.
    pub fn open(&self, id: impl Into<String>) {
        self.state.dispatch(ModalControllerAction::Open(id.into()));
    }

    /// Close a modal by id.
    pub fn close(&self, id: impl AsRef<str>) {
        self.state.dispatch(ModalControllerAction::Close(id.as_ref().to_owned()));
    }

    /// Close all modals.
    pub fn close_all(&self) {
        self.state.dispatch(ModalControllerAction::CloseAll);
    }
}

/// Context type for the modal controller.
pub type ModalControllerContext = ModalController;

static MODAL_AUTO_ID: AtomicUsize = AtomicUsize::new(1);

fn next_modal_id(prefix: &str) -> String {
    format!("{}-{}", prefix, MODAL_AUTO_ID.fetch_add(1, Ordering::Relaxed))
}

const DIALOG_STYLE: &str = r#"
/* Avoid ghost overlays when state is closed. */
dialog.modal:not([open]) {
    display: none !important;
}

/* Make <dialog class="modal"> match Bulma's modal container behavior. */
dialog.modal[open] {
    position: fixed !important;
    inset: 0 !important;
    width: 100vw !important;
    height: 100vh !important;

    display: flex !important;
    align-items: center !important;
    justify-content: center !important;

    border: 0 !important;
    outline: 0 !important;
    box-shadow: none !important;
    padding: 0 !important;
    margin: 0 !important;
    background: transparent !important;
    color: inherit !important;
    max-width: none !important;
    max-height: none !important;
    -webkit-appearance: none;
    appearance: none;
}

dialog.modal:focus,
dialog.modal:focus-visible {
    outline: 0 !important;
    box-shadow: none !important;
}

dialog.modal::backdrop {
    background: rgba(10, 10, 10, 0.86);
}

/* Use a Font Awesome icon instead of Bulma's pseudo-element cross. */
dialog.modal .ybc-modal-icon-close::before,
dialog.modal .ybc-modal-icon-close::after {
    display: none !important;
}

dialog.modal .ybc-modal-icon-close .icon {
    color: #fff;
    font-size: 1.1rem;
}
"#;

fn base_class(extra: &Classes, is_active: bool) -> Classes {
    let mut class = classes!("modal");
    class.push(extra.clone());
    if is_active {
        class.push("is-active");
    }
    class
}

fn focus_dialog(dialog: &HtmlDialogElement) {
    if let Ok(Some(el)) = dialog.query_selector("[data-ybc-dialog-focus]")
        && let Ok(html) = el.dyn_into::<HtmlElement>()
    {
        let _ = html.focus();
        return;
    }
    let _ = dialog.focus();
}

fn close_dialog(dialog_ref: &NodeRef) {
    if let Some(dialog) = dialog_ref.cast::<HtmlDialogElement>()
        && dialog.open()
    {
        dialog.close();
    }
}

fn close_icon() -> Html {
    html! {
        <span class="icon" aria-hidden="true">
            <i class="fa-solid fa-xmark"></i>
        </span>
    }
}

fn should_ignore_target(event: &MouseEvent) -> bool {
    let Some(target) = event.target() else {
        return false;
    };

    target
        .dyn_into::<web_sys::Element>()
        .map(|el| el.id().starts_with("modal-ignore-"))
        .unwrap_or(false)
}

#[derive(Properties, PartialEq)]
struct DialogShellProps {
    id: String,
    #[prop_or_default]
    classes: Classes,
    is_active: bool,
    set_is_active: Callback<bool>,
    dialog_ref: NodeRef,
    #[prop_or_default]
    children: Children,
}

#[component(DialogShell)]
fn dialog_shell(props: &DialogShellProps) -> Html {
    let controller = use_context::<ModalControllerContext>();

    {
        let dialog_ref = props.dialog_ref.clone();
        let set_is_active = props.set_is_active.clone();
        use_effect_with(props.is_active, move |active| {
            if let Some(dialog) = dialog_ref.cast::<HtmlDialogElement>() {
                if *active {
                    if !dialog.open() {
                        let _ = dialog.show_modal();
                    }
                    focus_dialog(&dialog);
                } else if dialog.open() {
                    dialog.close();
                }

                if !dialog.open() && *active {
                    set_is_active.emit(false);
                }
            }

            || {}
        });
    }

    let class = base_class(&props.classes, props.is_active);

    let id_for_cancel = props.id.clone();
    let id_for_close = props.id.clone();

    let dialog_ref_for_cancel = props.dialog_ref.clone();
    let set_is_active_for_cancel = props.set_is_active.clone();
    let controller_for_cancel = controller.clone();

    let set_is_active_for_close = props.set_is_active.clone();
    let controller_for_close = controller.clone();

    html! {
        <>
            <style>{DIALOG_STYLE}</style>
            <dialog
                id={props.id.clone()}
                class={class}
                ref={props.dialog_ref.clone()}
                oncancel={Callback::from(move |ev: Event| {
                    ev.prevent_default();
                    close_dialog(&dialog_ref_for_cancel);
                    set_is_active_for_cancel.emit(false);
                    if let Some(controller) = controller_for_cancel.as_ref() {
                        controller.close(&id_for_cancel);
                    }
                })}
                onclose={Callback::from(move |_ev: Event| {
                    set_is_active_for_close.emit(false);
                    if let Some(controller) = controller_for_close.as_ref() {
                        controller.close(&id_for_close);
                    }
                })}
            >
                { for props.children.iter() }
            </dialog>
        </>
    }
}

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct ModalProps {
    /// Optional modal id used as controller key and dialog id attribute.
    ///
    /// If omitted, a unique id is generated automatically. For programmatic
    /// open/close via `ModalControllerContext`, provide a stable id.
    #[prop_or_default]
    pub id: Option<String>,
    /// The content of the `"modal-content"` element.
    #[prop_or_default]
    pub children: Children,
    /// The contents of the modal trigger, typically a button or the like.
    #[prop_or_default]
    pub trigger: Html,
    /// Extra classes applied to the root `.modal`.
    #[prop_or_default]
    pub classes: Classes,
    /// Controlled open state.
    #[prop_or_default]
    pub open: Option<bool>,
    /// Controlled state setter.
    #[prop_or_default]
    pub set_open: Option<Callback<bool>>,
}

/// A Bulma modal overlay built on top of native `<dialog>`.
///
/// By default this component manages its own local state and opens on trigger click.
/// If wrapped in a [`ModalControllerProvider`], controller state becomes the source of truth
/// for uncontrolled modals.
#[component(Modal)]
pub fn modal(props: &ModalProps) -> Html {
    let internal_open = use_state(|| false);
    let is_controlled = props.open.is_some() && props.set_open.is_some();
    let is_active = props.open.unwrap_or(*internal_open);

    let controller = use_context::<ModalControllerContext>();
    let dialog_ref = use_node_ref();
    let auto_id = use_state(|| next_modal_id("modal"));
    let modal_id = props.id.clone().unwrap_or_else(|| (*auto_id).clone());

    let set_local_open = {
        let internal_open = internal_open.clone();
        let set_open = props.set_open.clone();
        Callback::from(move |value: bool| {
            if is_controlled {
                if let Some(set_open) = set_open.as_ref() {
                    set_open.emit(value);
                }
            } else {
                internal_open.set(value);
            }
        })
    };

    {
        let set_local_open = set_local_open.clone();
        use_effect_with(
            (controller.clone(), modal_id.clone(), is_controlled),
            move |(controller, modal_id, is_controlled)| {
                if !*is_controlled && let Some(controller) = controller.as_ref() {
                    set_local_open.emit(controller.is_open(modal_id));
                }

                || {}
            },
        );
    }

    let open_action = {
        let modal_id = modal_id.clone();
        let controller = controller.clone();
        let set_local_open = set_local_open.clone();
        Callback::from(move |_| {
            if !is_controlled && let Some(controller) = controller.as_ref() {
                controller.open(modal_id.clone());
                return;
            }

            set_local_open.emit(true);
            if let Some(controller) = controller.as_ref() {
                controller.open(modal_id.clone());
            }
        })
    };

    let close_action = {
        let modal_id = modal_id.clone();
        let controller = controller.clone();
        let set_local_open = set_local_open.clone();
        let dialog_ref = dialog_ref.clone();
        Callback::from(move |_| {
            close_dialog(&dialog_ref);

            if !is_controlled && let Some(controller) = controller.as_ref() {
                controller.close(&modal_id);
                return;
            }

            set_local_open.emit(false);
            if let Some(controller) = controller.as_ref() {
                controller.close(&modal_id);
            }
        })
    };

    let bg_close = {
        let close_action = close_action.clone();
        Callback::from(move |event: MouseEvent| {
            if should_ignore_target(&event) {
                event.stop_propagation();
                return;
            }
            close_action.emit(());
        })
    };

    let close_btn_close = {
        let close_action = close_action.clone();
        Callback::from(move |_| close_action.emit(()))
    };

    html! {
        <>
            <div onclick={open_action}>
                {props.trigger.clone()}
            </div>

            <DialogShell
                id={modal_id}
                classes={props.classes.clone()}
                is_active={is_active}
                set_is_active={set_local_open}
                dialog_ref={dialog_ref}
            >
                <div class="modal-background" onclick={bg_close}></div>

                <div class="modal-content">
                    { for props.children.iter() }
                </div>

                <button class="modal-close is-large ybc-modal-icon-close" type="button" aria-label="close" onclick={close_btn_close}>
                    {close_icon()}
                </button>
            </DialogShell>
        </>
    }
}

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct ModalCardProps {
    /// Optional modal id used as controller key and dialog id attribute.
    ///
    /// If omitted, a unique id is generated automatically. For programmatic
    /// open/close via `ModalControllerContext`, provide a stable id.
    #[prop_or_default]
    pub id: Option<AttrValue>,
    /// The title of this modal.
    pub title: AttrValue,
    /// The content to be placed in the `modal-card-body`.
    #[prop_or_default]
    pub body: Html,
    /// The content to be placed in the `modal-card-footer`.
    #[prop_or_default]
    pub footer: Html,
    /// The contents of the modal trigger, typically a button or the like.
    #[prop_or_default]
    pub trigger: Html,
    /// Extra classes applied to the root `.modal`.
    #[prop_or_default]
    pub classes: Classes,
    /// Controlled open state.
    #[prop_or_default]
    pub open: Option<bool>,
    /// Controlled state setter.
    #[prop_or_default]
    pub set_open: Option<Callback<bool>>,
}

/// A Bulma modal card built on top of native `<dialog>`.
#[component(ModalCard)]
pub fn modal_card(props: &ModalCardProps) -> Html {
    let internal_open = use_state(|| false);
    let is_controlled = props.open.is_some() && props.set_open.is_some();
    let is_active = props.open.unwrap_or(*internal_open);

    let controller = use_context::<ModalControllerContext>();
    let dialog_ref = use_node_ref();
    let auto_id = use_state(|| AttrValue::from(next_modal_id("modal-card")));
    let modal_id = props.id.clone().unwrap_or_else(|| (*auto_id).clone()).to_string();

    let set_local_open = {
        let internal_open = internal_open.clone();
        let set_open = props.set_open.clone();
        Callback::from(move |value: bool| {
            if is_controlled {
                if let Some(set_open) = set_open.as_ref() {
                    set_open.emit(value);
                }
            } else {
                internal_open.set(value);
            }
        })
    };

    {
        let set_local_open = set_local_open.clone();
        use_effect_with(
            (controller.clone(), modal_id.clone(), is_controlled),
            move |(controller, modal_id, is_controlled)| {
                if !*is_controlled && let Some(controller) = controller.as_ref() {
                    set_local_open.emit(controller.is_open(modal_id));
                }

                || {}
            },
        );
    }

    let open_action = {
        let modal_id = modal_id.clone();
        let controller = controller.clone();
        let set_local_open = set_local_open.clone();
        Callback::from(move |_| {
            if !is_controlled && let Some(controller) = controller.as_ref() {
                controller.open(modal_id.clone());
                return;
            }

            set_local_open.emit(true);
            if let Some(controller) = controller.as_ref() {
                controller.open(modal_id.clone());
            }
        })
    };

    let close_action = {
        let modal_id = modal_id.clone();
        let controller = controller.clone();
        let set_local_open = set_local_open.clone();
        let dialog_ref = dialog_ref.clone();
        Callback::from(move |_| {
            close_dialog(&dialog_ref);

            if !is_controlled && let Some(controller) = controller.as_ref() {
                controller.close(&modal_id);
                return;
            }

            set_local_open.emit(false);
            if let Some(controller) = controller.as_ref() {
                controller.close(&modal_id);
            }
        })
    };

    let bg_close = {
        let close_action = close_action.clone();
        Callback::from(move |event: MouseEvent| {
            if should_ignore_target(&event) {
                event.stop_propagation();
                return;
            }
            close_action.emit(());
        })
    };

    let delete_btn_close = {
        let close_action = close_action.clone();
        Callback::from(move |_| close_action.emit(()))
    };
    let close_btn_close = {
        let close_action = close_action.clone();
        Callback::from(move |_| close_action.emit(()))
    };

    html! {
        <>
            <div onclick={open_action}>
                {props.trigger.clone()}
            </div>

            <DialogShell
                id={modal_id}
                classes={props.classes.clone()}
                is_active={is_active}
                set_is_active={set_local_open}
                dialog_ref={dialog_ref}
            >
                <div class="modal-background" onclick={bg_close}></div>

                <div class="modal-card">
                    <header class="modal-card-head">
                        <p class="modal-card-title" tabindex="-1" data-ybc-dialog-focus="true">{props.title.clone()}</p>
                        <button class="delete" type="button" aria-label="close" onclick={delete_btn_close}></button>
                    </header>
                    <section class="modal-card-body">
                        {props.body.clone()}
                    </section>
                    <footer class="modal-card-foot">
                        {props.footer.clone()}
                    </footer>
                </div>

                <button class="modal-close is-large ybc-modal-icon-close" type="button" aria-label="close" onclick={close_btn_close}>
                    {close_icon()}
                </button>
            </DialogShell>
        </>
    }
}

/// Backwards-compatible alias for `ModalCard`.
#[component(ModalCard2)]
pub fn modal_card2(props: &ModalCardProps) -> Html {
    html! { <ModalCard ..props.clone() /> }
}

#[derive(Properties, Debug, PartialEq)]
pub struct ModalControllerProviderProps {
    #[prop_or_default]
    pub children: Children,
}

/// Provides [`ModalControllerContext`] to descendants.
#[component]
pub fn ModalControllerProvider(props: &ModalControllerProviderProps) -> Html {
    let state = use_reducer(ModalControllerState::default);
    let controller = ModalController::new(state);

    html! {
        <ContextProvider<ModalControllerContext> context={controller}>
            { for props.children.iter() }
        </ContextProvider<ModalControllerContext>>
    }
}
