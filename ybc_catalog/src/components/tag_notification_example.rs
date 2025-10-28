use ybc::*;
use yew::prelude::*;

#[function_component(TagNotificationExamplePage)]
pub fn tags_notifications_page() -> Html {
    html! {
      <ybc::Section>
        <ybc::Container classes={classes!("content")}>
          <h2>{"Tags"}</h2>
          <ybc::Tags>
            <ybc::Tag classes={classes!("is-primary")}>{"Primary"}</ybc::Tag>
            <ybc::Tag classes={classes!("is-info")}>{"Info"}</ybc::Tag>
            <ybc::Tag classes={classes!("is-success")}>{"Success"}</ybc::Tag>
            <ybc::Tag classes={classes!("is-warning")}>{"Warning"}</ybc::Tag>
            <ybc::Tag classes={classes!("is-danger")}>{"Danger"}</ybc::Tag>
          </ybc::Tags>
          <h2>{"Notifications"}</h2>
          <ybc::Notification classes={classes!("is-info")}>{"An info notification"}</ybc::Notification>
          <ybc::Notification classes={classes!("is-warning")}>{"A warning notification"}</ybc::Notification>
        </ybc::Container>
      </ybc::Section>
    }
}
