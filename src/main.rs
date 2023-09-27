#![allow(non_snake_case)]
mod components;
mod libs;

use crate::components::gallery::Gallery;
use crate::libs::data::GifData;
use dioxus::prelude::*;
use libs::config::DEFAULT_CONFIG;
use libs::data::get_gif_data;
// use crate::components::gif::Gif;
use crate::libs::search::filter_gallery_data;

fn main() {
    // launch the web app
    dioxus_web::launch(App);
}

// create a component that renders a div with the text "Hello, world!"
fn App(cx: Scope) -> Element {
    let gallery_data = use_future(cx, (), |_| get_gif_data());
    let search_query = use_state(cx, String::new);

    match gallery_data.value() {
        Some(Ok(gif_list)) => match filter_gallery_data(gif_list, search_query.get()) {
            Ok(filtered_gif_list) => {
                render! {
                    h1 {
                        "Hackers (1995) Gifs Gallery by "
                        a { href: "https://chimbosonic.com/", "chimbosonic" }
                    }
                    input {
                        "type": "text",
                        placeholder: "Search...",
                        value: "{search_query}",
                        oninput: move |event| {
                            search_query.set(event.value.clone());
                        }
                    }
                    Gallery { gallery_data: filtered_gif_list }
                }
            }
            Err(_err) => {
                render! {
                    div { h1 { "Error loading gallery data" } }
                }
            }
        },

        Some(Err(_err)) => {
            render! {
                div { h1 { "Error loading gallery data" } }
            }
        }

        None => {
            render! {
                div { h1 { "Loading..." } }
            }
        }
    }
}


