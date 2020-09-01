extern crate percent_encoding;

use percent_encoding::{utf8_percent_encode, AsciiSet, CONTROLS};

const FRAGMENT: &AsciiSet = &CONTROLS.add(b' ').add(b'"').add(b'<').add(b'>').add(b'`');

pub fn construct_github_url(query: &str) -> String {
    if query.trim() == "gh" {
        let github_url = "https://github.com";
        github_url.to_string()
    } else {
        // Assume the other match is "gh sometext" and search for such a github profile or repo
        construct_github_user_or_repo_url(&query[3..])
    }
}

pub fn construct_github_user_or_repo_url(query: &str) -> String {
    let encoded_query = utf8_percent_encode(query, FRAGMENT).to_string();
    let github_url = format!("https://github.com/{}", encoded_query);

    github_url
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_construct_github_url() {
        let fake_query = "gh";
        assert_eq!(construct_github_url(fake_query), "https://github.com")
    }

    #[test]
    fn test_construct_github_user_or_repo_url() {
        let fake_query = "gh mediocreeee/cumunisp";
        assert_eq!(
            construct_github_user_or_repo_url(fake_query),
            "https://github.com/gh%20mediocreeee/cumunisp"
        )
    }
}
