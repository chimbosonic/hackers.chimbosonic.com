use fuzzy_matcher::skim::SkimMatcherV2;
use fuzzy_matcher::FuzzyMatcher;
use itertools::Itertools;

use super::data::GifData;

/// Filters and Sorts the Gallery by searching tags for given query.
/// This will return an error if no results are found. and will copy the gallery data.
pub fn filter_gallery_data(
    gallery_data: &[GifData],
    search_query: &str,
) -> Result<Vec<GifData>, (Vec<GifData>,&'static str)> {
    if search_query.is_empty() {
        return Ok(gallery_data.to_vec());
    }

    let matcher = SkimMatcherV2::default();

    let filtered_gif_list: Vec<GifData> = gallery_data
        .iter()
        .map(|gif| {
            let mut score: i32 = 0;

            for tag in &gif.tags {
                for query in search_query.split_whitespace() {
                    if matcher
                        .fuzzy_match(tag, query.to_lowercase().as_str())
                        .is_some()
                        && query.ne("the")
                    {
                        score += 1;
                    }
                }
            }

            (gif, score)
        })
        .filter(|(_, score)| *score > 0)
        .sorted_by_key(|x| -x.1)
        .map(|(gif, _)| gif.clone())
        .collect();

    if filtered_gif_list.is_empty() {
        Ok(gallery_data.to_vec())
    } else {
        Ok(filtered_gif_list)
    }
}
