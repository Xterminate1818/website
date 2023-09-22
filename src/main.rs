#![feature(fn_traits)]
pub mod calculator;

use calculator::*;
use leptos::*;

#[component]
fn Explanation() -> impl IntoView {
  let md = include_str!("../explanation.md");
  let html = markdown::to_html(md);
  view! {
    <div inner_html={html}/>
  }
}

#[component]
fn App() -> impl IntoView {
  view! {
    <div>
      <h1> "Online Calculator" </h1>
      <Calculator/>
    </div>
    <div>
      <Explanation/>
    </div>
  }
}

fn main() {
  mount_to_body(|| view! {<App/>})
}
