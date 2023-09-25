/*
    Scrapping dropdowns
*/

use reqwest;
use serde_json;
use xml::reader::XmlEvent;  // for google

use crate::engines::client::client::Client;
use crate::exceptions::exceptions::throw_violet_search_exception;
use crate::exceptions::exceptions::VioletSearchExceptions::VioletSearchUndefinedException;
use crate::helpers::helpers::handle_query;

pub struct Dropdowns {
    pub engine: String,
    pub query: String,
    pub lang: String,
    pub user_agent: String,
    pub results: serde_json::Value,
}

impl Dropdowns {
    pub fn new(engine: String, query: String, lang: String, user_agent: String) -> Dropdowns {
        Dropdowns {
            engine,
            query,
            lang,
            user_agent,
            results: serde_json::Value::Null,
        }
    }

    pub async fn scrap_dropdowns(&self) -> Dropdowns {
        handle_query(self.query.clone(), "query");

        // Checking engine
        let engine = match self.engine.as_str() {
            "artado" => format!("https://www.artadosearch.com/api/autocomplete?q={}", self.query),
            "google" => format!("https://suggestqueries.google.com/complete/search?output=toolbar&hl={}&q={}", self.lang, self.query),
            "duckduckgo" => format!("https://duckduckgo.com/ac/?q={}&kl={}", self.query, self.lang),
            "qwant" => format!("https://api.qwant.com/api/suggest/?q={}&client=opensearch&lang={}", self.query, self.lang),
            "yahoo" => format!("https://search.yahoo.com/sugg/gossip/gossip-us-ura/?output=sd1&command={}", self.query),
            "swisscows" => format!("https://swisscows.com/api/suggest?query={}&lang={}", self.query, self.lang),
            "ecosia" => format!("https://ac.ecosia.org/autocomplete?q={}", self.query),
            "ask" => format!("https://amg-ss.ask.com/query?q={}", self.query),
            "brave" => format!("https://search.brave.com/api/suggest?q={}", self.query),
            _ => throw_violet_search_exception(
                VioletSearchUndefinedException("Error:    Engine not found".to_string())
            )
        };

        let res = Client::create_client(self.user_agent.clone(), engine).await;
        /*if engine == "google".to_string() {
            let mut reader = xml::reader::EventReader::from_str(&res);
            let mut buf = Vec::new();
            loop {
                match reader.next() {
                    Ok(XmlEvent::Characters(e)) => buf.push(e),
                    Err(_) => break,
                    _ => (),
                }
            }
            let res = buf.join("");

            // Convert the xml data to json
            let res = res.replace("window.google.ac.h(", "");
            let res = res.replace(")", "");
            let res = res.replace(",,", ",\"\",");
            let res = res.replace(",,", ",\"\",");
            let res = res.replace(",,", ",\"\",");
            let res = res.replace(",,", ",\"\",");

        }*/

        // Getting the json data
        let json: serde_json::Value = serde_json::from_str(&res).unwrap();

        Dropdowns {
            engine: self.engine.clone(),
            query: self.query.clone(),
            lang: self.lang.clone(),
            user_agent: self.user_agent.clone(),
            results: json,
        }
    }
}
