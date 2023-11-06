use tor_client::{TorClient, Error};

pub struct ReturnConnection {
    pub title: String,
    pub description: String,
    pub url: String,
}

pub struct TorConnection<'a> {
    pub tor_client: TorClient,
    pub search_engine: String,
}

impl TorConnection<'a> {
    pub async fn new(search_engine: String) -> Result<TorConnection<'a>, Error> {
        let tor_client = TorClient::new().await?;

        Ok(TorConnection {
            tor_client,
            search_engine,
        })
    }

    pub async fn get_connection(&self, query: String) -> Result<ReturnConnection, Error> {
        let url = format!("{}{}", self.search_engine, query);
        let response = self.tor_client.get(&url).await?;

        match &self.search_engine {
            "https://3g2upl4pq6kufc4m.onion/" => {
                let title = response
                    .find("title")
                    .nth(0)
                    .unwrap()
                    .text()
                    .collect::<Vec<_>>()
                    .join("");
                let description = response
                    .find("p")
                    .nth(0)
                    .unwrap()
                    .text()
                    .collect::<Vec<_>>()
                    .join("");
                let url = response.url().to_string();

                Ok(ReturnConnection {
                    title,
                    description,
                    url,
                })
            }
            "https://ahmia.fi/search/" => {
                let title = response
                    .find("h3")
                    .nth(0)
                    .unwrap()
                    .text()
                    .collect::<Vec<_>>()
                    .join("");
                let description = response
                    .find("p")
                    .nth(0)
                    .unwrap()
                    .text()
                    .collect::<Vec<_>>()
                    .join("");
                let url = response.url().to_string();

                Ok(ReturnConnection {
                    title,
                    description,
                    url,
                })
            }
            "https://darksearch.io/search" => {
                let title = response
                    .find("h3")
                    .nth(0)
                    .unwrap()
                    .text()
                    .collect::<Vec<_>>()
                    .join("");
                let description = response
                    .find("p")
                    .nth(0)
                    .unwrap()
                    .text()
                    .collect::<Vec<_>>()
                    .join("");
                let url = response.url().to_string();

                Ok(ReturnConnection {
                    title,
                    description,
                    url,
                })
            }
        }
    }
}