use std::error::Error;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Articles {
    pub articles: Vec<Article>,
}

#[derive(Deserialize, Debug)]
pub struct Article {
    pub title: String,
    pub url: String,
}

pub fn get_articles(url: &str, api_key: &str) -> Result<Articles, Box<dyn Error>> {
    let response = ureq::get(&url)
        .set("X-Api-Key", api_key)
        .call()?
        .into_string()?;

    let articles: Articles = serde_json::from_str(&response)?;


    Ok(articles)
}
