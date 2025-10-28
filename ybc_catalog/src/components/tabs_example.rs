use ybc::*;
use yew::prelude::*;

#[function_component(TabsExamplePage)]
pub fn tabs_example_page() -> Html {
    let idx = use_state(|| 0usize);
    let set_idx = |i| {
        let idx = idx.clone();
        Callback::from(move |_| idx.set(i))
    };

    html! {
      <ybc::Section>
        <ybc::Container>
           <ybc::Tabs toggle=true fullwidth=true>
            <ul>
              <li class={ if *idx==0 {"is-active"} else {""} }><a onclick={set_idx(0)}>{"First"}</a></li>
              <li class={ if *idx==1 {"is-active"} else {""} }><a onclick={set_idx(1)}>{"Second"}</a></li>
              <li class={ if *idx==2 {"is-active"} else {""} }><a onclick={set_idx(2)}>{"Third"}</a></li>
            </ul>
        </ybc::Tabs>
         <div class="box">
            {
              match *idx {
                0 => html! { <p>{"First tab content"}</p> },
                1 => html! { <p>{"Second tab content"}</p> },
                _ => html! { <p>{"Third tab content"}</p> },
              }
            }
          </div>
        </ybc::Container>
      </ybc::Section>
    }
}
