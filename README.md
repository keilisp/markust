# Markust

# Description

Bookmarking/easysearch tool written in Rust with Rocket framework.

# Preview

![markust](https://raw.githubusercontent.com/mediocreeee/markust/master/markustwork.gif)

# Installation

Clone the repo into your folder

```sh
cd *your-folder*
git clone https://github.com/mediocreeee/markust.git
```

Run the app via cargo (server will be opened on port 8000)

```sh
cargo run
```

Add new search engine in your browser with this line as a query

> http://localhost:8000/search?cmd=%s

Now you probably should add markust binary to your system startup

# Usage

## Github

To go to github.com:

> gh

To go to user's profile:

> gh _username_

To go to user's repo:

> gh _username_/_repository_

## YouTube

To go to youtube.com:

> yt

To make a search on youtube:

> yt _search_query_

## Wikipedia

To go to wikipedia.org:

> wk

To make a search on wikipedia:

> wk _search_query_

## Google Translate

To go to translate.google.com:

> tr

To translate query with your default languages:

> tr _query_

To translate query from specified language to specified language:

> tr #_from_-_to_ _query_

## Reddit

To go to reddit.com:

> rd

To go to _subreddit_ page:

> rd _subreddit_

To go to _username_ page:

> rd @_username_

## Habr

To go to habr.com:

> hb

To make a search on habr.com:

> hb _search_query_

## FastFingers

To go to 10fastfingers.com:

> tt

To go to 10fastfingers.com and select language you want:

> tt _lang_code_
> Example:
> tt german

## Aliexpress

To go to aliexpress.ru:

> ax

To make a search on aliexpress:

> ax _search_query_

## Rozetka

To go to rozetka.com.ua:

> rz

To make a search on rozetka:

> rz _search_query_
