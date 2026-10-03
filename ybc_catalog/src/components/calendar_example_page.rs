use ybc::*;
use yew::prelude::*;
use crate::ui::DemoTitle;

#[component(CalendarExamplePage)]
pub fn calendar_example_page() -> Html {
    let date = use_state(|| Some("12/31/2030".to_owned()));
    let day = use_state(|| Some("2026-10-01".to_string()));
    let locked = use_state(|| false);
    let on_day_changed = {
        let day = day.clone();
        Callback::from(move |picked: String| day.set(Some(picked)))
    };
    let toggle_locked = {
        let locked = locked.clone();
        Callback::from(move |_| locked.set(!*locked))
    };
    let on_date_changed = {
        let date = date.clone();
        Callback::from(move |d: String| {
            date.set(if d.is_empty() { None } else { Some(d) });
        })
    };

    html! {
        <ybc::Section>
          <ybc::Container classes={classes!("content")}> 
            <DemoTitle title={"Calendar"} icon_classes={classes!("fa-solid", "fa-calendar-days")} />
            <p class="is-size-6">{"Bulma Calendar with a native fallback if its script is unavailable. Form values use mm/dd/yyyy."}</p>

            <ybc::Field>
              <label for="demo-calendar">{"Pick date"}</label>
              <ybc::Control>
                <Calendar id="demo-calendar" date_format="mm/dd/yyyy" date={(*date).clone()} on_date_changed={on_date_changed.clone()} class={vec!["is-small".into()]} />
              </ybc::Control>
            </ybc::Field>

            <ybc::Field>
              <label for="demo-calendar-disabled">{"Disabled date"}</label>
              <ybc::Control>
                <Calendar id="demo-calendar-disabled" date_format="mm/dd/yyyy" date={(*date).clone()} disabled=true on_date_changed={on_date_changed.clone()} />
              </ybc::Control>
            </ybc::Field>

            <ybc::Field>
              <ybc::Tag tag="Label">{"A day, picked in a dialog (for use inside a modal), and lockable"}</ybc::Tag>
              <ybc::Control>
                <Calendar id="demo-calendar-dialog" display_mode="dialog" disabled={*locked}
                          date={(*day).clone()} on_date_changed={on_day_changed} />
              </ybc::Control>
              <button class="button is-small mt-2" onclick={toggle_locked}>
                { if *locked { "Unlock" } else { "Lock" } }
              </button>
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
