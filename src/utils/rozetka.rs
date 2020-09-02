extern crate percent_encoding;

use percent_encoding::{utf8_percent_encode, AsciiSet, CONTROLS};

const FRAGMENT: &AsciiSet = &CONTROLS.add(b' ').add(b'"').add(b'<').add(b'>').add(b'`');

pub fn construct_rozetka_url(query: &str) -> String {
    if query.trim() == "rz" {
        let rozetka_url = "https://rozetka.com.ua";
        rozetka_url.to_string()
    } else {
        // Assume the other match is "rz sometext" and search for such a product on Rozetka
        construct_rozetka_search_url(&query[3..])
    }
}

pub fn construct_rozetka_search_url(query: &str) -> String {
    let encoded_query = utf8_percent_encode(query, FRAGMENT).to_string();
    let rozetka_url = format!("https://rozetka.com.ua/search/?text={}", encoded_query);

    rozetka_url
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_construct_rozetka_url() {
        let fake_query = "rz";
        assert_eq!(construct_rozetka_url(fake_query), "https://rozetka.com.ua")
    }

    #[test]
    fn test_construct_rozetka_search_url() {
        let fake_query = "rz rust";
        assert_eq!(
            construct_rozetka_search_url(fake_query),
            "https://rozetka.com.ua/search/?text=rz%20rust"
        )
    }
}
