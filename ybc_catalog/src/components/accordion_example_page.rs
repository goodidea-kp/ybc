use yew::prelude::*;
use yew_router::prelude::Link;
use crate::router::Route;
use crate::ui::DemoTitle;

#[component(AccordionExamplePage)]
pub fn accordion_example_page() -> Html {
    html! {
        <ybc::Section>
            <ybc::Container classes={classes!("content")}>
                <DemoTitle title={"Accordion"} icon_classes={classes!("fa-solid", "fa-list")} />

                <details open=true>
                    <summary class="is-size-5">{ "Overview" }</summary>
                    <p>{ "This example uses native details/summary for accessibility." }</p>
                </details>

                <details>
                    <summary class="is-size-6">{ "Navigation links" }</summary>
                    <ul>
                        <li><Link<Route> to={Route::Message}>{ "Message example" }</Link<Route>></li>
                        <li><Link<Route> to={Route::Panel}>{ "Panel example" }</Link<Route>></li>
                        <li><Link<Route> to={Route::Pagination}>{ "Pagination example" }</Link<Route>></li>
                    </ul>
                </details>

                <details>
                    <summary class="is-size-6">{ "Another section" }</summary>
                    <p>{ "Put any content here." }</p>
                </details>
            </ybc::Container>
        </ybc::Section>
    }
}
