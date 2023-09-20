#![feature(fn_traits)]
use leptos::*;

#[component]
fn App() -> impl IntoView {
  let (read, write) = create_signal(0);

  view! {
      <button on:click=move |_| write.update(|n| *n += 1)>
          "Click me: "
          {move || {read()}}
      </button>
      <p>"hello"</p>
  }
}

fn main() {
  mount_to_body(|| view! {<App/>})
}
