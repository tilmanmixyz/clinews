use std::error::Error;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct Articles {
    articles: Vec<Article>,
}

#[derive(Deserialize, Debug)]
struct Article {
    title: String,
    url: String,
}

fn get_articles(url: &str, api_key: &str) -> Result<Articles, Box<dyn Error>> {
    let response = ureq::get(&url)
        .set("X-Api-Key", api_key)
        .call()?
        .into_string()?;

    let articles: Articles = serde_json::from_str(&response)?;


    Ok(articles)
}

fn render_articles(articles: Articles) {
    for a in &articles.articles {
        println!("{}", a.title);
        println!("{}\n\n", a.url);
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let api_key: &str = "f5fd5e5064bf44ae862e45f6a08c945a";
    let url: &str = "https://newsapi.org/v2/top-headlines?country=de";

    let articles = get_articles(url, api_key)?;

    render_articles(articles);

    Ok(())
}
