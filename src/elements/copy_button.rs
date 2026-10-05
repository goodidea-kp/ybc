use std::time::Duration;
use wasm_bindgen_futures::JsFuture;
use yew::platform::spawn_local;
use yew::platform::time::sleep;
use yew::prelude::*;

/// How long the button says it copied before it says what it does again.
const COPIED_FOR: Duration = Duration::from_secs(2);

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct CopyButtonProps {
    /// What goes to the clipboard.
    pub text: AttrValue,
    #[prop_or_else(|| AttrValue::from("Copy"))]
    pub label: AttrValue,
    #[prop_or_else(|| AttrValue::from("Copied"))]
    pub copied_label: AttrValue,
    /// What a screen reader says for the button when the label alone is not enough
    /// ("Copy the SQL that fixes GRANTS_MATCH_POLICY").
    #[prop_or_default]
    pub aria_label: Option<AttrValue>,
    /// Defaults to `button is-small`.
    #[prop_or_default]
    pub classes: Classes,
    #[prop_or_default]
    pub testid: Option<AttrValue>,
}

/// What the last click did.
#[derive(Clone, Debug, PartialEq)]
enum Copy {
    Idle,
    Copied,
    Failed(String),
}

/// Copies `text`, and says so: the label turns to "Copied" for two seconds and
/// a polite live region announces it. A clipboard the browser refuses (no
/// secure context, permission denied) is said in words beside the button --
/// it never panics.
#[component(CopyButton)]
pub fn copy_button(props: &CopyButtonProps) -> Html {
    let state = use_state(|| Copy::Idle);
    let onclick = {
        let (state, text) = (state.clone(), props.text.clone());
        Callback::from(move |_: MouseEvent| {
            let (state, text) = (state.clone(), text.clone());
            spawn_local(async move {
                match write(&text).await {
                    Ok(()) => {
                        state.set(Copy::Copied);
                        sleep(COPIED_FOR).await;
                        state.set(Copy::Idle);
                    }
                    Err(why) => state.set(Copy::Failed(why)),
                }
            });
        })
    };
    let classes = if props.classes.is_empty() {
        classes!("button", "is-small")
    } else {
        props.classes.clone()
    };
    let copied = *state == Copy::Copied;
    html! {
        <>
            <button type="button" class={classes} {onclick} aria-label={props.aria_label.clone()} data-testid={props.testid.clone()}>
                { if copied { props.copied_label.clone() } else { props.label.clone() } }
            </button>
            <span class="is-sr-only" aria-live="polite">{ if copied { props.copied_label.clone() } else { AttrValue::default() } }</span>
            if let Copy::Failed(why) = &*state {
                <p class="help is-danger" role="alert">
                    { format!("Could not copy ({why}). Select the text and copy it by hand.") }
                </p>
            }
        </>
    }
}

async fn write(text: &str) -> Result<(), String> {
    let window = web_sys::window().ok_or_else(|| "no browser window".to_string())?;
    let promise = window.navigator().clipboard().write_text(text);
    JsFuture::from(promise)
        .await
        .map(|_| ())
        .map_err(|error| error.as_string().unwrap_or_else(|| "the browser refused".to_string()))
}
