use serde::{Deserialize, Serialize};

use super::config::DEFAULT_CONFIG;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GifData {
    pub name: String,
    pub tags: Vec<String>,
    pub id: String,
}

impl GifData {
    pub fn get_ur(&self) -> String {
        format!("{protocol}://{domain}/{path}/{name}.{format}", protocol = DEFAULT_CONFIG.protocol, domain = DEFAULT_CONFIG.domain, path = DEFAULT_CONFIG.gif_path, name = self.name, format = DEFAULT_CONFIG.image_format)
    }
}


pub async fn get_gif_data() -> Result<Vec<GifData>, reqwest::Error> {
    let db_url = format!("{protocol}://{domain}/{path}", protocol = DEFAULT_CONFIG.protocol, domain = DEFAULT_CONFIG.domain, path = "db.json");
    let resp = reqwest::get(db_url).await?.json::<Vec<GifData>>().await?;

    Ok(resp)
}