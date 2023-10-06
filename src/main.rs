#![feature(fn_traits)]
pub mod base_conversion;
pub mod calculator;

use base_conversion::*;
use calculator::*;
use leptos::*;
use leptos_router::*;

#[component]
fn Explanation() -> impl IntoView {
  let md = include_str!("../explanation.md");
  let html = markdown::to_html(md);
  view! {
    <div inner_html={html}/>
  }
}

#[component]
fn Both() -> impl IntoView {
  view! {
    <BaseConversion/>
    <Calculator/>
  }
}

#[component]
fn App() -> impl IntoView {
  view! {
    <Router>
      <main>
      <Routes>
        <Route path="/conversion" view=BaseConversion/>
        <Route path="calc" view=Calculator/>
        <Route path="" view=Both/>
      </Routes>
      </main>
    </Router>
  }
}

fn main() {
  console_log::init_with_level(log::Level::Debug).unwrap();
  mount_to_body(|| view! {<App/>})
}
