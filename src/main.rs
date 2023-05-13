use std::error::Error;
use newsapi::{NewsApi, Article, Country, Endpoint};


fn render_articles(articles: &Vec<Article>) {
    for a in articles {
        println!("{}", a.title());
        println!("{}\n\n", a.url());
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let api_key: &str = "f5fd5e5064bf44ae862e45f6a08c945a";

    let mut newsapi = NewsApi::new(api_key);
    newsapi.endpoint(Endpoint::TopHeadlines).country(Country::De);

    let articles = newsapi.fetch_async().await?;

    render_articles(articles.articles());
    println!("News are provided by https://newsapi.org");

    Ok(())
}
