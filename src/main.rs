#![recursion_limit = "256"]

mod app;
mod ffmpeg;

use app::App;

fn main() {
    yew::start_app::<App>();
}