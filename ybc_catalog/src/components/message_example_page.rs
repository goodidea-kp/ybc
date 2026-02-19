use yew::prelude::*;
use ybc::components::message::{Message, MessageHeader, MessageBody};
use crate::ui::DemoTitle;

#[component(MessageExamplePage)]
pub fn message_example_page() -> Html {
    html! {
        <ybc::Section>
            <ybc::Container classes={classes!("content")}>
                <DemoTitle title={"Message"} icon_classes={classes!("fa-solid", "fa-envelope-open-text")} />

                <Message>
                    <MessageHeader>
                        { "Default" }
                        <ybc::Button classes={classes!("delete")} no_button_class={true} aria_label={"delete"} />
                    </MessageHeader>
                    <MessageBody>
                        { "A simple message with a header and a body." }
                    </MessageBody>
                </Message>

                <Message classes={classes!("is-primary")}>
                    <MessageHeader>
                        { "Primary" }
                        <ybc::Button classes={classes!("delete")} no_button_class={true} aria_label={"delete"} />
                    </MessageHeader>
                    <MessageBody>
                        { "Primary color message body." }
                    </MessageBody>
                </Message>

                <Message classes={classes!("is-link")}>
                    <MessageHeader>
                        { "Link" }
                        <ybc::Button classes={classes!("delete")} no_button_class={true} aria_label={"delete"} />
                    </MessageHeader>
                    <MessageBody>
                        { "Link color message body." }
                    </MessageBody>
                </Message>

                <Message classes={classes!("is-success", "is-small")}>
                    <MessageHeader>
                        { "Success (small)" }
                        <ybc::Button classes={classes!("delete")} no_button_class={true} aria_label={"delete"} />
                    </MessageHeader>
                    <MessageBody>
                        { "Small size message." }
                    </MessageBody>
                </Message>

                <Message classes={classes!("is-warning", "is-medium")}>
                    <MessageHeader>
                        { "Warning (medium)" }
                        <ybc::Button classes={classes!("delete")} no_button_class={true} aria_label={"delete"} />
                    </MessageHeader>
                    <MessageBody>
                        { "Medium size message." }
                    </MessageBody>
                </Message>

                <Message classes={classes!("is-danger", "is-large")}>
                    <MessageHeader>
                        { "Danger (large)" }
                        <ybc::Button classes={classes!("delete")} no_button_class={true} aria_label={"delete"} />
                    </MessageHeader>
                    <MessageBody>
                        { "Large size message." }
                    </MessageBody>
                </Message>
            </ybc::Container>
        </ybc::Section>
    }
}
