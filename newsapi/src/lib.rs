use serde::Deserialize;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum NewsApiError {
    #[error("Could not request server")]
    RequestError(ureq::Error),
    #[error("Could not parse into a string")]
    StringParseError(std::io::Error),
    #[error("Could not parse response into json")]
    JsonParseError(serde_json::Error),
}

#[derive(Deserialize, Debug)]
pub struct Articles {
    pub articles: Vec<Article>,
}

#[derive(Deserialize, Debug)]
pub struct Article {
    pub title: String,
    pub url: String,
}

pub fn get_articles(url: &str, api_key: &str) -> Result<Articles, NewsApiError> {
    let response = ureq::get(&url)
        .set("X-Api-Key", api_key)
        .call().map_err(|e| NewsApiError::RequestError(e))?
        .into_string().map_err(|e| NewsApiError::StringParseError(e))?;

    let articles: Articles = serde_json::from_str(&response)
        .map_err(NewsApiError::JsonParseError)?;


    Ok(articles)
}
