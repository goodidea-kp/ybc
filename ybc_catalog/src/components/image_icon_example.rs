use ybc::Icon;
use yew::prelude::*;

#[function_component(ImageIconExamplePage)]
pub fn image_icon_example_page() -> Html {
    html! {
      <div class="section">
        <div class="container">
          <figure class="image is-128x128">
            <img src="https://picsum.photos/256" alt="random"/>
          </figure>

          <p class="mt-4">
            <Icon classes={classes!("fa-solid","fa-user","fa-2x")}/>
            {" "}
            <Icon classes={classes!("fa-brands","fa-rust","fa-2x")}/>
          </p>
        </div>
      </div>
    }
}
