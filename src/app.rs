use yew::prelude::*;

use crate::ffmpeg::print_version;
use crate::ffmpeg::get_versions_map;

pub enum Msg {}

#[derive(Debug, Default)]
pub struct App;

impl App {
    fn lib_entry(&self, (libname, version): (&String, &String)) -> Html {
        html! {
            <li>
                { format!("{}{}", libname, version) }
            </li>
        }
    }
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        print_version();

        html! {
            <div>
                <h2>{ "FFmpeg versions" }</h2>
                <ul>
                { for get_versions_map().iter().map(|entry|
                    self.lib_entry(entry)
                )}
                </ul>
            </div>
        }
    }
}
