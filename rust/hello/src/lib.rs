pub trait Summarizable {
    fn author_summary(&self) -> String;

    fn summary(&self) -> String {
        format!("(Read more from {}...)", self.author_summary())
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summarizable for NewsArticle {
    fn author_summary(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
    fn summary(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summarizable for Tweet {
    fn author_summary(&self) -> String {
        format!("@{}", self.username)
    }
    fn summary(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

#[cfg(test)]
mod tests {
    use crate::Tweet;
    use crate::Summarizable;

    #[test]
    fn it_works() {
        let tweet = Tweet {
            username: String::from("horse_ebooks"),
            content: String::from("of course, as you probably already know, people"),
            reply: false,
            retweet: false,
        };
        
        println!("1 new tweet: {}", tweet.summary());
        assert_eq!(4, 4);
    }
}
