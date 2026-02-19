use derive_more::Display;
use std::rc::Rc;
use std::string::ToString;
use std::sync::atomic::{AtomicUsize, Ordering};
use yew::events::{KeyboardEvent, MouseEvent};
use yew::prelude::*;

use crate::Button;

static NAVBAR_AUTO_ID: AtomicUsize = AtomicUsize::new(1);
static NAVBAR_DROPDOWN_AUTO_ID: AtomicUsize = AtomicUsize::new(1);

fn next_navbar_id() -> String {
    format!("navbar-menu-{}", NAVBAR_AUTO_ID.fetch_add(1, Ordering::Relaxed))
}

fn next_navbar_dropdown_id() -> String {
    format!("navbar-dropdown-{}", NAVBAR_DROPDOWN_AUTO_ID.fetch_add(1, Ordering::Relaxed))
}

#[derive(Clone, Eq, PartialEq)]
pub struct NavBurgerCloserState {
    /// The total number of clicks received.
    pub total_clicks: u32,
}

/// The message type used by the `Navbar` component.
pub enum NavbarMsg {
    ToggleMenu,
    CloseEvent(Rc<NavBurgerCloserState>),
}

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct NavbarProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub classes: Classes,
    /// Make the navbar fixed to the top or bottom of the UI.
    #[prop_or_default]
    pub fixed: Option<NavbarFixed>,
    /// Seamlessly integrate the navbar in any visual context.
    ///
    /// [https://bulma.io/documentation/components/navbar/#transparent-navbar](https://bulma.io/documentation/components/navbar/#transparent-navbar)
    #[prop_or_default]
    pub transparent: bool,
    /// Sets **top** and **bottom** paddings with **1rem**, **left** and **right** paddings with **2rem**.
    ///
    /// [https://bulma.io/documentation/components/navbar/#navbar-helper-classes](https://bulma.io/documentation/components/navbar/#navbar-helper-classes)
    #[prop_or_default]
    pub spaced: bool,
    /// The contents of the navbar brand. The `navbar-burger` is automatically appended to the
    /// end of this content.
    ///
    /// [https://bulma.io/documentation/components/navbar/#navbar-brand](https://bulma.io/documentation/components/navbar/#navbar-brand)
    /// If true, the contents of the navbar will be wrapped in a container.
    #[prop_or_default]
    pub padded: bool,
    /// The contents of the `navbar-brand` section of the navbar.
    #[prop_or_default]
    pub navbrand: Option<Html>,
    /// The contents of the `navbar-start` section of the navbar.
    #[prop_or_default]
    pub navstart: Option<Html>,
    /// The contents of the `navbar-end` section of the navbar.
    #[prop_or_default]
    pub navend: Option<Html>,
    /// A bool controlling if the navbar should have a navbar burger for smaller viewports.
    #[prop_or_else(|| true)]
    pub navburger: bool,
    /// Extra classes for the navbar burger.
    #[prop_or_default]
    pub navburger_classes: Classes,
    /// Controlled open state for the mobile menu.
    #[prop_or_default]
    pub open: Option<bool>,
    /// Controlled setter for the mobile menu.
    #[prop_or_default]
    pub set_open: Option<Callback<bool>>,
    /// Called when the mobile menu opens.
    #[prop_or_default]
    pub on_open: Callback<()>,
    /// Called when the mobile menu closes.
    #[prop_or_default]
    pub on_close: Callback<()>,
    /// Allow closing the mobile menu with Escape.
    #[prop_or(true)]
    pub close_on_escape: bool,
    /// Optional menu id used by `aria-controls`.
    #[prop_or_default]
    pub menu_id: Option<AttrValue>,
    /// Optional `aria-label` for the nav container.
    #[prop_or_default]
    pub aria_label: AttrValue,
}

