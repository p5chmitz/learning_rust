
//===================================
// News article

/**Struct NewsArticle implements the Summary trait*/
pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}
//Implements the Summary trait for the NewsArticle type
impl crate::cncpt::traits::Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

//===================================
// Tweet

/**Struct Tweet implements the Summary trait*/
pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}
//Implements the Summary trait for the Tweet type
impl crate::cncpt::traits::Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

