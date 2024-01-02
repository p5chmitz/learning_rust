use crate::util::time;

/**The trait is Metadata, each behavior of the trait is represented by an "incomplete" function that
 * is ultimately defined by the implementing type; Any type with the Metadata trait will have access
 * to the functions defined within the trait block*/
pub trait Metadata {
    fn default(&self) -> String {
        format!(
            "\n\tAuthor: {}\n\tTime: {}",
            &self.author(),
            time::static_time(8)
        )
    }
    fn author(&self) -> String;
    fn summarize(&self) -> String;
}

//===================================
// News article

/**Struct NewsArticle implements the Metadata trait*/
pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}
//Implements the Metadata trait for the NewsArticle type
impl Metadata for NewsArticle {
    fn author(&self) -> String {
        String::from(&self.author)
    }
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

//===================================
// Tweet

/**Struct Tweet implements the Metadata trait*/
pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}
//Implements the Metadata trait for the Tweet type
impl Metadata for Tweet {
    fn author(&self) -> String {
        String::from(&self.username)
    }
    fn summarize(&self) -> String {
        format!("\n\t{}: {}", self.username, self.content)
    }
}

//===================================
// Calling code

/**Creates two structs that implement summarize() and default() traits, prints them*/
pub fn traits_1() {
    //Instantiates a news article to summarize
    let news_article = NewsArticle {
        headline: String::from("The oppression of indiginous communities"),
        location: String::from("North Dakota"),
        author: String::from("Peter Schmitz"),
        content: String::from(
            "This is gonna be super long bro Im not entirely sure you're ready for this yet",
        ),
    };
    let news_default = news_article.default();
    let news_summary = news_article.summarize();
    //Instantiates a tweet to summarize
    let tweet = Tweet {
        username: String::from("pschmitz"),
        content: String::from(
            "This is a tweet so its gonna be a bit shorter than a news article. Its mostly jokes.",
        ),
        reply: false,
        retweet: true,
    };
    let tweet_default = tweet.default();
    let tweet_summary = tweet.summarize();
    println!(
        "News article default: {}\nNews article summary: {}\nTweet default: {}\nTweet summary: {}",
        news_default, news_summary, tweet_default, tweet_summary
    );
}

/**Illustrates traits as function parameters*/
pub fn traits_2(item: &impl Metadata) {
    println!("Breaking Twitter news! {}", item.summarize());
}

/**Does the same thing as traits_2() but without the syntax sugar; This form more clearly defines
 * the generic function parameter*/
pub fn traits_3<T: Metadata>(item: &T) {
    format!("TESTING More breaking news! {}", item.summarize());
    println!("The thing we thought: {}", item.default());
}
pub fn traits_4<S>(s: &S)
where
    S: std::fmt::Display,
{
    println!("Generic types without trait-bound syntax: {}", s)
}