/// A responsive horizontal navbar that can support images, links, buttons, and dropdowns.
///
/// [https://bulma.io/documentation/components/navbar/](https://bulma.io/documentation/components/navbar/)
#[component(Navbar)]
pub fn navbar(props: &NavbarProps) -> Html {
    let internal_open = use_state(|| false);
    let is_controlled = props.open.is_some() && props.set_open.is_some();
    let is_menu_open = props.open.unwrap_or(*internal_open);

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
        let on_open = props.on_open.clone();
        let on_close = props.on_close.clone();
        let prev_open = use_mut_ref(move || is_menu_open);
        use_effect_with(is_menu_open, move |is_open| {
            let mut prev = prev_open.borrow_mut();
            if *prev != *is_open {
                if *is_open {
                    on_open.emit(());
                } else {
                    on_close.emit(());
                }
                *prev = *is_open;
            }
            || {}
        });
    }

    let closer_state = use_context::<Rc<NavBurgerCloserState>>();
    {
        let set_local_open = set_local_open.clone();
        let prev_clicks = use_mut_ref(|| closer_state.as_ref().map(|state| state.total_clicks));
        use_effect_with(closer_state, move |state| {
            let mut prev = prev_clicks.borrow_mut();
            let current = state.as_ref().map(|s| s.total_clicks);
            if prev.is_some() && current != *prev {
                set_local_open.emit(false);
            }
            *prev = current;
            || {}
        });
    }

    let auto_menu_id = use_state(|| AttrValue::from(next_navbar_id()));
    let menu_id = props.menu_id.clone().unwrap_or_else(|| (*auto_menu_id).clone());
    let aria_label = if props.aria_label.is_empty() {
        AttrValue::from("main navigation")
    } else {
        props.aria_label.clone()
    };

    let toggle_menu_action = {
        let set_local_open = set_local_open.clone();
        let is_menu_open = is_menu_open;
        Callback::from(move |_| set_local_open.emit(!is_menu_open))
    };

    let toggle_menu = {
        let toggle_menu_action = toggle_menu_action.clone();
        Callback::from(move |_event: MouseEvent| toggle_menu_action.emit(()))
    };

    let burger_on_keydown = {
        let toggle_menu_action = toggle_menu_action.clone();
        let set_local_open = set_local_open.clone();
        let close_on_escape = props.close_on_escape;
        Callback::from(move |event: KeyboardEvent| match event.key().as_str() {
            "Enter" | " " => {
                event.prevent_default();
                toggle_menu_action.emit(());
            }
            "Escape" if close_on_escape => {
                event.prevent_default();
                set_local_open.emit(false);
            }
            _ => {}
        })
    };

    // navbar classes
    let class = classes!(
        "navbar",
        props.classes.clone(),
        props.fixed.as_ref().map(ToString::to_string),
        props.transparent.then_some("is-transparent"),
        props.spaced.then_some("is-spaced"),
    );

    // navbar-menu classes
    let navclasses = classes!("navbar-menu", is_menu_open.then_some("is-active"));
    let burgerclasses = classes!("navbar-burger", props.navburger_classes.clone(), is_menu_open.then_some("is-active"));

    let navbrand = if let Some(navbrand) = &props.navbrand {
        html! {
            <div class="navbar-brand">
                {navbrand.clone()}
                {
                    if props.navburger {
                        html! {
                            <Button
                                classes={burgerclasses}
                                no_button_class={true}
                                aria_label={"menu"}
                                aria_controls={menu_id.clone()}
                                aria_expanded={Some(is_menu_open)}
                                aria_haspopup={"true"}
                                onclick={toggle_menu}
                                onkeydown={burger_on_keydown}
                            >
                                <span aria-hidden="true"></span>
                                <span aria-hidden="true"></span>
                                <span aria-hidden="true"></span>
                            </Button>
                        }
                    } else {
                        Html::default()
                    }
                }
            </div>
        }
    } else {
        Html::default()
    };

    let navstart = if let Some(navstart) = &props.navstart {
        html! {<div class="navbar-start">{navstart.clone()}</div>}
    } else {
        Html::default()
    };
    let navend = if let Some(navend) = &props.navend {
        html! {<div class="navbar-end">{navend.clone()}</div>}
    } else {
        Html::default()
    };
    let contents = html! {
        <>
            {navbrand}
            <div class={navclasses} id={menu_id}>
                {navstart}
                {navend}
            </div>
        </>
    };

    if props.padded {
        html! {
            <nav {class} role="navigation" aria-label={aria_label}>
                <div class="container">{contents}</div>
                {props.children.clone()}
            </nav>
        }
    } else {
        html! {
            <nav {class} role="navigation" aria-label={aria_label}>
                {contents}
                {props.children.clone()}
            </nav>
        }
    }
}

