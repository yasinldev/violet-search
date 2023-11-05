use serde_json::{json, Value};

pub(crate) fn handle_query(json_str: String, key_name: &str) -> String {
    let parsed_json: Result<Value, _> = serde_json::from_str(&json_str);

    return if let Ok(json_value) = parsed_json {
        if let Some(value) = json_value.get(key_name) {
            value.to_string()
        } else {
            json!({"error": "Key not found"}).to_string()
        }
    } else {
        json!({"error": "Invalid JSON"}).to_string()
    }
}

pub(crate) fn handle_duckduckgo_thumbnails(html: String, img_path: String) -> String {
    let parsed_html = scraper::Html::parse_document(&html);
    let selector = scraper::Selector::parse("img.tile--img__img").unwrap();

    let mut img_src = String::new();

    for element in parsed_html.select(&selector) {
        let src = element.value().attr("src").unwrap();
        img_src = src.to_string();
    }

    let img_src = img_src.replace("u=", "");
    let img_src = img_src.replace("&f=1", "");

    let img_src = format!("{}{}", "https://duckduckgo.com", img_src);

    let res = reqwest::blocking::get(&img_src).unwrap().bytes().unwrap();

    std::fs::write(img_path, res).unwrap();

    img_src
}
