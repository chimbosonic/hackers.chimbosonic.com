pub struct Config<'a> {
    pub protocol: &'a str,
    pub domain: &'a str,
    pub gif_path: &'a str,
    pub image_format: &'a str,
    pub items_per_page: u32,
}

// https://hackers.chimbosonic.com/gifs/whats-with-him.gif
pub const DEFAULT_CONFIG: Config<'static> = Config {
    protocol: "https",
    domain: "hackers.chimbosonic.com",
    gif_path: "gifs",
    image_format: "gif",
    items_per_page: 6,
};
