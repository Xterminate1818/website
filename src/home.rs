use leptos::*;

use crate::util::*;

#[component]
fn Demo(
  name: &'static str,
  url: &'static str,
  desc: &'static str,
) -> impl IntoView {
  view! {
    <blockquote>
      <a href={url}> {name} </a>
      <br/>
      {desc}
    </blockquote>
  }
}

#[component]
pub fn About() -> impl IntoView {
  view! {
    <article>
      <Navigation>
        <CrumbHome/>
        <CrumbCurrent name="About"/>
      </Navigation>

      <p>
        "Contact me at logan@gatlintc.com"
      </p>
      <iframe src="https://github.com/Xterminate1818/based"> </iframe>
    </article>
  }
}

#[component]
pub fn Homepage() -> impl IntoView {
  view! {
    <article>
      <h4> "Personal projects" </h4>
      <Demo name="Base Conversion" url="/conversion" desc="Convert binary, octal, decimal, and hexadecimal numbers"/>
      <Demo name="Expression Solver" url="/calc" desc="Solve arbitrary text-based expressions. Supports trig functions, complex numbers, and more"/>
    </article>
  }
}
