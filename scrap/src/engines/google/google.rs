/*
    Scraping Google Search Results
*/

use reqwest;
use scraper::{Html, Selector};
use serde_json::de::Read;

use crate::helpers::helpers::handle_query;
use crate::engines::client::client::Client;

pub struct Google {
    pub search_type: String,
    pub user_agent: String,
    pub lang: String,
}

impl Google {
    pub fn new(search_type: String, user_agent: String, lang: String) -> Google {
        Google {
            search_type,
            user_agent,
            lang,
        }
    }

    pub async fn scrap_google_results(query: String, lang: String, user_agent: String) -> () {
        // Converting the Json query to a String
        if let get_query = handle_query(query, "query") {
            let url = format!("https://google.com/search?q={}&hl={}", get_query, lang);
            /*
                EXAMPLE:
                https://www.google.com/complete/search?q=hello&hl=en
                query from "{\"query\":\"hello\"}" to "hello" so key_name is "query" default
             */

            let res = Client::create_client(user_agent, url).await;

            let document = Html::parse_document(&res);
            let selector = Selector::parse("h3").unwrap();

            // println!("{:?}", document);

            for element in document.select(&selector) {
                let text = element.text().collect::<Vec<_>>();
                println!("{:?}", text);
            }
        }
        ()
    }
}
