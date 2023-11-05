extern crate reqwest;
use select::document::Document;
use select::predicate::Name;

pub struct DuckDuckGo {
    pub img_result: Vec<String>,
    pub title_result: Vec<String>,
    pub url_result: Vec<String>,
    pub desc_result: Vec<String>,
}

impl DuckDuckGo {
    pub async fn scrap_duckduckgo(query: String, user_agent: String) -> Result<DuckDuckGo, Box<dyn std::error::Error>> {
        let url = format!("https://duckduckgo.com/html/?q={}&kl={}", query, user_agent);
        let res = reqwest::get(&url).await?;
        let body = res.text().await?;


        let document = Document::from_read(body.as_bytes()).unwrap();
        let titles: Vec<String> = document
        .find(Name("a"))
        .filter_map(|n| {
            if let Some(title) = n.attr("class") {
                if title == "result__a" {
                    return Some(n.text());
                }
            }
            None
        })
        .collect();
        
        let urls: Vec<String> = document
        .find(Name("a"))
        .filter_map(|n| {
            if let Some(title) = n.attr("class") {
                if title == "result__a" {
                    return n.attr("href").map(|s| s.to_string());
                }
            }
            None
        })
        .collect();

        let descs: Vec<String> = document
        .find(Name("a"))
        .filter_map(|n| {
            if let Some(title) = n.attr("class") {
                if title == "result__snippet" {
                    return Some(n.text());
                }
            }
            None
        })
        .collect();

        let img: Vec<String> = document 
        .find(Name("img"))
        .filter_map(|n| {
            if let Some(title) = n.attr("class") {
                if title == "result__icon__img" {
                    return n.attr("src").map(|s| s.to_string());
                }
            }
            None
        })
        .collect();

        let duckduckgo = DuckDuckGo {
            title_result: titles.clone(),
            url_result: urls.clone(),
            desc_result: descs.clone(),
            img_result: img.clone(),
        };
        
        Ok(duckduckgo)
        
    }
}
