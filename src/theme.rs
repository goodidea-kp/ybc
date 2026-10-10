//! Light and dark colour schemes for Bulma 1.x.
//!
//! Bulma 1.x already follows the operating system: under
//! `prefers-color-scheme: dark` it redefines its `--bulma-*` variables. It also
//! honours `data-theme="light"` / `data-theme="dark"` on any ancestor, which
//! wins over the OS. This module is the one place that attribute is written:
//!
//! - [`Theme`] is the choice: follow the system, or force light or dark.
//! - [`apply`] puts the choice on `<html>`; [`Theme::System`] removes the
//!   attribute so Bulma's media query decides again.
//! - [`store`] / [`stored`] keep the choice in `localStorage` under
//!   [`THEME_STORAGE_KEY`], per browser.
//! - [`use_theme`] is the hook; [`ThemeToggle`] the control: three buttons with
//!   visible text and `aria-pressed`, never a colour-only switch.
//!
//! To avoid a flash of the wrong scheme before the WASM loads, the page applies
//! the stored choice itself from an inline script in `<head>`:
//!
//! ```html
//! <script>
//!   try {
//!     var theme = localStorage.getItem("ybc.theme");
//!     if (theme === "light" || theme === "dark") {
//!       document.documentElement.setAttribute("data-theme", theme);
//!     }
//!   } catch (ignored) {}
//! </script>
//! ```
//!
//! Outside a browser (server-side rendering, host tests) reading the stored
//! choice yields [`Theme::System`] and applying it does nothing.

use yew::prelude::*;

/// The `localStorage` key the choice is kept under. A page's no-flash script
/// must read the same key.
pub const THEME_STORAGE_KEY: &str = "ybc.theme";

/// The attribute Bulma 1.x reads on `<html>`.
pub const THEME_ATTRIBUTE: &str = "data-theme";

/// Which colour scheme the page shows.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum Theme {
    /// Follow the operating system (`prefers-color-scheme`).
    #[default]
    System,
    Light,
    Dark,
}

impl Theme {
    /// Every choice, in the order a toggle offers them.
    pub const ALL: [Theme; 3] = [Theme::System, Theme::Light, Theme::Dark];

    /// The value written to `data-theme`; `None` for [`Theme::System`], whose
    /// attribute is removed instead.
    pub fn attribute_value(self) -> Option<&'static str> {
        match self {
            Theme::System => None,
            Theme::Light => Some("light"),
            Theme::Dark => Some("dark"),
        }
    }

    /// The value kept in `localStorage`.
    pub fn storage_value(self) -> &'static str {
        self.attribute_value().unwrap_or("system")
    }

    /// Reads a stored value back. Anything unknown (a value from a newer
    /// version, a hand edit) means [`Theme::System`], never an error.
    pub fn from_storage_value(value: &str) -> Theme {
        match value {
            "light" => Theme::Light,
            "dark" => Theme::Dark,
            _ => Theme::System,
        }
    }

    /// The visible label of the toggle button.
    pub fn label(self) -> &'static str {
        match self {
            Theme::System => "System",
            Theme::Light => "Light",
            Theme::Dark => "Dark",
        }
    }

    /// Font Awesome 6/7 solid icon shown beside the label.
    pub fn icon(self) -> &'static str {
        match self {
            Theme::System => "fa-circle-half-stroke",
            Theme::Light => "fa-sun",
            Theme::Dark => "fa-moon",
        }
    }
}

/// Puts `theme` on `<html>`: sets `data-theme`, or removes it for
/// [`Theme::System`]. Does nothing outside a browser.
pub fn apply(theme: Theme) {
    browser::set_root_attribute(theme.attribute_value());
}

/// Remembers `theme` in `localStorage`. A browser that refuses storage
/// (private mode, blocked site data) keeps the choice for this page only.
pub fn store(theme: Theme) {
    browser::write_storage(theme.storage_value());
}

/// The remembered choice, or [`Theme::System`] when there is none or storage
/// cannot be read.
pub fn stored() -> Theme {
    browser::read_storage().map_or(Theme::System, |value| Theme::from_storage_value(&value))
}

