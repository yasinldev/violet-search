/*
    Creating a client with reqwest for requests
*/

use reqwest;

pub(crate) struct Client {
    pub user_agent: String,
}

impl Client {
    pub fn new(user_agent: String) -> Client {
        Client {
            user_agent,
        }
    }

    pub async fn create_client(user_agent: String, url: String) -> String {
        let client = reqwest::Client::new();
        let res = client
            .get(url)
            .header("User-Agent", user_agent)
            .send()
            .await
            .unwrap()
            .text()
            .await
            .unwrap();

        res
    }
}
