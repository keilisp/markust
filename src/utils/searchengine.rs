// TODO: change to startpage
extern crate percent_encoding;

use percent_encoding::{utf8_percent_encode, AsciiSet, CONTROLS};

const FRAGMENT: &AsciiSet = &CONTROLS.add(b' ').add(b'"').add(b'<').add(b'>').add(b'`');

pub fn construct_search_url(query: &str) -> String {
    // Implement several search engines
    let encoded_query = utf8_percent_encode(query, FRAGMENT).to_string();
    let search_url = format!("https://google.com/search?q={}", encoded_query);

    search_url
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_construct_search_url() {
        let fake_query = "foss";
        assert_eq!(
            construct_search_url(fake_query),
            "https://google.com/search?q=foss"
        );
    }

    #[test]
    fn test_construct_search_url_with_encoding() {
        let fake_query = "hello foss";
        assert_eq!(
            construct_search_url(fake_query),
            "https://google.com/search?q=hello%20foss"
        );
    }
}
