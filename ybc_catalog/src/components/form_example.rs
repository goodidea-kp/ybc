use ybc::*;
use yew::prelude::*;

#[function_component(FormExamplePage)]
pub fn form_example_page() -> Html {
    let text = use_state(|| String::new());
    let textinput = use_state(|| "Hello".to_string());
    let oninput = {
        let text = text.clone();
        Callback::from(move |e: String| {
            text.set(e);
        })
    };
    let ontextinput = {
        let text = textinput.clone();
        Callback::from(move |e: String| {
            text.set(e);
        })
    };
    let on_radio_update = {
        let text = text.clone();
        Callback::from(move |value: String| {
            text.set(value);
        })
    };
    let selected_radio = use_state(|| "A".to_string());
    let on_radio_update = {
        let selected_radio = selected_radio.clone();
        Callback::from(move |value: String| selected_radio.set(value))
    };
    let drop_selection = use_state(|| String::new());
    let on_select_update = {
        let text = drop_selection.clone();
        Callback::from(move |value: String| {
            text.set(value);
        })
    };
    let checked = use_state(|| false);
    let on_check_update = {
        let checked = checked.clone();
        Callback::from(move |value: bool| {
            checked.set(value);
        })
    };
    // Calendar state and callback
    let date = use_state(|| Option::<String>::None);
    let on_date_changed = {
        let date = date.clone();
        Callback::from(move |d: String| {
            date.set(if d.is_empty() { None } else { Some(d) });
        })
    };
    let calendar_departure_date = html! {
       <Calendar id="my-calendar" date={"2030-01-01 01:02"} on_date_changed={on_date_changed} class={vec!["input".to_string()]} />
    };

    let select_value = "B";
    html! {
      <ybc::Section>
        <ybc::Container classes={classes!("content")}>
          <h2>{"Form Controls"}</h2>
          <ybc::Field>
            <ybc::Tag tag="Label">{"Username"}</ybc::Tag>
            <ybc::Control>
              <Input name="username" value={(*text).clone()} update={oninput}/>
            </ybc::Control>
            <ybc::Tag tag="Help" classes={classes!("is-info")}>{"Type anything"}</ybc::Tag>
          </ybc::Field>

          <ybc::Field>
            <ybc::Tag tag="Label">{"Message"}</ybc::Tag>
            <ybc::Control>
              <TextArea value={(*textinput).clone()} name="message" update={ontextinput} readonly={false}/>
            </ybc::Control>
          </ybc::Field>

          <ybc::Field>
            <ybc::Tag tag="Label">{"Favorite"}</ybc::Tag>
            <ybc::Control>
              <ybc::Select name="select" update={on_select_update} loading={false} disabled={false} value={select_value}>
                <option value="R" selected={if "R".eq_ignore_ascii_case(select_value) { true } else { false }}>{"Rust"}</option>
                <option value="Y" selected={if "Y".eq_ignore_ascii_case(select_value) { true } else { false }}>{"Yew"}</option>
                <option value="B" selected={if "B".eq_ignore_ascii_case(select_value) { true } else { false }}>{"Bulma"}</option>
              </ybc::Select>
            </ybc::Control>
          </ybc::Field>

          <ybc::Field grouped=true>
            <ybc::Control><ybc::Checkbox name="checkbox" checked={*checked} update={on_check_update}>{" Accept terms"}</ybc::Checkbox></ybc::Control>
            <ybc::Control>
            <ybc::Radio name="r1" value="A" update={on_radio_update.clone()} checked_value={(*selected_radio).clone()}>{" Choice A"}</ybc::Radio>
            <ybc::Radio name="r1" value="B" update={on_radio_update} checked_value={(*selected_radio).clone()}>{" Choice B"}</ybc::Radio>
           </ybc::Control>
          </ybc::Field>

         <ybc::Field>
            <ybc::Tag tag="Label">{"Appointment"}</ybc::Tag>
            <ybc::Control>
              {calendar_departure_date}
            </ybc::Control>
          </ybc::Field>
        </ybc::Container>
      </ybc::Section>
    }
}
