use yew::prelude::*;
use ybc::components::message::{Message, MessageHeader, MessageBody};

#[function_component(MessageExamplePage)]
pub fn message_example_page() -> Html {
    html! {
        <div class="section">
            <div class="container content">
                <h3>{ "Message" }</h3>

                <Message>
                    <MessageHeader>
                        { "Default" }
                        <button class="delete" aria-label="delete"></button>
                    </MessageHeader>
                    <MessageBody>
                        { "A simple message with a header and a body." }
                    </MessageBody>
                </Message>

                <Message classes={classes!("is-primary")}>
                    <MessageHeader>
                        { "Primary" }
                        <button class="delete" aria-label="delete"></button>
                    </MessageHeader>
                    <MessageBody>
                        { "Primary color message body." }
                    </MessageBody>
                </Message>

                <Message classes={classes!("is-link")}>
                    <MessageHeader>
                        { "Link" }
                        <button class="delete" aria-label="delete"></button>
                    </MessageHeader>
                    <MessageBody>
                        { "Link color message body." }
                    </MessageBody>
                </Message>

                <Message classes={classes!("is-success", "is-small")}>
                    <MessageHeader>
                        { "Success (small)" }
                        <button class="delete" aria-label="delete"></button>
                    </MessageHeader>
                    <MessageBody>
                        { "Small size message." }
                    </MessageBody>
                </Message>

                <Message classes={classes!("is-warning", "is-medium")}>
                    <MessageHeader>
                        { "Warning (medium)" }
                        <button class="delete" aria-label="delete"></button>
                    </MessageHeader>
                    <MessageBody>
                        { "Medium size message." }
                    </MessageBody>
                </Message>

                <Message classes={classes!("is-danger", "is-large")}>
                    <MessageHeader>
                        { "Danger (large)" }
                        <button class="delete" aria-label="delete"></button>
                    </MessageHeader>
                    <MessageBody>
                        { "Large size message." }
                    </MessageBody>
                </Message>
            </div>
        </div>
    }
}

