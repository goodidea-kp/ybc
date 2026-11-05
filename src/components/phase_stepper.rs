//! An ordered row of phases, each marked done, current, waiting or failed.
//!
//! The phases and their words come from the caller; the stepper only draws them.
//!
//! ```rust,ignore
//! let phases = vec![
//!     ybc::Phase::new("schema", "Copy schema", ybc::PhaseState::Done),
//!     ybc::Phase::new("rows", "Copy rows", ybc::PhaseState::Current),
//!     ybc::Phase::new("verify", "Verify", ybc::PhaseState::Waiting).with_reason("Starts when every row is copied"),
//! ];
//! html! {
//!     <ybc::PhaseStepper aria_label="Import progress" {phases} testid="import-phases" />
//! }
//! ```
use yew::prelude::*;

/// Where one phase of a [`PhaseStepper`] stands.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum PhaseState {
    /// Finished.
    Done,
    /// Under way now. At most one phase should be current; it gets `aria-current="step"`.
    Current,
    /// Not started yet, or held up; say why with [`Phase::with_reason`].
    Waiting,
    /// Stopped with an error.
    Failed,
}

impl PhaseState {
    /// Font Awesome 6/7 solid icon of the state. Each state has its own shape, so colour is
    /// never the only signal (WCAG 1.4.1).
    pub fn icon(self) -> &'static str {
        match self {
            PhaseState::Done => "fas fa-circle-check",
            PhaseState::Current => "fas fa-circle-dot",
            PhaseState::Waiting => "fas fa-clock",
            PhaseState::Failed => "fas fa-circle-xmark",
        }
    }

    /// The state as rendered in `data-state`, for tests: `done`, `current`, `waiting`, `failed`.
    pub fn code(self) -> &'static str {
        match self {
            PhaseState::Done => "done",
            PhaseState::Current => "current",
            PhaseState::Waiting => "waiting",
            PhaseState::Failed => "failed",
        }
    }

    /// Bulma 1 colour of the icon, the variant meant for text on the page surface (so it keeps
    /// its contrast in light and dark schemes), with a light-scheme fallback outside Bulma 1.
    fn color(self) -> &'static str {
        match self {
            PhaseState::Done => "var(--bulma-success-on-scheme, #257942)",
            PhaseState::Current => "var(--bulma-link-on-scheme, #2a5fd6)",
            PhaseState::Waiting => "var(--bulma-text-weak, #6b6b6b)",
            PhaseState::Failed => "var(--bulma-danger-on-scheme, #c0172f)",
        }
    }
}

/// The word a screen reader hears after each phase's label, one per state. The defaults are
/// plain English; pass the words your own pages already use for these states.
#[derive(Clone, Debug, PartialEq)]
pub struct PhaseStateWords {
    pub done: AttrValue,
    pub current: AttrValue,
    pub waiting: AttrValue,
    pub failed: AttrValue,
}

impl Default for PhaseStateWords {
    fn default() -> Self {
        Self {
            done: AttrValue::from("Done"),
            current: AttrValue::from("In progress"),
            waiting: AttrValue::from("Waiting"),
            failed: AttrValue::from("Failed"),
        }
    }
}

impl PhaseStateWords {
    fn word(&self, state: PhaseState) -> AttrValue {
        match state {
            PhaseState::Done => self.done.clone(),
            PhaseState::Current => self.current.clone(),
            PhaseState::Waiting => self.waiting.clone(),
            PhaseState::Failed => self.failed.clone(),
        }
    }
}

/// One step of a [`PhaseStepper`].
#[derive(Clone, Debug, PartialEq)]
pub struct Phase {
    /// Stable identity of the phase: the render key, and the end of its `data-testid`.
    pub key: AttrValue,
    /// What the phase does, in the caller's words.
    pub label: AttrValue,
    pub state: PhaseState,
    /// One short line under the label, e.g. why the phase is waiting.
    pub reason: Option<AttrValue>,
}

impl Phase {
    pub fn new(key: impl Into<AttrValue>, label: impl Into<AttrValue>, state: PhaseState) -> Self {
        Self {
            key: key.into(),
            label: label.into(),
            state,
            reason: None,
        }
    }

    #[must_use]
    pub fn with_reason(mut self, reason: impl Into<AttrValue>) -> Self {
        self.reason = Some(reason.into());
        self
    }
}

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct PhaseStepperProps {
    /// The phases in the order they run.
    pub phases: Vec<Phase>,
    /// The list's accessible name (`aria-label`), e.g. "Import progress". Absent: nothing is
    /// rendered.
    #[prop_or_default]
    pub aria_label: Option<AttrValue>,
    /// The word read after each label for its state.
    #[prop_or_default]
    pub state_words: PhaseStateWords,
    /// `data-testid` of the list. Each phase gets `<testid>-<key>`, or just its key when this is
    /// absent.
    #[prop_or_default]
    pub testid: Option<AttrValue>,
    /// Extra classes of the list, e.g. spacing.
    #[prop_or_default]
    pub classes: Classes,
}

const LIST_STYLE: &str = "list-style:none;margin:0;padding:0;display:flex;flex-wrap:wrap;gap:.75rem 1.5rem";
/// A phase is at least 9rem wide, so phases share a row on a wide page and wrap two (or one) to
/// a row at phone width.
const PHASE_STYLE: &str = "flex:1 1 9rem;min-width:0;display:flex;align-items:flex-start;gap:.5rem";
const TEXT_STYLE: &str = "min-width:0;overflow-wrap:anywhere";
const REASON_STYLE: &str = "display:block;color:var(--bulma-text-weak, #6b6b6b)";

/// The phases of a long operation as an ordered list (`<ol>`), one item per phase.
///
/// Each phase shows the state's icon (its own shape per state, not only its colour) and its
/// label; a screen reader hears the label followed by the state word, and the current phase
/// is marked `aria-current="step"`. Each item carries `data-state` (see [`PhaseState::code`])
/// and a `data-testid` derived from the phase key. The row wraps on narrow screens.
#[component(PhaseStepper)]
pub fn phase_stepper(props: &PhaseStepperProps) -> Html {
    let phases = props.phases.iter().map(|phase| render_phase(phase, props));
    html! {
        <ol role="list" class={classes!("ybc-phase-stepper", props.classes.clone())} style={LIST_STYLE}
            aria-label={props.aria_label.clone()} data-testid={props.testid.clone()}>
            { for phases }
        </ol>
    }
}

fn render_phase(phase: &Phase, props: &PhaseStepperProps) -> Html {
    let is_current = phase.state == PhaseState::Current;
    let testid = match &props.testid {
        Some(list_testid) => format!("{list_testid}-{}", phase.key),
        None => phase.key.to_string(),
    };
    let label_class = classes!("ybc-phase-label", is_current.then_some("has-text-weight-semibold"));
    let icon_style = format!("color:{}", phase.state.color());
    html! {
        <li key={phase.key.to_string()} class="ybc-phase" style={PHASE_STYLE} data-testid={testid} data-state={phase.state.code()}
            aria-current={is_current.then_some("step")}>
            <span class="icon" style={icon_style}><i class={phase.state.icon()} aria-hidden="true"></i></span>
            <span style={TEXT_STYLE}>
                <span class={label_class}>{ phase.label.clone() }</span>
                <span class="is-sr-only">{ format!(": {}", props.state_words.word(phase.state)) }</span>
                if let Some(reason) = &phase.reason {
                    <span class="ybc-phase-reason is-size-7" style={REASON_STYLE}>{ reason.clone() }</span>
                }
            </span>
        </li>
    }
}
