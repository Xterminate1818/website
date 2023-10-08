#![feature(fn_traits)]
pub mod base_conversion;
pub mod calculator;

use base_conversion::*;
use calculator::*;
use leptos::*;
use leptos_router::*;

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
    <main class="container">
      <Routes>
        <Route path="/conversion" view=BaseConversion />
        <Route path="calc" view=Calculator />
        <Route path="" view=Both />
      </Routes>
    </main>
  </Router>
  }
}

fn main() {
  console_log::init_with_level(log::Level::Debug).unwrap();
  std::panic::set_hook(Box::new(console_error_panic_hook::hook));
  mount_to_body(|| {
    view! {
    <App />}
  })
}
