use ybc::*;
use yew::prelude::*;
#[function_component(ModalExamplePage)]
pub fn modal_example_page() -> Html {
    const ID: &str = "id";
    const ID2: &str = "id2";
    html! {
      <ybc::Section>
        <ybc::Container>
        <ModalCloserProvider id={ID}>
           <MyModal id={ID} classes={classes!("is-success")} />
        </ModalCloserProvider>
        <ModalCloserProvider id={ID2}>
           <MyModal id={ID2} classes={classes!("is-danger")} />
        </ModalCloserProvider>
        </ybc::Container>
      </ybc::Section>
    }
}

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct MyModalProps {
    /// The ID of this modal, used for triggering close events from other parts of the app.
    #[prop_or_default("id")]
    pub id: String,
    #[prop_or_default]
    pub classes: Classes,
}
#[function_component(MyModal)]
pub fn my_modal(props: &MyModalProps) -> Html {
    let msg_ctx = use_context::<ModalCloserContext>().unwrap();
    let id_close = format!("{}-close", props.id);
    let onclick = {
        let id = id_close.clone();
        Callback::from(move |e: MouseEvent| msg_ctx.dispatch(id.clone().parse().unwrap()))
    };
    let msg_ctx2 = use_context::<ModalCloserContext>().unwrap();
    let onsave = {
        let id = id_close.clone();
        Callback::from(move |e: MouseEvent| msg_ctx2.dispatch(id.parse().unwrap()))
    };
    let on_click_cb = Callback::from(move |e: MouseEvent| {
        gloo_console::log!(" Button clicked! ID:", id_close.clone());
    });
    html! {
            <ybc::ModalCard
                classes={classes!("")}
                id={props.id.clone()}
                title={"Modal"}
                trigger={html!{
                    <ybc::Button classes={&props.classes} onclick={on_click_cb}>
                        {"Open Modal"}
                    </ybc::Button>
                }}
                body={
                    html!{
                    <ybc::Content>
                        <p classes="has-text-green">{"This is the body of the modal."}</p>
                    </ybc::Content>
                    }
                }
                footer={html!(
                      <>
                        <ybc::Button classes={classes!("is-success")} onclick={onsave}>
                            {"Save changes"}
                        </ybc::Button>
                        <ybc::Button classes={classes!("is-danger")} onclick={onclick}>
                            {"Close"}
                        </ybc::Button>
                    </>
                )}
            />
    }
}
