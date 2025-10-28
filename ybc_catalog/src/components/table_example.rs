use ybc::*;
use yew::prelude::*;

#[function_component(TableExamplePage)]
pub fn table_example_page() -> Html {
    html! {
      <ybc::Section>
        <ybc::Container>
          <Table classes={classes!("is-striped","is-hoverable","is-fullwidth")}>
            <thead>
              <tr><th>{"ID"}</th><th>{"Name"}</th><th>{"Role"}</th></tr>
            </thead>
            <tbody>
              <tr><td>{"1"}</td><td>{"Alice"}</td><td>{"Admin"}</td></tr>
              <tr><td>{"2"}</td><td>{"Bob"}</td><td>{"User"}</td></tr>
              <tr><td>{"3"}</td><td>{"Caro"}</td><td>{"Guest"}</td></tr>
            </tbody>
          </Table>
        </ybc::Container>
      </ybc::Section>
    }
}
