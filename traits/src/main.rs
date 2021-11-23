use traits::{Summary, Tweet};
use std::fmt::Display;

struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T: Display + PartialOrd> Pair<T> {
    fn cmd_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}

fn main() {
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        retweet: false,
    };

    let tweet2 = Tweet {
        username: String::from("nnoce14"),
        content: String::from("some content"),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}", tweet.summarize());
    println!("");
    notify(&tweet);
    println!("");
    notify2(&tweet);
    println!("");
    notify_tweets(&tweet, &tweet2);
    println!("");

    let pair = Pair::new(String::from("abc"), String::from("def"));
    let pair2 = Pair::new(5, 10);

    pair.cmd_display();
    pair2.cmd_display();
}

pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

pub fn notify2<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}

pub fn notify_tweets<T: Summary>(item1: &T, item2: &T) {
    println!("Breaking news!\n > {}\n > {}", item1.summarize(), item2.summarize());
}

fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("nnoce14"),
        content: String::from(
            "Jets will beat the dolphins this week with Joe Flacco",
        ),
        reply: false,
        retweet: false,

    }
}


