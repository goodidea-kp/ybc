use ybc::*;
use yew::prelude::*;
use crate::ui::DemoTitle;
#[component(ModalExamplePage)]
pub fn modal_example_page() -> Html {
    const ID: &str = "modal-example-1";
    const ID2: &str = "modal-example-2";
    html! {
      <ybc::Section>
        <ybc::Container>
          <DemoTitle title={"Modal"} icon_classes={classes!("fa-solid", "fa-window-maximize")} />
          <ModalControllerProvider>
            <CloseAllModalsButton />
            <MyModal id={ID} classes={classes!("is-success")} />
            <MyModal id={ID2} classes={classes!("is-danger")} />
          </ModalControllerProvider>
        </ybc::Container>
      </ybc::Section>
    }
}

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct MyModalProps {
    /// The ID of this modal, used as the controller key and DOM id.
    #[prop_or_default("id")]
    pub id: String,
    #[prop_or_default]
    pub classes: Classes,
}

#[component(CloseAllModalsButton)]
pub fn close_all_modals_button() -> Html {
    let controller = use_context::<ModalControllerContext>().expect("Modal controller in context");
    let onclick = Callback::from(move |_| controller.close_all());

    html! {
        <ybc::Button classes={classes!("is-warning", "is-light")} {onclick}>
            {"Close all modals"}
        </ybc::Button>
    }
}

#[component(MyModal)]
pub fn my_modal(props: &MyModalProps) -> Html {
    let controller = use_context::<ModalControllerContext>().expect("Modal controller in context");
    let id = props.id.clone();

    let onclick = {
        let controller = controller.clone();
        let id = id.clone();
        Callback::from(move |_| controller.close(&id))
    };
    let onsave = {
        let controller = controller.clone();
        let id = id.clone();
        Callback::from(move |_| controller.close(&id))
    };

    html! {
            <ybc::ModalCard
                classes={classes!("")}
                id={props.id.clone()}
                title={"Modal"}
                trigger={html!{
                    <ybc::Button classes={&props.classes}>
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
