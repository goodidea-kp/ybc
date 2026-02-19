use std::sync::atomic::{AtomicUsize, Ordering};

use yew::events::{KeyboardEvent, MouseEvent};
use yew::prelude::*;

use crate::{Alignment, Size};

static TABS_AUTO_ID: AtomicUsize = AtomicUsize::new(1);

fn next_tabs_id() -> String {
    format!("tabs-{}", TABS_AUTO_ID.fetch_add(1, Ordering::Relaxed))
}

#[derive(Clone, PartialEq)]
struct TabsContext {
    active: usize,
    set_active: Callback<usize>,
}

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct TabsProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub classes: Classes,
    /// The alignment of this component.
    #[prop_or_default]
    pub alignment: Option<Alignment>,
    /// The size of this component.
    #[prop_or_default]
    pub size: Option<Size>,
    /// Add a more classic style with borders to this component.
    #[prop_or_default]
    pub boxed: bool,
    /// Add the "radio button" style to the elements of this component.
    #[prop_or_default]
    pub toggle: bool,
    /// Make the tab elements of this component rounded.
    #[prop_or_default]
    pub rounded: bool,
    /// Make this component fullwidth.
    #[prop_or_default]
    pub fullwidth: bool,
    /// Optional id for the internal tablist.
    #[prop_or_default]
    pub id: Option<AttrValue>,
    /// Optional label for assistive technologies.
    #[prop_or_default]
    pub aria_label: AttrValue,
    /// Controlled active tab index.
    #[prop_or_default]
    pub active: Option<usize>,
    /// Controlled active tab index setter.
    #[prop_or_default]
    pub set_active: Option<Callback<usize>>,
    /// Default active index for uncontrolled mode.
    #[prop_or_default]
    pub default_active: usize,
    /// Callback emitted when the active tab index changes.
    #[prop_or_default]
    pub on_change: Callback<usize>,
}

/// Simple responsive horizontal navigation tabs, with different styles.
///
/// [https://bulma.io/documentation/components/tabs/](https://bulma.io/documentation/components/tabs/)
///
/// For accessible keyboard controls and controlled/uncontrolled state handling,
/// pair this component with [`TabItem`] and optionally [`TabPanel`].
#[component(Tabs)]
pub fn tabs(props: &TabsProps) -> Html {
    let internal_active = use_state(|| props.default_active);
    let is_controlled = props.active.is_some() && props.set_active.is_some();
    let active = props.active.unwrap_or(*internal_active);

    let set_active = {
        let internal_active = internal_active.clone();
        let set_active = props.set_active.clone();
        Callback::from(move |value: usize| {
            if is_controlled {
                if let Some(set_active) = set_active.as_ref() {
                    set_active.emit(value);
                }
            } else {
                internal_active.set(value);
            }
        })
    };

    {
        let on_change = props.on_change.clone();
        let prev_active = use_mut_ref(move || active);
        use_effect_with(active, move |active| {
            let mut prev = prev_active.borrow_mut();
            if *prev != *active {
                on_change.emit(*active);
                *prev = *active;
            }
            || {}
        });
    }

    let class = classes!(
        "tabs",
        props.classes.clone(),
        props.alignment.as_ref().map(ToString::to_string),
        props.size.as_ref().map(ToString::to_string),
        props.boxed.then_some("is-boxed"),
        props.toggle.then_some("is-toggle"),
        props.rounded.then_some("is-rounded"),
        props.fullwidth.then_some("is-fullwidth"),
    );

    let auto_id = use_state(|| AttrValue::from(next_tabs_id()));
    let list_id = props.id.clone().unwrap_or_else(|| (*auto_id).clone());
    let aria_label = (!props.aria_label.is_empty()).then_some(props.aria_label.clone());
    let context = TabsContext {
        active,
        set_active: set_active.clone(),
    };

    html! {
        <ContextProvider<TabsContext> {context}>
            <div {class}>
                <ul id={list_id} role="tablist" aria-label={aria_label}>
                    {props.children.clone()}
                </ul>
            </div>
        </ContextProvider<TabsContext>>
    }
}

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct TabItemProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub classes: Classes,
    /// Zero-based tab index used by the tab state manager.
    pub index: usize,
    /// Disable this tab.
    #[prop_or_default]
    pub disabled: bool,
    /// Optional id for the underlying `button[role=tab]`.
    #[prop_or_default]
    pub id: Option<AttrValue>,
    /// Optional `aria-controls` target id for this tab.
    #[prop_or_default]
    pub panel_id: Option<AttrValue>,
    /// Optional aria label for this tab button.
    #[prop_or_default]
    pub aria_label: AttrValue,
    /// Callback emitted after this tab gets selected.
    #[prop_or_default]
    pub on_select: Callback<usize>,
}

/// Accessible tab item for use inside [`Tabs`].
#[component(TabItem)]
pub fn tab_item(props: &TabItemProps) -> Html {
    let Some(ctx) = use_context::<TabsContext>() else {
        return html! {
            <li class={props.classes.clone()}>{props.children.clone()}</li>
        };
    };

    let is_active = ctx.active == props.index;
    let tab_id = props.id.clone().unwrap_or_else(|| AttrValue::from(format!("tab-{}", props.index)));
    let aria_label = (!props.aria_label.is_empty()).then_some(props.aria_label.clone());
    let panel_id = props.panel_id.clone();

    let select_tab = {
        let set_active = ctx.set_active.clone();
        let on_select = props.on_select.clone();
        let index = props.index;
        let disabled = props.disabled;
        Callback::from(move |_| {
            if disabled {
                return;
            }
            set_active.emit(index);
            on_select.emit(index);
        })
    };

    let on_click = {
        let select_tab = select_tab.clone();
        Callback::from(move |_event: MouseEvent| select_tab.emit(()))
    };

    let on_keydown = {
        let select_tab = select_tab.clone();
        Callback::from(move |event: KeyboardEvent| {
            let key = event.key();
            if key == "Enter" || key == " " {
                event.prevent_default();
                select_tab.emit(());
            }
        })
    };

    html! {
        <li class={classes!(props.classes.clone(), is_active.then_some("is-active"))} role="presentation">
            <button
                id={tab_id}
                class="ybc-tab-button"
                type="button"
                role="tab"
                aria-selected={if is_active { "true" } else { "false" }}
                aria-controls={panel_id}
                aria-label={aria_label}
                tabindex={if is_active { "0" } else { "-1" }}
                disabled={props.disabled}
                onclick={on_click}
                onkeydown={on_keydown}
            >
                {props.children.clone()}
            </button>
        </li>
    }
}

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct TabPanelProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub classes: Classes,
    /// Zero-based tab index linked with `TabItem::index`.
    pub index: usize,
    /// Optional id for this panel.
    #[prop_or_default]
    pub id: Option<AttrValue>,
    /// Optional tab id referenced by `aria-labelledby`.
    #[prop_or_default]
    pub tab_id: Option<AttrValue>,
}

/// Accessible tab panel that syncs visibility with [`Tabs`] state.
#[component(TabPanel)]
pub fn tab_panel(props: &TabPanelProps) -> Html {
    let is_active = use_context::<TabsContext>().map(|ctx| ctx.active == props.index).unwrap_or(true);

    html! {
        <div
            class={props.classes.clone()}
            id={props.id.clone()}
            role="tabpanel"
            aria-labelledby={props.tab_id.clone()}
            hidden={!is_active}
            tabindex="0"
        >
            {props.children.clone()}
        </div>
    }
}
