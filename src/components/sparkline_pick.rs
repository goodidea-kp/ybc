//! Which point of a [`Sparkline`](super::sparkline::Sparkline) is shown, and what is announced.
//!
//! The shown point is kept as how many points it is before the latest, so a poll that appends a
//! point keeps the same one shown. Only a move by the keyboard is announced: focus, a hover, a tap
//! and a poll change the tooltip alone, so a screen reader is not talked over.
//!
//! Every handler reads the key, or the line's own box through its `NodeRef`; none reads
//! `current_target()`, which is the app root under Yew's delegated listeners.
use std::rc::Rc;

use wasm_bindgen::JsCast;
use wasm_bindgen::closure::Closure;
use yew::prelude::*;

use super::sparkline_parts::{back_from_index, index_from_back, moved_by_key, nearest_index};

/// The interaction state of one sparkline.
#[derive(Clone, PartialEq)]
pub(super) struct Pick {
    /// How many points before the latest the shown one is; `None` while none is shown.
    back: UseStateHandle<Option<usize>>,
    /// What the live region says: written only when the keyboard moves to a point.
    announced: UseStateHandle<String>,
}

impl Pick {
    /// The index of the shown point among `count` drawn points.
    pub fn shown(&self, count: usize) -> Option<usize> {
        (*self.back).and_then(|back| index_from_back(back, count))
    }

    pub fn announced(&self) -> String {
        (*self.announced).clone()
    }

    fn show(&self, index: usize, count: usize) {
        self.back.set(Some(back_from_index(index, count)));
    }

    fn close(&self) {
        self.back.set(None);
        self.announced.set(String::new());
    }

    /// Keyboard focus shows the latest point, silently: the image's name already says it.
    pub fn on_focus(&self) -> Callback<FocusEvent> {
        let pick = self.clone();
        Callback::from(move |_: FocusEvent| pick.back.set(Some(0)))
    }

    pub fn on_blur(&self) -> Callback<FocusEvent> {
        let pick = self.clone();
        Callback::from(move |_: FocusEvent| pick.close())
    }

    /// Arrows, Home and End move between points and announce the point moved to.
    pub fn on_key(&self, spoken: Rc<Vec<String>>) -> Callback<KeyboardEvent> {
        let pick = self.clone();
        Callback::from(move |event: KeyboardEvent| {
            let count = spoken.len();
            let Some(next) = moved_by_key(&event.key(), pick.shown(count), count) else {
                return;
            };
            event.prevent_default();
            pick.show(next, count);
            pick.announced.set(spoken[next].clone());
        })
    }

    /// A hover or a tap shows the nearest point, measured against the line's own box.
    pub fn on_pointer(&self, line_ref: &NodeRef, positions: Rc<Vec<(f64, f64)>>, drawn_width: f64) -> Callback<MouseEvent> {
        let pick = self.clone();
        let line_ref = line_ref.clone();
        Callback::from(move |event: MouseEvent| {
            let Some(line) = line_ref.cast::<web_sys::Element>() else {
                return;
            };
            let bounds = line.get_bounding_client_rect();
            if bounds.width() <= 0.0 {
                return;
            }
            let x = (f64::from(event.client_x()) - bounds.left()) * drawn_width / bounds.width();
            if let Some(index) = nearest_index(x, &positions) {
                pick.show(index, positions.len());
            }
        })
    }

    /// The pointer leaving closes what a hover opened, never what the keyboard holds open.
    pub fn on_leave(&self, line_ref: &NodeRef) -> Callback<MouseEvent> {
        let pick = self.clone();
        let line_ref = line_ref.clone();
        Callback::from(move |_: MouseEvent| {
            let focused = line_ref
                .cast::<web_sys::Element>()
                .is_some_and(|line| line.matches(":focus").unwrap_or(false));
            if !focused {
                pick.close();
            }
        })
    }
}

#[hook]
pub(super) fn use_pick() -> Pick {
    let pick = Pick {
        back: use_state_eq(|| None::<usize>),
        announced: use_state_eq(String::new),
    };
    use_escape_closes(pick.clone());
    pick
}

/// While a point is shown, Escape anywhere closes it: a hover tooltip is dismissible without
/// moving the pointer (WCAG 1.4.13). The listener lives only while something is shown.
#[hook]
fn use_escape_closes(pick: Pick) {
    let open = pick.back.is_some();
    use_effect_with(open, move |open| {
        let listener = open.then(|| EscapeListener::attach(pick));
        move || drop(listener)
    });
}

struct EscapeListener {
    document: web_sys::Document,
    on_key: Closure<dyn Fn(web_sys::KeyboardEvent)>,
}

impl EscapeListener {
    fn attach(pick: Pick) -> Option<Self> {
        let document = web_sys::window()?.document()?;
        let on_key = Closure::<dyn Fn(web_sys::KeyboardEvent)>::new(move |event: web_sys::KeyboardEvent| {
            if event.key() == "Escape" {
                pick.close();
            }
        });
        document
            .add_event_listener_with_callback("keydown", on_key.as_ref().unchecked_ref())
            .ok()?;
        Some(Self { document, on_key })
    }
}

impl Drop for EscapeListener {
    fn drop(&mut self) {
        let _ = self
            .document
            .remove_event_listener_with_callback("keydown", self.on_key.as_ref().unchecked_ref());
    }
}

/// Puts the sparkline's `<style>` in the page's `<head>` once, however many sparklines it draws.
#[hook]
pub(super) fn use_style_once(id: &'static str, css: &'static str) {
    use_effect_with((), move |_| {
        install_style(id, css);
    });
}

fn install_style(id: &str, css: &str) -> Option<()> {
    let document = web_sys::window()?.document()?;
    if document.get_element_by_id(id).is_some() {
        return Some(());
    }
    let style = document.create_element("style").ok()?;
    style.set_id(id);
    style.set_text_content(Some(css));
    document.head()?.append_child(&style).ok()?;
    Some(())
}
