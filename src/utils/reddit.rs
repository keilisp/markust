extern crate percent_encoding;

use percent_encoding::{utf8_percent_encode, AsciiSet, CONTROLS};

const FRAGMENT: &AsciiSet = &CONTROLS.add(b' ').add(b'"').add(b'<').add(b'>').add(b'`');

pub fn construct_reddit_url(query: &str) -> String {
    if query.trim() == "rd" {
        let reddit_url = "https://www.reddit.com";
        reddit_url.to_string()

    // Check if it looks like a reddit user profile
    } else if &query[..4] == "rd @" {
        construct_reddit_user_url(&query[4..])
    } else {
        // Assume the other match is "rd sometext" and search for such a subbredit
        construct_reddit_community_url(&query[3..])
    }
}

pub fn construct_reddit_community_url(query: &str) -> String {
    let encoded_query = utf8_percent_encode(query, FRAGMENT).to_string();
    let reddit_url = format!("https://www.reddit.com/r/{}", encoded_query);

    reddit_url
}

pub fn construct_reddit_user_url(query: &str) -> String {
    let encoded_query = utf8_percent_encode(query, FRAGMENT).to_string();
    let reddit_url = format!("https://www.reddit.com/u/{}", encoded_query);

    reddit_url
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_construct_reddit_url() {
        let fake_query = "rd";
        assert_eq!(construct_reddit_url(fake_query), "https://www.reddit.com")
    }

    #[test]
    fn test_construct_reddit_community_url() {
        let fake_query = "rd hello world";
        assert_eq!(
            construct_reddit_community_url(fake_query),
            "https://www.reddit.com/r/rd%20hello%20world"
        )
    }

    #[test]
    fn test_construct_reddit_user_url() {
        let fake_query = "rd @hello_world";
        assert_eq!(
            construct_reddit_user_url(fake_query),
            "https://www.reddit.com/u/rd%20@hello_world"
        )
    }
}
