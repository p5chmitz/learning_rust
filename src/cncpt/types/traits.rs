
/**The trait is Summary, each behavior of the trait is represented by an "incomplete" function that
 * is ultimately defined by the implementing type; Any type with the Summary trait will have access
 * to the functions defined within the trait block*/
pub trait Summary {
    fn summarize(&self) -> String;
}

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
impl Summary for NewsArticle {
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
impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

