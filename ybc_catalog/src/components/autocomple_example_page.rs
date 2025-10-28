use std::rc::Rc;
use ybc::components::autocomplete::AutoComplete;
use yew::prelude::*;

#[function_component(AutocompleteExamplePage)]
pub fn autocomplete_example_page() -> Html {
    let selected = use_state(|| Vec::<String>::new());

    let on_update = {
        let selected = selected.clone();
        Callback::from(move |tag: String| {
            let mut v = (*selected).clone();
            if !v.contains(&tag) {
                v.push(tag);
                selected.set(v);
            }
        })
    };

    let on_remove = {
        let selected = selected.clone();
        Callback::from(move |tag: String| {
            let mut v = (*selected).clone();
            v.retain(|t| t != &tag);
            selected.set(v);
        })
    };

    let items = vec!["Rust", "Yew", "Bulma", "WASM", "Web"]
        .into_iter()
        .map(|s| s.to_string())
        .collect::<Vec<_>>();

    html! {
            <ybc::Section>
                <ybc::Container classes={classes!("content")}>
                    <h3>{ "Autocomplete" }</h3>

                    <p class="is-size-6">{ "Static tags" }</p>
                    <AutoComplete
                        id={Rc::<str>::from("ac-static")}
                        max_items={5}
                        items={items}
                        on_update={on_update}
                        on_remove={on_remove}
                        current_selector={Rc::<str>::from("")}
                        placeholder={Rc::<str>::from("Choose Tags")}
                        classes={classes!()}
                        case_sensitive={false}
                    />

                    <div class="is-size-7 mt-3">
                        <span class="has-text-weight-semibold">{ "Selected:" }</span>
                        {
                            if selected.is_empty() {
                                html! { <span class="has-text-grey">{ " none" }</span> }
                            } else {
                                html! {
                                    <div class="tags mt-1">
                                        { for (*selected).iter().cloned().map(|t| html!{ <span class="tag is-info">{ t }</span> }) }
                                    </div>
                                }
                            }
                        }
                    </div>
                </ybc::Container>
            </ybc::Section>
        }
}
