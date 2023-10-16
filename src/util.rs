use leptos::*;

#[component]
pub fn Github(#[prop(into)] repo: String) -> impl IntoView {
  let link = format!("https://github.com/Xterminate1818/{}", repo);
  view! {
    <a href={link} target="_blank">
      "View source code on GitHub"
    </a>
  }
}

#[component]
pub fn Crumb(
  #[prop(into)] name: String,
  #[prop(into)] url: String,
) -> impl IntoView {
  view! {
    <li>
      <a href={url}>
        {name}
      </a>
    </li>
  }
}

#[component]
pub fn CrumbCurrent(#[prop(into)] name: String) -> impl IntoView {
  view! {
    <li>
      <a class="contrast">
        {name}
      </a>
    </li>
  }
}

#[component]
pub fn CrumbHome() -> impl IntoView {
  view! {
    <Crumb name="Home" url="/"/>
  }
}

#[component]
pub fn Navigation(children: Children) -> impl IntoView {
  view! {
    <header>
      <nav aria-label="breadcrumb">
        <ul>
          {children()}
        </ul>
      </nav>
    </header>
  }
}
