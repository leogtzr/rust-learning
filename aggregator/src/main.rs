use aggregator::{NewsArticle, SocialPost, Summary};

fn main() {
    let news_article = NewsArticle { 
        headline: String::from("h1"), location: String::from("MX"), author: String::from("leogtzr"), content: String::from("<body>"),
    };

    let post = SocialPost {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        repost: false,
    };

    let article = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from(
            "The Pittsburgh Penguins once again are the best \
             hockey team in the NHL.",
        ),
    };

    let post2 = SocialPost {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        repost: false,
    };

    println!("1 new post: {}", post2.summarize());
    println!("New article available! {}", article.summarize());
    println!("Article: {}", news_article.summarize());
    println!("1 new post: {}", post.summarize());
}