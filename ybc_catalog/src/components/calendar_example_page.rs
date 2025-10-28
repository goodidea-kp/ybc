use ybc::*;
use yew::prelude::*;

#[function_component(CalendarExamplePage)]
pub fn calendar_example_page() -> Html {
    let date = use_state(|| Option::<String>::None);
    let on_date_changed = {
        let date = date.clone();
        Callback::from(move |d: String| {
            date.set(if d.is_empty() { None } else { Some(d) });
        })
    };

    html! {
        <ybc::Section>
          <ybc::Container classes={classes!("content")}> 
            <h3>{"Calendar"}</h3>
            <p class="is-size-6">{"bulma-calendar date/time picker wrapped in Yew."}</p>

            <ybc::Field>
              <ybc::Tag tag="Label">{"Pick date & time"}</ybc::Tag>
              <ybc::Control>
                <Calendar id="demo-calendar" date_format="yyyy-MM-dd" time_format="HH:mm" date={(*date).clone()} on_date_changed={on_date_changed.clone()} class={vec!["is-small".into()]} />
              </ybc::Control>
            </ybc::Field>

            <div class="is-size-7 mt-3">
              <span class="has-text-weight-semibold">{"Current value:"}</span>
              { match &*date {
                  None => html!{ <span class="has-text-grey">{" none"}</span> },
                  Some(v) if v.is_empty() => html!{ <span class="has-text-grey">{" none"}</span> },
                  Some(v) => html!{ <span class="tag is-link is-light ml-2">{ v.clone() }</span> }
                }
              }
            </div>
          </ybc::Container>
        </ybc::Section>
    }
}

