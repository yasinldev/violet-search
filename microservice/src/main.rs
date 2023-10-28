use tokio::net::{TcpListener, TcpStream as TokioTcpStream};
use tokio::spawn;
use serde::{Deserialize, Serialize};
use colored::*;
use futures_util::SinkExt;
use tokio_tungstenite::{accept_async, tungstenite};
use futures_util::stream::StreamExt;
use crate::engines::dropdowns::dropdowns::Dropdowns;
use crate::engines::client::client::Client;
use crate::exceptions::exceptions::{throw_violet_search_exception, VioletSearchExceptions};

pub mod helpers;
pub mod exceptions;
pub mod engines;

#[derive(Debug, Serialize, Deserialize)]
pub struct JsonData {
    pub lang: String,
    pub search_engine: String,
    pub search_type: String,
    pub user_agent: String,
    pub query: String,
    pub use_proxy: String,
}

impl JsonData {
    pub fn new(lang: String, search_engine: String, search_type: String, user_agent: String, query: String, use_proxy: String) -> Self {
        Self {
            lang,
            search_engine,
            search_type,
            user_agent,
            query,
            use_proxy
        }
    }

    pub fn get_param(&self, json_data_param: &str) -> String {
        match json_data_param {
            "lang" => self.lang.clone(),
            "search_engine" => self.search_engine.clone(),
            "search_type" => self.search_type.clone(),
            "user_agent" => self.user_agent.clone(),
            "query" => self.user_agent.clone(),
            "use_proxy" => self.use_proxy.clone(),
            _ => panic!("Error: JsonData param not found"),
        }
    }

    pub async fn get_dropdown_data(&self, query: &str) -> Dropdowns {
        let lang = &self.lang;
        let user_agent = &self.user_agent;
        let search_engine = &self.search_engine;

        Dropdowns::new(search_engine.to_string(), query.to_string(), lang.to_string(), user_agent.to_string())
            .scrap_dropdowns()
            .await
    }
}


#[tokio::main]
async fn main() {
    let server = TcpListener::bind("127.0.0.1:3000")
        .await.
        unwrap();

    print!("{}[2J", 27 as char);

    println!(
        "{}",
        "SUCCESS:    Server listening on ws://127.0.0.1:3000 \n"
            .green()
            .bold()
    );

    while let Ok((stream, _)) = server.accept().await {
        spawn(handle_connection(stream));
    }
}

async fn handle_connection(stream: TokioTcpStream) {
    let ws_stream = accept_async(stream)
        .await
        .expect("Error during WebSocket handshake");

    let (mut write, mut read) = ws_stream.split();

    while let Some(result) = read.next().await {
        match result {
            Ok(message) => {
                if let Ok(call_json) = serde_json::from_str::<JsonData>(&message.to_string()) {
                    let query = call_json.query.clone();
                    println!(
                        "{} {}",
                        "INFO:    Violet Search is scraping from".bright_blue(),
                        &call_json.get_param("search_engine").bright_blue()
                    );

                    match call_json.search_type.as_str().clone() {
                        "dropdown" => {
                            let dropdown_result = call_json.get_dropdown_data(query.as_str()).await;

                            write.send(tungstenite::Message::Text(dropdown_result.results.to_string()))
                                .await
                                .unwrap();
                        }
                        "web" => {
                            if &call_json.use_proxy == "true" {
                                println!(
                                    "{}",
                                    "INFO:    Violet Search is using Violet Proxy".bright_blue()
                                );
                                
                                // creating a client
                                let res = Client::connect_violet_proxy(
                                    call_json.user_agent.clone(),
                                    "127.0.0.1".to_string(),
                                    "8080".to_string()
                                ).await;
                                println!("{}", res);
                            }
                        }
                        _ => throw_violet_search_exception(
                            VioletSearchExceptions::VioletSearchInvalidParameterException(
                                "Error: Undefined search type".to_string()
                            )
                        )
                    }
                }
            }
            Err(e) => {
                eprintln!("Error reading WebSocket message: {:?}", e);
                break;
            }
        }
    }
}
