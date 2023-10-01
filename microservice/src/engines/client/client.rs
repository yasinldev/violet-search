/*
    Creating a client with reqwest for requests
*/

use reqwest;

pub(crate) struct Client;

impl Client {
    pub async fn create_client(user_agent: String, url: String) -> String {
        let client = reqwest::Client::new();
        let res = client
            .get(url)
            .header("User-Agent", user_agent.clone())
            .send()
            .await
            .unwrap()
            .text()
            .await
            .unwrap();

        res
    }

    pub async fn connect_violet_proxy(user_agent: String, url: String, port: String) -> String {
        let mut violet_headers = reqwest::header::HeaderMap::new();
        violet_headers.insert("User-Agent", reqwest::header::HeaderValue::from_str(&user_agent).unwrap());

        let client = reqwest::ClientBuilder::new()
            .proxy(reqwest::Proxy::all(&format!("http://{}:{}", url, port)).unwrap())
            .default_headers(violet_headers)
            .build()
            .unwrap();

        let url = format!("http://{}:{}", url, port);

        let res = client.get(url).send().await.unwrap().text().await.unwrap();
        match res {
            _ => res
        }
    }
}
