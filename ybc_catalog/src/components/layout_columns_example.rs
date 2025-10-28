use ybc::*;
use yew::prelude::*;

#[function_component(ColumnsExamplePage)]
pub fn columns_example_page() -> Html {
    html! {
      <div class="section">
        <div class="container">
          <Columns>
            <Column classes={classes!("is-one-third")}>
              <div class="box">{"One third"}</div>
            </Column>
            <Column>
              <div class="box">{"Auto"}</div>
            </Column>
            <Column classes={classes!("is-narrow")}>
              <div class="box">{"Narrow"}</div>
            </Column>
          </Columns>
        </div>
      </div>
    }
}
