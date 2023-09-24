use ::math::evaluate_to_string;
use leptos::*;

#[component]
pub fn Example(
  text: String,
  input: WriteSignal<String>,
  out: WriteSignal<String>,
) -> impl IntoView {
  let t2 = text.clone();
  view! {
    <li on:click=move|_| {
      input(text.clone());
      let result = evaluate_to_string(text.clone());
      out(result);
    }
      >
      {t2}
    </li>
  }
}

#[component]
pub fn Calculator() -> impl IntoView {
  let (read_input, write_input) = create_signal("".to_string());
  let (read_out, write_out) = create_signal("".to_string());
  write_out(evaluate_to_string("".to_string()));
  view! {
    <div class="calc">
      <h1> "Expression Evaluator" </h1>
      <input type="text" value=read_input placeholder="Expression" autofocus on:input=move|s| {
        let input = event_target_value(&s);
        let result = evaluate_to_string(input);
        write_out(result)
      }/>
      <p>" = " {read_out}</p>
      <p> "View source code on " <a target="_blank" href="https://github.com/Xterminate1818/math">"GitHub"</a> </p>
      <hr/>
        <h3> "Examples to try:" </h3>
        <ul>
          <Example text="5+3*2".to_string() input=write_input out=write_out/>
          <Example text="4^3^2+1".to_string() input=write_input out=write_out/>
          <Example text="sin(pi)".to_string() input=write_input out=write_out/>
          <Example text="e^4".to_string() input=write_input out=write_out/>
          <Example text="arctan(tan(2pi))".to_string() input=write_input out=write_out/>
          <Example text="sqrt(-1)".to_string() input=write_input out=write_out/>
          <Example text="sec(i)".to_string() input=write_input out=write_out/>
        </ul>
        <hr/>
      <h3> "Made by Logan Gatlin" </h3>
    </div>

  }
}
