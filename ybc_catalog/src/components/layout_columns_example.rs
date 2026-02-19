use ybc::*;
use yew::prelude::*;
use crate::ui::DemoTitle;

#[component(ColumnsExamplePage)]
pub fn columns_example_page() -> Html {
    html! {
      <ybc::Section>
        <ybc::Container>
          <DemoTitle title={"Columns"} icon_classes={classes!("fa-solid", "fa-table-columns")} />
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
        </ybc::Container>
      </ybc::Section>
    }
}
