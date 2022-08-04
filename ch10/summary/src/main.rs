use summary::{NewsArticle, notify, returns_summarizable, Summary, Tweet};

fn main() {
    let tweet = Tweet{
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };
    println!("1 new tweet: {}", tweet.summarize());
    
    let article = NewsArticle{
        headline: "Penguins win the Stanley Cup Championship!".to_string(),
        location: "Pittsburgh, PA, USA".to_string(),
        author: "Iceburgh".to_string(),
        content: "he Pittsburgh Penguins once again are the best \
             hockey team in the NHL.".to_string(),
    };
    // println!("new article available! {}", article.summarize());
    notify(article);

    let s = returns_summarizable();
    notify(s);
}
