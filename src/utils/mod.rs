pub mod github;
pub mod habr;
pub mod reddit;
pub mod searchengine;
pub mod typingtest;
pub mod wikipedia;
pub mod youtube;

// Get the command from search query string
pub fn get_cmd_from_query(query: &str) -> &str {
    if query.contains(' ') {
        // If so slice the string
        let index_of_ws = query.find(' ').unwrap_or(0);
        return &query[..index_of_ws];
    }
    // Otherwise, return the query string as is
    &query
}
