extern crate percent_encoding;

use percent_encoding::{utf8_percent_encode, AsciiSet, CONTROLS};

const FRAGMENT: &AsciiSet = &CONTROLS.add(b' ').add(b'"').add(b'<').add(b'>').add(b'`');

pub fn construct_translator_url(query: &str) -> String {
    if query.trim() == "tr" {
        let translator_url = "https://translate.google.com/?hl=uk";
        translator_url.to_string()

    // Check if it looks like a request for special languages
    } else if &query[..4] == "tr #" {
        construct_translator_langs_url(&query[4..])
    } else {
        // Assume the other match is "tr sometext" and just translate with default languages
        construct_translator_simple_url(&query[3..])
    }
}

pub fn construct_translator_langs_url(query: &str) -> String {
    let from = &query[0..2];
    let to = &query[3..5];
    let query = &query[6..];

    let encoded_query = utf8_percent_encode(query, FRAGMENT).to_string();
    let translator_url = format!(
        "https://translate.google.com/?hl=uk&sl={}&tl={}&text={}",
        from, to, encoded_query
    );

    translator_url
}

pub fn construct_translator_simple_url(query: &str) -> String {
    let encoded_query = utf8_percent_encode(query, FRAGMENT).to_string();
    let translator_url = format!("https://translate.google.com/?hl=uk&text={}", encoded_query);

    translator_url
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_construct_translator_url() {
        let fake_query = "tr";
        assert_eq!(
            construct_translator_url(fake_query),
            "https://translate.google.com/?hl=uk"
        )
    }

    #[test]
    fn test_construct_translator_simple_url() {
        let fake_query = "tr hello world";
        assert_eq!(
            construct_translator_simple_url(fake_query),
            "https://translate.google.com/?hl=uk&text=tr%20hello%20world"
        )
    }
}
