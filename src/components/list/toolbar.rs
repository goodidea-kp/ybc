use wasm_bindgen::JsCast;
use web_sys::{HtmlInputElement, HtmlSelectElement};
use yew::prelude::*;

use super::{MIN_TARGET_STYLE, next_list_id};

/// The rows-per-page choices a list offers unless it says otherwise.
pub const PAGE_SIZES: [u64; 3] = [25, 50, 100];

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct ListToolbarProps {
    /// The text in the filter box: what the operator typed, which may run
    /// ahead of the filter the list is showing.
    pub filter: AttrValue,
    /// Called with the box's text on every keystroke.
    pub on_filter: Callback<String>,
    /// Called by the Clear button; without it Clear sends an empty
    /// [`on_filter`](Self::on_filter).
    #[prop_or_default]
    pub on_clear: Option<Callback<()>>,
    /// The filter box's accessible name, e.g. "Filter domains by name".
    #[prop_or_else(|| AttrValue::from("Filter"))]
    pub filter_label: AttrValue,
    /// e.g. "Filter by name".
    #[prop_or_default]
    pub placeholder: Option<AttrValue>,
    /// Rows per page now.
    pub size: u64,
    /// The rows-per-page choices.
    #[prop_or(&PAGE_SIZES)]
    pub sizes: &'static [u64],
    /// Called with the rows per page picked.
    pub on_size: Callback<u64>,
    /// The range sentence, e.g. "1–25 of 112 domains"; `None` while the page
    /// is on its way (a skeleton line, `aria-busy`). It is a polite live
    /// region, so a screen reader hears the new range after each change.
    #[prop_or_default]
    pub range_text: Option<AttrValue>,
    /// Prefix of the test ids: `<testid>-filter-name`, `-filter-clear`,
    /// `-range` and `-page-size` (also the select's id). Without it the
    /// select's id is generated.
    #[prop_or_default]
    pub testid: Option<AttrValue>,
}

/// The bar above a list: the text filter (with Clear while it has text), the
/// range sentence and the rows-per-page choice.
#[component(ListToolbar)]
pub fn list_toolbar(props: &ListToolbarProps) -> Html {
    let generated = use_state(|| next_list_id("ybc-list"));
    let prefix = props.testid.as_ref().map_or_else(|| (*generated).clone(), ToString::to_string);
    let testid = |suffix: &str| props.testid.as_ref().map(|_| AttrValue::from(format!("{prefix}-{suffix}")));
    let size_id = format!("{prefix}-page-size");
    html! {
        <div class="level mb-2">
            <div class="level-left">
                <div class="level-item">{ filter_box(props, &testid) }</div>
            </div>
            <div class="level-right">
                <div class="level-item">{ range_sentence(props.range_text.as_ref(), testid("range")) }</div>
                <div class="level-item">
                    <label class="is-size-7 has-text-grey mr-2" for={size_id.clone()}>{ "Rows per page" }</label>
                    { size_select(props, size_id, testid("page-size")) }
                </div>
            </div>
        </div>
    }
}

fn filter_box(props: &ListToolbarProps, testid: &dyn Fn(&str) -> Option<AttrValue>) -> Html {
    let oninput = props.on_filter.reform(|event: InputEvent| {
        event
            .target()
            .and_then(|target| target.dyn_into::<HtmlInputElement>().ok())
            .map(|input| input.value())
            .unwrap_or_default()
    });
    let onclear = {
        let (on_clear, on_filter) = (props.on_clear.clone(), props.on_filter.clone());
        Callback::from(move |_: MouseEvent| match &on_clear {
            Some(on_clear) => on_clear.emit(()),
            None => on_filter.emit(String::new()),
        })
    };
    html! {
        <div class="field has-addons mb-0">
            <div class="control has-icons-left">
                <input class="input is-small" type="search" autocomplete="off" placeholder={props.placeholder.clone()}
                       aria-label={props.filter_label.clone()} data-testid={testid("filter-name")}
                       value={props.filter.clone()} {oninput}/>
                <span class="icon is-small is-left"><i class="fas fa-magnifying-glass" aria-hidden="true"></i></span>
            </div>
            if !props.filter.is_empty() {
                <div class="control">
                    <button type="button" class="button is-small" style={MIN_TARGET_STYLE} onclick={onclear}
                            data-testid={testid("filter-clear")} aria-label="Clear the filter">
                        { "Clear" }
                    </button>
                </div>
            }
        </div>
    }
}

/// The polite live region with the range; a skeleton line while loading.
fn range_sentence(range_text: Option<&AttrValue>, testid: Option<AttrValue>) -> Html {
    let loading = range_text.is_none();
    html! {
        <span class="is-size-7 has-text-grey" data-testid={testid} aria-live="polite" aria-busy={loading.then_some("true")}>
            if let Some(text) = range_text {
                { text.clone() }
            } else {
                <span class="is-skeleton">{ "Loading…" }</span>
            }
        </span>
    }
}

fn size_select(props: &ListToolbarProps, id: String, testid: Option<AttrValue>) -> Html {
    let current = props.size;
    let onchange = props.on_size.reform(move |event: Event| {
        event
            .target()
            .and_then(|target| target.dyn_into::<HtmlSelectElement>().ok())
            .and_then(|select| select.value().parse().ok())
            .unwrap_or(current)
    });
    let options = props.sizes.iter().map(|size| {
        let value = size.to_string();
        html! { <option value={value.clone()} selected={*size == current}>{ value }</option> }
    });
    html! {
        <div class="select is-small">
            <select id={id.clone()} name={id} data-testid={testid} {onchange}>
                { for options }
            </select>
        </div>
    }
}
