use dioxus::prelude::*;

use crate::libs::data::GifData;

#[inline_props]
pub fn Gif(cx: Scope, gif: GifData) -> Element {
    let GifData { name: _, tags, id } = gif;
    let url = gif.get_ur();
    render! {
        div { key: "{id}",
            a { href: "{url}", img { src: "{url}", title: "{tags:?}", alt: "{tags:?}" } }
        }
    }
}
