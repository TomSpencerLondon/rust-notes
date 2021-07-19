use summary_trait_revision_10::{NewsArticle, Tweet, notify, BlogPost, largest};

use std::fmt::Display;

pub fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
    where
        T: Display,
{
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    let tweet = Tweet {
        username: String::from("tomspencer"),
        content: String::from("Rust tweet!"),
        reply: false,
        retweet: false
    };

    let news_article = NewsArticle {
        headline: String::from("News item!"),
        location: String::from("London"),
        author: String::from("Tom"),
        content: String::from("This is the content!")
    };

    let blog_post = BlogPost {
        name: String::from("This is a blog about traits"),
        content: String::from("Clever traits!"),
        author: String::from("Tom")
    };

    notify(tweet);
    notify(news_article);
    notify(blog_post);

    let number_list = vec![34, 50, 25, 100, 65];

    let char_list = vec!['y', 'm', 'a', 'q'];
    println!("The largest char is {}", largest(&number_list));

    let string_1 = String::from("First");
    let string_2 = String::from("Second");

    println!("{}", longest_with_an_announcement(
        &string_1,
        &string_2,
        String::from("This is an announcement"),
    ));
}
