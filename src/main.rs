#![feature(fn_traits)]
use ::math::evaluate_to_string;
use leptos::*;

#[component]
fn Explanation() -> impl IntoView {
  let html = include_str!("../calc_explanation.html");
  view! {
    <div inner_html={html}/>
  }
}

#[component]
fn App() -> impl IntoView {
  view! {
    <div>
    </div>
    <div>
      <h1> "Online Calculator" </h1>
      <Calculator/>
    </div>
    <div>
      <Explanation/>
    </div>
  }
}

#[component]
fn Calculator() -> impl IntoView {
  let (read_input, write_input) = create_signal("".to_string());
  let (read_output, write_output) = create_signal("".to_string());
  view! {
      <table>
        <tr>
          <td>
            <input type="text" on:input=move|s| {
              write_input(event_target_value(&s))
            }/>
          </td>
          <td>
            <button on:click=move|_| {
              let input = read_input();
              let output = evaluate_to_string(input);
              write_output(output);
            }>
              "Calculate"
            </button>
          </td>
        </tr>
        <tr>
          <p>"Result: " {read_output}</p>
        </tr>
      </table>
  }
}

fn main() {
  mount_to_body(|| view! {<App/>})
}