/// What [`use_theme`] returns: the current choice and a way to change it.
#[derive(Clone, Debug, PartialEq)]
pub struct UseThemeHandle {
    pub theme: Theme,
    /// Applies, stores and re-renders with the new choice.
    pub set: Callback<Theme>,
}

/// The current colour scheme choice, starting from the stored one. Setting it
/// applies it to `<html>` and stores it.
#[hook]
pub fn use_theme() -> UseThemeHandle {
    let state = use_state(stored);
    let set = {
        let state = state.clone();
        Callback::from(move |theme: Theme| {
            apply(theme);
            store(theme);
            state.set(theme);
        })
    };
    UseThemeHandle { theme: *state, set }
}

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct ThemeToggleProps {
    /// The group's accessible name.
    #[prop_or_else(|| AttrValue::from("Colour scheme"))]
    pub label: AttrValue,
    /// Called after a choice is applied and stored.
    #[prop_or_default]
    pub onchange: Option<Callback<Theme>>,
    /// Extra classes on the `buttons` group.
    #[prop_or_default]
    pub classes: Classes,
    /// `data-testid` of the group; each button gets `<testid>-<storage value>`.
    #[prop_or_default]
    pub testid: Option<AttrValue>,
}

/// Three buttons -- System, Light, Dark -- with visible text and
/// `aria-pressed` on the chosen one. Choosing applies the scheme at once and
/// remembers it in this browser.
#[component(ThemeToggle)]
pub fn theme_toggle(props: &ThemeToggleProps) -> Html {
    let handle = use_theme();
    let buttons = Theme::ALL.iter().map(|&theme| {
        let onclick = {
            let (set, onchange) = (handle.set.clone(), props.onchange.clone());
            Callback::from(move |_: MouseEvent| {
                set.emit(theme);
                if let Some(onchange) = &onchange {
                    onchange.emit(theme);
                }
            })
        };
        let pressed = handle.theme == theme;
        let testid = props.testid.as_ref().map(|testid| format!("{testid}-{}", theme.storage_value()));
        html! {
            <button type="button" key={theme.storage_value()}
                    class={classes!("button", "is-small", pressed.then_some("is-selected"), pressed.then_some("is-link"))}
                    aria-pressed={pressed.to_string()} data-testid={testid} {onclick}>
                <span class="icon is-small" aria-hidden="true"><i class={classes!("fas", theme.icon())}></i></span>
                <span>{ theme.label() }</span>
            </button>
        }
    });
    html! {
        <div class={classes!("buttons", "has-addons", props.classes.clone())} role="group"
             aria-label={props.label.clone()} data-testid={props.testid.clone()}>
            { for buttons }
        </div>
    }
}

/// The DOM and storage calls, compiled only for the browser so that host
/// tests and server-side rendering never reach a wasm-bindgen import.
#[cfg(target_arch = "wasm32")]
mod browser {
    use super::{THEME_ATTRIBUTE, THEME_STORAGE_KEY};

    pub fn set_root_attribute(value: Option<&str>) {
        let Some(root) = web_sys::window()
            .and_then(|window| window.document())
            .and_then(|document| document.document_element())
        else {
            return;
        };
        // Neither call can fail for a valid attribute name; a refusal leaves
        // the page on its previous scheme, which is still readable.
        let _ = match value {
            Some(value) => root.set_attribute(THEME_ATTRIBUTE, value),
            None => root.remove_attribute(THEME_ATTRIBUTE),
        };
    }

    fn storage() -> Option<web_sys::Storage> {
        web_sys::window()?.local_storage().ok().flatten()
    }

    pub fn read_storage() -> Option<String> {
        storage()?.get_item(THEME_STORAGE_KEY).ok().flatten()
    }

    pub fn write_storage(value: &str) {
        if let Some(storage) = storage() {
            let _ = storage.set_item(THEME_STORAGE_KEY, value);
        }
    }
}

#[cfg(not(target_arch = "wasm32"))]
mod browser {
    pub fn set_root_attribute(_value: Option<&str>) {}

    pub fn read_storage() -> Option<String> {
        None
    }

    pub fn write_storage(_value: &str) {}
}
