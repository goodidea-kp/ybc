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
    let parent_ctx = use_context::<TabsContext>();
    let has_parent = parent_ctx.is_some();

    let internal_active = use_state(|| props.default_active);
    let is_controlled = props.active.is_some() && props.set_active.is_some();
    let active = parent_ctx
        .as_ref()
        .map(|ctx| ctx.active)
        .unwrap_or_else(|| props.active.unwrap_or(*internal_active));

    let set_active = if let Some(ref ctx) = parent_ctx {
        ctx.set_active.clone()
    } else {
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

    let on_change = props.on_change.clone();
    let prev_active = use_mut_ref(move || active);
    use_effect_with(active, move |active| {
        if !has_parent {
            let mut prev = prev_active.borrow_mut();
            if *prev != *active {
                on_change.emit(*active);
                *prev = *active;
            }
        }
        || {}
    });

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

    if has_parent {
        html! {
            <div {class}>
                <ul id={list_id} role="tablist" aria-label={aria_label}>
                    {props.children.clone()}
                </ul>
            </div>
        }
    } else {
        let context = TabsContext { active, set_active };
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
}

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct TabsProviderProps {
    #[prop_or_default]
    pub children: Children,
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

/// A wrapper component providing state context for [`Tabs`], [`TabItem`] and [`TabPanel`].
/// Use this component to separate the tab list navigation from the tab panel contents.
#[component(TabsProvider)]
pub fn tabs_provider(props: &TabsProviderProps) -> Html {
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

    let context = TabsContext { active, set_active };

    html! {
        <ContextProvider<TabsContext> {context}>
            {props.children.clone()}
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
            } else if move_to_sibling_tab(&event, &key) {
                event.prevent_default();
            }
        })
    };

    html! {
        <li class={classes!(props.classes.clone(), is_active.then_some("is-active"))} role="presentation">
            <a
                id={tab_id.to_string()}
                class="ybc-tab-button"
                role="tab"
                aria-selected={if is_active { Some("true") } else { Some("false") }}
                aria-disabled={props.disabled.then_some("true")}
                aria-controls={panel_id}
                aria-label={aria_label}
                tabindex={if is_active { Some("0") } else { Some("-1") }}
                onclick={on_click}
                onkeydown={on_keydown}
            >
                {props.children.clone()}
            </a>
        </li>
    }
}

/// Where an arrow key moves focus in a tab list of `count` tabs, from the tab
/// at `current`: the next or previous one (wrapping round), the first on Home,
/// the last on End. `None` for any other key, or an empty list.
///
/// Only the selected tab is in the Tab order (`tabindex="0"`, the rest `-1`),
/// so without this a keyboard could never reach another tab (WCAG 2.1.1; the
/// WAI-ARIA tabs pattern).
pub fn sibling_tab_index(key: &str, current: usize, count: usize) -> Option<usize> {
    let last = count.checked_sub(1)?;
    match key {
        "ArrowRight" => Some(if current >= last { 0 } else { current + 1 }),
        "ArrowLeft" => Some(if current == 0 || current > last { last } else { current - 1 }),
        "Home" => Some(0),
        "End" => Some(last),
        _ => None,
    }
}

/// Moves focus to the tab `key` points at in the same tab list and selects it
/// (automatic activation: the tab is clicked, as a pointer would). Disabled
/// tabs are skipped. `true` when the key was one of the list's own.
fn move_to_sibling_tab(event: &KeyboardEvent, key: &str) -> bool {
    use wasm_bindgen::JsCast;
    // Yew delegates listeners to the app's root element, so `current_target()`
    // is that root, not the tab. Start from the element the key event came
    // from (the tab itself, or a child such as an icon) and walk up to the tab.
    let Some(origin) = event.target().and_then(|target| target.dyn_into::<web_sys::Element>().ok()) else {
        return false;
    };
    let Some(current) = origin
        .closest("[role=tab]")
        .ok()
        .flatten()
        .and_then(|tab| tab.dyn_into::<web_sys::HtmlElement>().ok())
    else {
        return false;
    };
    let Some(list) = current.closest("[role=tablist]").ok().flatten() else {
        return false;
    };
    let Ok(found) = list.query_selector_all("[role=tab]:not([aria-disabled=true])") else {
        return false;
    };
    let tabs: Vec<web_sys::HtmlElement> = (0..found.length())
        .filter_map(|position| found.item(position))
        .filter_map(|node| node.dyn_into::<web_sys::HtmlElement>().ok())
        .collect();
    let Some(current_position) = tabs.iter().position(|tab| *tab == current) else {
        return false;
    };
    let Some(target) = sibling_tab_index(key, current_position, tabs.len()).and_then(|position| tabs.get(position)) else {
        return false;
    };
    let _ = target.focus();
    target.click();
    true
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
