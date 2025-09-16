use aggregator::{NewsArticle, Summary};

fn main() {
    let news_article = NewsArticle { 
        headline: String::from("h1"), location: String::from("MX"), author: String::from("leogtzr"), content: String::from("<body>"),
    };

    println!("Article: {}", news_article.summarize());
}