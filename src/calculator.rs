use ::math::evaluate_to_string;
use leptos::*;

use crate::util::*;

#[component]
pub fn Example(
  #[prop(into)] text: String,
  io: (WriteSignal<String>, WriteSignal<String>),
) -> impl IntoView {
  let t2 = text.clone();
  view! {
    <li on:click=move|_| {
      io.0(text.clone());
      let result = evaluate_to_string(text.clone());
      io.1(result);
    }
    >
      <a>
        {t2}
      </a>
    </li>
  }
}

#[component]
pub fn Calculator() -> impl IntoView {
  let (read_input, write_input) = create_signal("".to_string());
  let (read_out, write_out) = create_signal("".to_string());
  let io = (write_input, write_out);
  write_out(evaluate_to_string("".to_string()));
  view! {
    <article>
      <Navigation>
        <CrumbHome/>
        <CrumbCurrent name="Calculator"/>
      </Navigation>
      <div>
        <input type="text" value=read_input placeholder="Expression" on:input=move|s| {
          let input = event_target_value(&s);
          let result = evaluate_to_string(input);
          write_out(result)
        }/>
        <p><strong>"result: "</strong> {read_out}</p>
        <hr/>
          <h3> "Examples to try:" </h3>
          <ul>
            <Example text="5+3*2" io=io/>
            <Example text="4^3^2+1" io=io/>
            <Example text="sin(pi)" io=io/>
            <Example text="e^4" io=io/>
            <Example text="arctan(tan(2pi))" io=io/>
            <Example text="sqrt(-1)" io=io/>
            <Example text="sec(i)" io=io/>
          </ul>
          <hr/>
        <Github repo="math"/>
      </div>
    </article>
  }
}
