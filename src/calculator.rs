use ::math::evaluate_to_string;
use leptos::*;

#[component]
pub fn Calculator() -> impl IntoView {
  let (read_input, write_input) = create_signal("".to_string());
  let (read_output, write_output) = create_signal("".to_string());

  view! {
    <input type="text" placeholder="Expression" autofocus on:input=move|s| {
      write_input(event_target_value(&s))
    }/>
    <button on:click=move|_| {
      let input = read_input();
      let output = evaluate_to_string(input);
      write_output(output);
    }>
      "Calculate"
    </button>
    <p>"Result: " {read_output}</p>
  }
}
