extern crate percent_encoding;

use percent_encoding::{utf8_percent_encode, AsciiSet, CONTROLS};

const FRAGMENT: &AsciiSet = &CONTROLS.add(b' ').add(b'"').add(b'<').add(b'>').add(b'`');

pub fn construct_typingtest_url(query: &str) -> String {
    if query.trim() == "tt" {
        let typingtest_url = "https://10fastfingers.com/typing-test/russian";
        typingtest_url.to_string()
    } else if &query[..2] == "tt" {
        // TODO: implement other languages
        if &query[3..] == "en" {
            construct_typingtest_lang_url("english")
        } else if &query[3..] == "ua" {
            construct_typingtest_lang_url("ukrainian")
        } else {
            construct_typingtest_lang_url(&query[3..])
        }
    } else {
        construct_typingtest_lang_url("russian")
    }
}

pub fn construct_typingtest_lang_url(query: &str) -> String {
    let encoded_query = utf8_percent_encode(query, FRAGMENT).to_string();
    let typingtest_url = format!("https://10fastfingers.com/typing-test/{}", encoded_query);

    typingtest_url
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_construct_typingtest_url() {
        let fake_query = "tt";
        assert_eq!(
            construct_typingtest_url(fake_query),
            "https://10fastfingers.com/typing-test/russian"
        )
    }

    #[test]
    fn test_construct_typingtest_lang_url() {
        let fake_query = "tt en";
        assert_eq!(
            construct_typingtest_lang_url(fake_query),
            "https://10fastfingers.com/typing-test/tt%20en"
        )
    }
}
