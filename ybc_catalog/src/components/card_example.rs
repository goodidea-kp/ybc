use ybc::*;
use yew::prelude::*;

#[function_component(CardExamplePage)]
pub fn card_example_page() -> Html {
    let image_loaded = use_state(|| false);

    html! {
      <div class="section">
        <div class="container">
          <Card>
            <CardHeader>
              <p>{"My Card"}</p>
            </CardHeader>

            <CardImage>
              <figure class="image is-4by3 is-relative">
                <div
                  class={classes!(
                    "is-overlay",
                    "is-flex",
                    "is-justify-content-center",
                    "is-align-items-center",
                    "has-background-light",
                    "has-text-grey-dark",
                    (*image_loaded).then_some("is-hidden"),
                  )}
                >
                  {"Loading image…"}
                </div>

                <img
                  class={classes!((!*image_loaded).then_some("is-hidden"))}
                  src="https://picsum.photos/800/600"
                  alt="random"
                  onload={{
                    let image_loaded = image_loaded.clone();
                    Callback::from(move |_| image_loaded.set(true))
                  }}
                />
              </figure>
            </CardImage>

            <CardContent>
              <div class="content">
                <p>{"Card content paragraph with "}
                  <a href="#">{"a link"}</a>
                  {" and some text."}
                </p>
              </div>
            </CardContent>

            <CardFooter>
              <ybc::Button>{"View"}</ybc::Button>
              <ybc::Button>{"Edit"}</ybc::Button>
            </CardFooter>
          </Card>
        </div>
      </div>
    }
}