/// The 2 possible fixed positions available for a navbar.
///
/// [https://bulma.io/documentation/components/navbar/#fixed-navbar](https://bulma.io/documentation/components/navbar/#fixed-navbar)
///
/// NOTE WELL: in order to work properly, the root `html` or `body` element must be configured with
/// the corresponding `has-navbar-fixed-top` or `has-navbar-fixed-bottom` class.
#[derive(Clone, Debug, Display, PartialEq, Eq)]
pub enum NavbarFixed {
    #[display("is-fixed-top")]
    Top,
    #[display("is-fixed-bottom")]
    Bottom,
}

//////////////////////////////////////////////////////////////////////////////
//////////////////////////////////////////////////////////////////////////////

/// The two HTML tags allowed for a navbar-item.
///
/// [https://bulma.io/documentation/components/navbar/#navbar-item](https://bulma.io/documentation/components/navbar/#navbar-item)
#[derive(Clone, Debug, Display, PartialEq, Eq)]
pub enum NavbarItemTag {
    #[display("a")]
    A,
    #[display("div")]
    Div,
}

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct NavbarItemProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub classes: Classes,
    /// The HTML tag to use for this component.
    #[prop_or_else(|| NavbarItemTag::Div)]
    pub tag: NavbarItemTag,
    /// Add the `has-dropdown` class to this element, indicating that it is the parent
    /// of a dropdown menu.
    #[prop_or_default]
    pub has_dropdown: bool,
    /// Turn this into a full-width element.
    #[prop_or_default]
    pub expanded: bool,
    /// Add a bottom border on hover, and show the bottom border using `is_active=true`.
    #[prop_or_default]
    pub tab: bool,
    /// Show the bottom border when `is_tab=true`.
    #[prop_or_default]
    pub active: bool,
    /// An optional `href` for when this element is using the `a` tag.
    #[prop_or_default]
    pub href: Option<String>,
    /// An optional `rel` for when this element is using the `a` tag.
    #[prop_or_default]
    pub rel: Option<String>,
    /// An optional `target` for when this element is using the `a` tag.
    #[prop_or_default]
    pub target: Option<String>,
}

/// A single element of the navbar.
///
/// [https://bulma.io/documentation/components/navbar/](https://bulma.io/documentation/components/navbar/)
#[component(NavbarItem)]
pub fn navbar_item(props: &NavbarItemProps) -> Html {
    let class = classes!(
        "navbar-item",
        props.classes.clone(),
        props.has_dropdown.then_some("has-dropdown"),
        props.expanded.then_some("is-expanded"),
        props.tab.then_some("is-tab"),
        props.active.then_some("is-active"),
    );
    match props.tag {
        NavbarItemTag::A => {
            html! {
                <a
                    {class}
                    href={props.href.clone().unwrap_or_default()}
                    rel={props.rel.clone().unwrap_or_default()}
                    target={props.target.clone().unwrap_or_default()}
                >
                    {props.children.clone()}
                </a>
            }
        }
        NavbarItemTag::Div => {
            html! {
                <div {class}>
                    {props.children.clone()}
                </div>
            }
        }
    }
}

//////////////////////////////////////////////////////////////////////////////
//////////////////////////////////////////////////////////////////////////////

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct NavbarDividerProps {
    #[prop_or_default]
    pub classes: Classes,
}

/// An element to display a horizontal rule in a navbar-dropdown.
///
/// [https://bulma.io/documentation/components/navbar/#dropdown-menu](https://bulma.io/documentation/components/navbar/#dropdown-menu)
#[component(NavbarDivider)]
pub fn navbar_divider(props: &NavbarDividerProps) -> Html {
    html! { <hr class={classes!("navbar-divider", props.classes.clone())} /> }
}

//////////////////////////////////////////////////////////////////////////////
//////////////////////////////////////////////////////////////////////////////

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct NavbarDropdownProps {
    /// The content of the dropdown; these should all be `NavbarItems` & `NavbarDividers`.
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub classes: Classes,
    /// The contents of the navbar-link used for triggering the dropdown menu.
    pub navlink: Html,
    /// Make this dropdown triggerable based on hover.
    #[prop_or_default]
    pub hoverable: bool,
    /// Configure this menu to be a dropup.
    #[prop_or_default]
    pub dropup: bool,
    /// Render the contents of this dropdown to the right.
    #[prop_or_default]
    pub right: bool,
    /// Remove the arrow from the dropdown menu trigger.
    #[prop_or_default]
    pub arrowless: bool,
    /// Use the boxed style for the dropdown, typically coupled with a transparent navbar.
    #[prop_or_default]
    pub boxed: bool,
    /// Controlled open state.
    #[prop_or_default]
    pub open: Option<bool>,
    /// Controlled open state setter.
    #[prop_or_default]
    pub set_open: Option<Callback<bool>>,
    /// Callback emitted when opened.
    #[prop_or_default]
    pub on_open: Callback<()>,
    /// Callback emitted when closed.
    #[prop_or_default]
    pub on_close: Callback<()>,
    /// Allow closing with Escape.
    #[prop_or(true)]
    pub close_on_escape: bool,
    /// Allow closing by clicking outside.
    #[prop_or(true)]
    pub close_on_click_outside: bool,
    /// Optional id used to build ARIA links.
    #[prop_or_default]
    pub id: Option<AttrValue>,
}

