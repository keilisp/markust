extern crate percent_encoding;

use percent_encoding::{utf8_percent_encode, AsciiSet, CONTROLS};

const FRAGMENT: &AsciiSet = &CONTROLS.add(b' ').add(b'"').add(b'<').add(b'>').add(b'`');

pub fn construct_youtube_url(query: &str) -> String {
    if query.trim() == "yt" {
        let youtube_url = "https://www.youtube.com";
        youtube_url.to_string()
    } else {
        // Assume the other match is "yt sometext" and search for such a video on youtube
        construct_youtube_search_url(&query[3..])
    }
}

pub fn construct_youtube_search_url(query: &str) -> String {
    let encoded_query = utf8_percent_encode(query, FRAGMENT).to_string();
    let youtube_url = format!(
        "https://www.youtube.com/results?search_query={}",
        encoded_query
    );

    youtube_url
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_construct_youtube_url() {
        let fake_query = "yt";
        assert_eq!(construct_youtube_url(fake_query), "https://www.youtube.com")
    }

    #[test]
    fn test_construct_youtube_search_url() {
        let fake_query = "yt foss";
        assert_eq!(
            construct_youtube_search_url(fake_query),
            "https://www.youtube.com/results?search_query=yt%20foss"
        )
    }
}
