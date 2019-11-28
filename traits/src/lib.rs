pub trait Summary {
    fn summarize(&self) -> String;

    fn summarize_author(&self) -> String;

    // default method
    fn default_summarize(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author()) // call another method
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
        format!("{} by {} {}", self.headline, self.author, self.location)
    }

    fn summarize_author(&self) -> String {
        format!("@{}", self.author)
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
        format!("{}: {}", self.username, self.content)
    }

    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }

    // override default method of trait
    fn default_summarize(&self) -> String {
        format!("User: {}, Tweet: {}", &self.username, &self.content)
    }
}
