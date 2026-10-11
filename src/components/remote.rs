//! Data on its way from a server, as one value instead of three flags.
//!
//! `is_loading` + `error: Option<_>` + `data: Option<_>` allow eight
//! combinations of which four are impossible, and every reader has to prove
//! that to themselves. [`Remote`] has exactly the four that exist, and
//! [`RemoteView`] gives each one its face: nothing, a skeleton, the data, or
//! the error with a way to try again.

use std::fmt::Display;

use yew::prelude::*;

use crate::{Callout, CalloutSeverity, SkeletonBlock};

/// Where a server value is. `E` is what a failure carries; its `Display` is
/// what [`RemoteView`] shows unless told otherwise.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub enum Remote<T, E = String> {
    /// Nothing asked for yet.
    #[default]
    Idle,
    /// Asked for, no answer yet.
    Loading,
    Loaded(T),
    Failed(E),
}

impl<T, E> Remote<T, E> {
    pub fn is_loading(&self) -> bool {
        matches!(self, Remote::Loading)
    }

    /// The data, when it is here.
    pub fn loaded(&self) -> Option<&T> {
        match self {
            Remote::Loaded(value) => Some(value),
            _ => None,
        }
    }

    /// The failure, when there is one.
    pub fn failure(&self) -> Option<&E> {
        match self {
            Remote::Failed(error) => Some(error),
            _ => None,
        }
    }

    /// The same state with the data turned into something else.
    pub fn map<U>(self, transform: impl FnOnce(T) -> U) -> Remote<U, E> {
        match self {
            Remote::Idle => Remote::Idle,
            Remote::Loading => Remote::Loading,
            Remote::Loaded(value) => Remote::Loaded(transform(value)),
            Remote::Failed(error) => Remote::Failed(error),
        }
    }

    /// Borrows the data and the failure, e.g. to `map` without cloning.
    pub fn as_ref(&self) -> Remote<&T, &E> {
        match self {
            Remote::Idle => Remote::Idle,
            Remote::Loading => Remote::Loading,
            Remote::Loaded(value) => Remote::Loaded(value),
            Remote::Failed(error) => Remote::Failed(error),
        }
    }
}

impl<T, E> From<Result<T, E>> for Remote<T, E> {
    fn from(result: Result<T, E>) -> Self {
        match result {
            Ok(value) => Remote::Loaded(value),
            Err(error) => Remote::Failed(error),
        }
    }
}

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct RemoteViewProps<T, E>
where
    T: Clone + PartialEq + 'static,
    E: Clone + PartialEq + Display + 'static,
{
    pub remote: Remote<T, E>,
    /// Draws the data. Keep `T` cheap to clone (`Rc<..>` for a list).
    pub render: Callback<T, Html>,
    /// Shown while [`Remote::Loading`]; defaults to a three-line
    /// [`SkeletonBlock`]. Use skeleton `Title`, `Tag` or `Button` to keep the
    /// page's shape.
    #[prop_or_default]
    pub skeleton: Option<Html>,
    /// Shown while [`Remote::Idle`]; nothing by default.
    #[prop_or_default]
    pub idle: Html,
    /// Draws a failure; defaults to a danger [`Callout`] with the error's text.
    #[prop_or_default]
    pub render_failure: Option<Callback<E, Html>>,
    /// Adds a "Try again" button under the default failure.
    #[prop_or_default]
    pub on_retry: Option<Callback<()>>,
    /// `data-testid` of the wrapper; the default failure gets `<testid>-failed`
    /// and its button `<testid>-retry`.
    #[prop_or_default]
    pub testid: Option<AttrValue>,
    #[prop_or_default]
    pub classes: Classes,
}

/// One face per [`Remote`] state, inside a wrapper that is `aria-busy` while
/// loading so assistive technology waits for the real content.
#[component(RemoteView)]
pub fn remote_view<T, E>(props: &RemoteViewProps<T, E>) -> Html
where
    T: Clone + PartialEq + 'static,
    E: Clone + PartialEq + Display + 'static,
{
    let content = match &props.remote {
        Remote::Idle => props.idle.clone(),
        Remote::Loading => props.skeleton.clone().unwrap_or_else(|| html! { <SkeletonBlock /> }),
        Remote::Loaded(value) => props.render.emit(value.clone()),
        Remote::Failed(error) => match &props.render_failure {
            Some(render_failure) => render_failure.emit(error.clone()),
            None => default_failure(error, props.on_retry.as_ref(), props.testid.as_ref()),
        },
    };
    html! {
        <div class={props.classes.clone()} aria-busy={props.remote.is_loading().to_string()} data-testid={props.testid.clone()}>
            { content }
        </div>
    }
}

fn default_failure(error: &impl Display, on_retry: Option<&Callback<()>>, testid: Option<&AttrValue>) -> Html {
    let derived = |suffix: &str| testid.map(|testid| AttrValue::from(format!("{testid}-{suffix}")));
    let retry = on_retry.map(|on_retry| {
        let onclick = on_retry.reform(|_: MouseEvent| ());
        html! { <button type="button" class="button is-small mt-2" data-testid={derived("retry")} {onclick}>{ "Try again" }</button> }
    });
    html! {
        <Callout severity={CalloutSeverity::Danger} testid={derived("failed")}>
            <p>{ error.to_string() }</p>
            { retry }
        </Callout>
    }
}
