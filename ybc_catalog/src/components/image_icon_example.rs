use ybc::FaIcon;
use yew::prelude::*;
use crate::ui::DemoTitle;

#[component(ImageIconExamplePage)]
pub fn image_icon_example_page() -> Html {
    html! {
      <ybc::Section>
        <ybc::Container>
          <DemoTitle title={"Image & Icon"} icon_classes={classes!("fa-solid", "fa-icons")} />
          <figure class="image is-128x128">
            <img src="https://picsum.photos/256" alt="random"/>
          </figure>

          <p class="mt-4">
            <FaIcon
              classes={classes!("has-text-info")}
              icon_classes={classes!("fa-solid","fa-user","fa-2x")}
              aria_label={"User profile icon"}
            />
            {" "}
            <FaIcon
              classes={classes!("has-text-danger")}
              icon_classes={classes!("fa-brands","fa-rust","fa-2x")}
              aria_label={"Rust logo"}
            />
          </p>
        </ybc::Container>
      </ybc::Section>
    }
}
