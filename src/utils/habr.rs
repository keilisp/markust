extern crate percent_encoding;

use percent_encoding::{utf8_percent_encode, AsciiSet, CONTROLS};

const FRAGMENT: &AsciiSet = &CONTROLS.add(b' ').add(b'"').add(b'<').add(b'>').add(b'`');

pub fn construct_habr_url(query: &str) -> String {
    // TODO: implement language choice
    if query.trim() == "hb" {
        let habr_url = "https://habr.com/ru/";
        habr_url.to_string()
    } else {
        // Assume the other match is "hb sometext" and search for such an article on Habr
        construct_habr_search_url(&query[3..])
    }
}

pub fn construct_habr_search_url(query: &str) -> String {
    let encoded_query = utf8_percent_encode(query, FRAGMENT).to_string();
    let habr_url = format!("https://habr.com/ru/search/?q={}", encoded_query);

    habr_url
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_construct_habr_url() {
        let fake_query = "hb";
        assert_eq!(construct_habr_url(fake_query), "https://habr.com/ru/")
    }

    #[test]
    fn test_construct_habr_search_url() {
        let fake_query = "hb rust";
        assert_eq!(
            construct_habr_search_url(fake_query),
            "https://habr.com/ru/search/?q=hb%20rust"
        )
    }
}
