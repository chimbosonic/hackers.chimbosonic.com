use crate::{
    components::gif::Gif,
    libs::{config::DEFAULT_CONFIG, data::GifData},
};

use dioxus::prelude::*;

#[inline_props]
pub fn Gallery(cx: Scope, gallery_data: Vec<GifData>) -> Element {
    let items_per_page = DEFAULT_CONFIG.items_per_page;

    let mut current_page = use_state(cx, || 1);

    let total_pages: u32 = (gallery_data.len() as f32 / items_per_page as f32).ceil() as u32;
    // calculate the start and end index of the current page
    let start_index = (current_page.get() - 1) * items_per_page;

    let items = gallery_data
        .iter()
        .skip(start_index.try_into().unwrap())
        .take(items_per_page.try_into().unwrap())
        .collect::<Vec<_>>();

    cx.render(rsx!(
        div { id: "gallery",

            for item in items {
                Gif { gif: item.clone() }
            }
        }
        div { class: "pagination",
            button { onclick: move |_| current_page.set(1), "First" }
            button {
                onclick: move |_| {
                    if current_page.gt(&1) {
                        current_page -= 1;
                    }
                },
                "Previous"
            }

            for page in (current_page.saturating_sub(3)).max(1)..=(current_page + 5).min(total_pages) {
                button { onclick: move |_| current_page.set(page), "{page}" }
            }

            button {
                onclick: move |_| {
                    if current_page.lt(&total_pages) {
                        current_page += 1;
                    }
                },
                "Next"
            }

            button { onclick: move |_| current_page.set(total_pages), "Last" }
        }
    ))
}
