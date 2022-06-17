use std::fmt::Display;

fn main() {
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };

    notify("tweet", &tweet);
    tweet.print();
    // println!("tweet.summarize() = {}", tweet.summarize());

    let article = NewsArticle {
        headline: String::from("13 Thing You Didn't Know about Rust!"),
        location: String::from("rust-lang.org"),
        author: String::from("Rusty Rustacean"),
        content: String::from("looooong loooooong maaaaaaaan"),
    };

    notify("article", &article);
    article.print();
    // println!("article.summarize() = {}", article.summarize());

    let blog = Blog {
        domain: String::from("MySpace"),
        title: String::from("My Top 8 🥰"),
        content: String::from("looooong loooooong maaaaaaaan"),
    };

    notify("blog", &blog);
    blog.print();
    // println!("blog.summarize() = {}", blog.summarize())

    notify("returns_summarizable()", &returns_summarizable());
}

// pub fn notify(name: &str, item: &impl Summary) {
//     println!("🚨 {} = {}", name, item.summarize());
// }

// pub fn notify<T: Summary>(name: &str, item: &T) {
//     println!("🚨 {} = {}", name, item.summarize());
// }

pub fn notify<T: Summary + Display>(name: &str, item: &T) {
    // NOTE: we are using `{}` not `{:?}` because we implemented `Display` trait 🥳
    println!("🚨 {} = {}", name, item);
}

pub trait Summary {
    fn summarize_author(&self) -> String {
        return String::from("Anonymous");
    }

    fn summarize(&self) -> String {
        return format!("Read more from {}...", self.summarize_author());
    }
}

pub trait Printable {
    fn print(&self);
}

// define a new trait on all types that implement both Summary and Display
// this means any type that implements Summary and Display can call a `print` method
// e.g. NewsArticle.print(), Tweet.print(), Blog.print()
impl<T: Summary + Display> Printable for T {
    fn print(&self) {
        println!("🖨️ {}", self);
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        return format!(
            r#""{}", by {} ({})"#,
            self.headline,
            self.summarize_author(),
            self.location
        );
    }

    fn summarize_author(&self) -> String {
        return format!("{}", self.author);
    }
}

impl Display for NewsArticle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return write!(f, "{}", self.summarize());
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        return format!("@{}: {}", self.username, self.content);
    }

    fn summarize_author(&self) -> String {
        return format!("@{}", self.username);
    }
}

impl Display for Tweet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return write!(f, "{}", self.summarize());
    }
}

pub struct Blog {
    pub domain: String,
    pub title: String,
    pub content: String,
}

// default impl
impl Summary for Blog {}

impl Display for Blog {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return write!(f, "{}", self.summarize());
    }
}

// // trait bounds can become verbose and hard to read
// fn some_function_a<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 {
//     return 42;
// }

// // use where clause to make fn signature more readable
// fn some_function_b<T, U>(t: &T, u: &U) -> i32
// where
//     T: Display + Clone,
//     U: Clone + Debug,
// {
// }

fn returns_summarizable() -> impl Summary + Display {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    }
}
