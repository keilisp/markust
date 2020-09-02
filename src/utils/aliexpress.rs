extern crate percent_encoding;

use percent_encoding::{utf8_percent_encode, AsciiSet, CONTROLS};

const FRAGMENT: &AsciiSet = &CONTROLS.add(b' ').add(b'"').add(b'<').add(b'>').add(b'`');

pub fn construct_aliexpress_url(query: &str) -> String {
    // TODO: implement language choice
    if query.trim() == "ax" {
        let aliexpress_url = "https://aliexpress.ru";
        aliexpress_url.to_string()
    } else {
        // Assume the other match is "wk sometext" and search for such a product on Aliexpress
        construct_aliexpress_search_url(&query[3..])
    }
}

pub fn construct_aliexpress_search_url(query: &str) -> String {
    let encoded_query = utf8_percent_encode(query, FRAGMENT).to_string();
    let aliexpress_url = format!(
        "https://aliexpress.ru/wholesale?SearchText={}",
        encoded_query
    );

    aliexpress_url
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_construct_aliexpress_url() {
        let fake_query = "ax";
        assert_eq!(
            construct_aliexpress_url(fake_query),
            "https://aliexpress.ru"
        )
    }

    #[test]
    fn test_construct_aliexpress_search_url() {
        let fake_query = "ax rust";
        assert_eq!(
            construct_aliexpress_search_url(fake_query),
            "https://aliexpress.ru/wholesale?SearchText=ax%20rust"
        )
    }
}
