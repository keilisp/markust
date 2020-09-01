extern crate percent_encoding;

use percent_encoding::{utf8_percent_encode, AsciiSet, CONTROLS};

const FRAGMENT: &AsciiSet = &CONTROLS.add(b' ').add(b'"').add(b'<').add(b'>').add(b'`');

pub fn construct_wikipedia_url(query: &str) -> String {
    // TODO: implement language choice
    if query.trim() == "wk" {
        let wikipedia_url = "https://ru.wikipedia.org";
        wikipedia_url.to_string()
    } else {
        // Assume the other match is "wk sometext" and search for such a wiki page
        construct_wikipedia_search_url(&query[3..])
    }
}

pub fn construct_wikipedia_search_url(query: &str) -> String {
    let encoded_query = utf8_percent_encode(query, FRAGMENT).to_string();
    let wikipedia_url= format!("https://ru.wikipedia.org/w/index.php?sort=relevance&search={}&title=Special%3ASearch&profile=advanced&fulltext=1",
                                encoded_query);

    wikipedia_url
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_construct_wikipedia_url() {
        let fake_query = "wk";
        assert_eq!(
            construct_wikipedia_url(fake_query),
            "https://ru.wikipedia.org"
        )
    }

    #[test]
    fn test_construct_wikipedia_search_url() {
        let fake_query = "wk foss";
        assert_eq!(
            construct_wikipedia_search_url(fake_query),
            "https://ru.wikipedia.org/w/index.php?sort=relevance&search=wk%20foss&title=Special%3ASearch&profile=advanced&fulltext=1"
        )
    }
}