/// A navbar dropdown menu, which can include navbar items and dividers.
///
/// This component is a composite of all of the elements needed in order to properly generate
/// a navbar dropdown component.
///
/// [https://bulma.io/documentation/components/navbar/#dropdown-menu](https://bulma.io/documentation/components/navbar/#dropdown-menu)
#[component(NavbarDropdown)]
pub fn navbar_dropdown(props: &NavbarDropdownProps) -> Html {
    let internal_open = use_state(|| false);
    let is_controlled = props.open.is_some() && props.set_open.is_some();
    let is_menu_active = props.open.unwrap_or(*internal_open);

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
        let on_open = props.on_open.clone();
        let on_close = props.on_close.clone();
        let prev_open = use_mut_ref(move || is_menu_active);
        use_effect_with(is_menu_active, move |is_open| {
            let mut prev = prev_open.borrow_mut();
            if *prev != *is_open {
                if *is_open {
                    on_open.emit(());
                } else {
                    on_close.emit(());
                }
                *prev = *is_open;
            }
            || {}
        });
    }

    let auto_id = use_state(|| AttrValue::from(next_navbar_dropdown_id()));
    let root_id = props.id.clone().unwrap_or_else(|| (*auto_id).clone());
    let menu_id = AttrValue::from(format!("{}-menu", root_id));

    // navbar-item classes
    let class = classes!(
        "navbar-item",
        "has-dropdown",
        props.classes.clone(),
        props.dropup.then_some("has-dropdown-up"),
        props.hoverable.then_some("is-hoverable"),
        (!props.hoverable && is_menu_active).then_some("is-active"),
    );

    // navbar-dropdown classes
    let dropclasses = classes!("navbar-dropdown", props.right.then_some("is-right"), props.boxed.then_some("is-boxed"),);

    // navbar-link classes
    let linkclasses = classes!("navbar-link", props.arrowless.then_some("is-arrowless"));

    let on_trigger_click = {
        let set_local_open = set_local_open.clone();
        let hoverable = props.hoverable;
        let is_menu_active = is_menu_active;
        Callback::from(move |event: MouseEvent| {
            if hoverable {
                return;
            }
            event.prevent_default();
            set_local_open.emit(!is_menu_active);
        })
    };

    let on_trigger_keydown = {
        let set_local_open = set_local_open.clone();
        let hoverable = props.hoverable;
        let close_on_escape = props.close_on_escape;
        Callback::from(move |event: KeyboardEvent| match event.key().as_str() {
            "Enter" | " " | "ArrowDown" if !hoverable => {
                event.prevent_default();
                set_local_open.emit(true);
            }
            "Escape" if close_on_escape => {
                event.prevent_default();
                set_local_open.emit(false);
            }
            _ => {}
        })
    };

    let overlay = if !props.hoverable && is_menu_active && props.close_on_click_outside {
        let set_local_open = set_local_open.clone();
        html! {
            <div
                onclick={Callback::from(move |_| set_local_open.emit(false))}
                style="z-index:10;background-color:rgba(0,0,0,0);position:fixed;top:0;bottom:0;left:0;right:0;"
            ></div>
        }
    } else {
        Html::default()
    };

    html! {
        <div id={root_id} {class}>
            {overlay}
            <a
                class={linkclasses}
                role="button"
                tabindex="0"
                aria-haspopup="true"
                aria-controls={menu_id.clone()}
                aria-expanded={if is_menu_active { "true" } else { "false" }}
                onclick={on_trigger_click}
                onkeydown={on_trigger_keydown}
            >
                {props.navlink.clone()}
            </a>
            <div id={menu_id} class={dropclasses} role="menu">
                {props.children.clone()}
            </div>
        </div>
    }
}
