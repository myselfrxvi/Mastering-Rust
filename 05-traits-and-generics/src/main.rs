fn main() {
    println!("Hello, world!");
    let news = NewsArticle {
        headline: String::from("New virus"),
        content: String::from("A new virus has broken out"),
        author: String::from("John Doe"),
    };
    let tweet = Tweet {
        author: String::from("rustlang"),
        message: String::from("Rust 1.85 is out! 🦀"),
    };
    print_Summary(&news);
    print_Summary(&tweet)
}

trait Summary {
    fn summarize(&self) -> String;
}
struct NewsArticle {
    headline: String,
    content: String, 
    author: String,
}

struct Tweet {
    author: String, 
    message: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{} by {}",self.headline, self.author)
    }
}

impl Summary for Tweet{
    fn summarize(&self) -> String{
        format!("@{} {}", self.author, self.message)
    }
}

fn print_Summary(item: &impl Summary) {
    println!("{}", item.summarize());
}