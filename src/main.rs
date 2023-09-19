use yew::prelude::*;

#[function_component(App)]
fn app() -> Html {
  let videos = vec![
    VideoData {
      id: 1,
      title: "Building and breaking things".to_string(),
      speaker: "John Doe".to_string(),
      url: "https://youtu.be/PsaFVLr8t4E".to_string(),
    },
    VideoData {
      id: 2,
      title: "The development process".to_string(),
      speaker: "Jane Smith".to_string(),
      url: "https://youtu.be/PsaFVLr8t4E".to_string(),
    },
    VideoData {
      id: 3,
      title: "The Web 7.0".to_string(),
      speaker: "Matt Miller".to_string(),
      url: "https://youtu.be/PsaFVLr8t4E".to_string(),
    },
    VideoData {
      id: 4,
      title: "Mouseless development".to_string(),
      speaker: "Tom Jerry".to_string(),
      url: "https://youtu.be/PsaFVLr8t4E".to_string(),
    },
  ];

  html! {
    <>
        <h1>{ "RustConf Explorer" }</h1>
        <div>
            <h3>{"Videos to watch"}</h3>
            <VideoList videos={videos} on_click={on_video_select.clone()}/>
        </div>
    </>
  }
}

#[derive(Clone, PartialEq)]
struct VideoData {
  id: usize,
  title: String,
  speaker: String,
  url: String,
}

#[derive(Properties, PartialEq)]
struct VideoListProps {
  videos: Vec<VideoData>,
  on_click: Callback<VideoData>,
}

#[derive(Properties, PartialEq)]
struct VideoPlayerProps {
  video: VideoData,
}

#[function_component(VideoPlayer)]
fn video_details(VideoPlayerProps { video }: &VideoPlayerProps) -> Html {
  html! {
      <div>
          <h3>{ video.title.clone() }</h3>
          <img src="https://via.placeholder.com/640x360.png?text=Video+Player+Placeholder" alt="video thumbnail" />
      </div>
  }
}

#[function_component(VideoList)]
fn video_list(VideoListProps { videos, on_click }: &VideoListProps) -> Html {
  videos
    .iter()
    .map(|video| {
      let on_video_select = Callback::from(|_| on_click.emit(video.clone()));
      html! {

          <p key={video.id} onclick={on_video_select}>{format!("{}: {}", video.speaker, video.title)}</p>
      }
    })
    .collect()
}

fn main() {
  yew::Renderer::<App>::new().render();
}
