use serde::Deserialize;
use thiserror::Error;
use url::Url;

#[derive(Error, Debug)]
pub enum NewsApiError {
    #[error("Could not request server")]
    RequestError(#[from] ureq::Error),
    #[error("Could not parse into a string")]
    StringParseError(#[from] std::io::Error),
    #[error("Could not parse response into json")]
    JsonParseError(#[from] serde_json::Error),
    #[error("Could not parse url")]
    UrlParseError(#[from] url::ParseError),
    #[error("Request failed {0}")]
    BadRequest(&'static str),
    #[error("Unknown Erro")]
    UnknownError,
    #[error("Asnyc reqeust failed")]
    #[cfg(feature = "async")]
    AsyncRequestFailed(#[from] reqwest::Error),
}

const BASE_URL: &str = "https://newsapi.org/v2";

#[derive(Deserialize, Debug)]
pub struct NewsApiResponse {
    code: Option<String>,
    status: String,
    articles: Vec<Article>,
}

impl NewsApiResponse {
    pub fn articles(&self) -> &Vec<Article> {
        &self.articles
    }
}

#[derive(Deserialize, Debug)]
pub struct Article {
    title: String,
    url: String,
}

impl Article {
    pub fn title(&self) -> &str {
        &self.title
    }

    pub fn url(&self) -> &str {
        &self.url
    }
}

pub struct NewsApi {
    api_key: String,
    endpoint: Endpoint,
    country: Country,
}

pub enum Country {
    Us,
    De,
}

impl ToString for Country {
    fn to_string(&self) -> String {
        match self {
            Self::De => "de".to_string(),
            Self::Us => "us".to_string(),
        }
    }
}

pub enum Endpoint {
    TopHeadlines,
}

impl ToString for Endpoint {
    fn to_string(&self) -> String {
        match self {
            Self::TopHeadlines => "top-headlines".to_string(),
        }
    }
}

impl NewsApi {
    pub fn new(api_key: &str) -> NewsApi {
        NewsApi {
            api_key: api_key.to_string(),
            endpoint: Endpoint::TopHeadlines,
            country: Country::De,
        }
    }

    pub fn endpoint(&mut self, endpoint: Endpoint) -> &mut NewsApi{
        self.endpoint = endpoint;
        self
    }

    pub fn country(&mut self, country: Country) -> &mut NewsApi{
        self.country = country;
        self
    }

    fn prepare_url(&self) -> Result<String, NewsApiError> {
        let mut url = Url::parse(BASE_URL)?;
        url.path_segments_mut().unwrap().push(&self.endpoint.to_string());

        let country = format!("country={}", self.country.to_string());
        url.set_query(Some(&country));

        Ok(url.to_string())
    }

    pub fn fetch(&self) -> Result<NewsApiResponse, NewsApiError> {
        let url = self.prepare_url()?;
        let req = ureq::get(&url)
            .set("X-Api-Key", &self.api_key);

        let res: NewsApiResponse = req.call()?.into_json()?;

        match res.status.as_str() {
            "ok" => Ok(res),
            _ => Err(map_response_err(res.code))
        }
    }

    #[cfg(feature = "async")]
    pub async fn fetch_async(&self) -> Result<NewsApiResponse, NewsApiError> {
        let url = self.prepare_url()?;

        let client = reqwest::Client::new();
        let req = client
            .request(reqwest::Method::GET, url)
            .header("X-Api-Key", &self.api_key)
            .header("User-Agent", "clinews")
            .build()
            .map_err(|e| NewsApiError::AsyncRequestFailed(e))?;

        let res: NewsApiResponse = client
            .execute(req)
            .await?
            .json()
            .await
            .map_err(|e| NewsApiError::AsyncRequestFailed(e))?;

        match res.status.as_str() {
            "ok" => Ok(res),
            _ => Err(map_response_err(res.code))
        }
    }
}

fn map_response_err(code: Option<String>) -> NewsApiError {
    if let Some(code) = code {
        match code.as_str() {
            "apiKeyDisabled" => NewsApiError::BadRequest("Your API key has been disabled"),
            _ => NewsApiError::UnknownError,
        }
    } else {
        NewsApiError::UnknownError
    }
}

// pub fn get_articles(url: &str, api_key: &str) -> Result<Articles, NewsApiError> {
//     let response = ureq::get(&url)
//         .set("X-Api-Key", api_key)
//         .call()?
//         .into_string()?;
// 
//     let articles: Articles = serde_json::from_str(&response)?;
// 
// 
//     Ok(articles)
// }
