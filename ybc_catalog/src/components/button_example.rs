use ybc::{Button, Buttons};
use yew::prelude::*;
use crate::ui::DemoTitle;

#[component(ButtonExamplePage)]
pub fn button_example_page() -> Html {
    let clicks = use_state(|| 0);
    let onclick = {
        let clicks = clicks.clone();
        Callback::from(move |_| clicks.set(*clicks + 1))
    };

    html! {
      <ybc::Section>
        <ybc::Container classes={classes!("content")}>
          <DemoTitle title={"Button"} icon_classes={classes!("fa-solid", "fa-hand-pointer")} />
          <p>{"YBC wraps Bulma buttons. Use classes for colors/sizes."}</p>

          <Buttons>
            <Button classes={classes!("button","is-primary")} onclick={onclick.clone()}>{"Primary"}</Button>
            <Button classes={classes!("button","is-link")}>{"Link"}</Button>
            <Button classes={classes!("button","is-light")}>{"Light"}</Button>
            <Button classes={classes!("button","is-danger","is-outlined")}>{"Danger"}</Button>
          </Buttons>

          <p>{format!("Clicked: {} time(s)", *clicks)}</p>
        </ybc::Container>
      </ybc::Section>
    }
}
