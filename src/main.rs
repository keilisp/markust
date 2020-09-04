// TODO: change port!
#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

use rocket::response::Redirect;
mod utils;

// Rocket setup
#[get("/")]
fn index() -> &'static str {
    "hello rocket"
}

// Search route will pass command
#[get("/search?<cmd>")]
fn search(cmd: String) -> Redirect {
    // TODO: Implement choice of search engines
    let command = utils::get_cmd_from_query(&cmd);
    let redirect_url = match command {
        "rd" => utils::reddit::construct_reddit_url(&cmd),
        "gh" => utils::github::construct_github_url(&cmd),
        "wk" => utils::wikipedia::construct_wikipedia_url(&cmd),
        "yt" => utils::youtube::construct_youtube_url(&cmd),
        "hb" => utils::habr::construct_habr_url(&cmd),
        "tt" => utils::typingtest::construct_typingtest_url(&cmd),
        "ax" => utils::aliexpress::construct_aliexpress_url(&cmd),
        "rz" => utils::rozetka::construct_rozetka_url(&cmd),
        "tr" => utils::translator::construct_translator_url(&cmd),
        _ => utils::searchengine::construct_search_url(&cmd),
    };
    Redirect::to(redirect_url)
}

fn main() {
    rocket::ignite().mount("/", routes![index, search]).launch();
}

// Tests
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_get_cmd_from_query_no_whitespace() {
        // Test with command only
        let actual = utils::get_cmd_from_query("gh");
        let expected = "gh";
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_get_cmd_from_query_with_whitespace() {
        // Test with command only
        let actual = utils::get_cmd_from_query("gh $fjdsklf34!@");
        let expected = "gh";
        assert_eq!(actual, expected);
    }
}
