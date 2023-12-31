/**The trait is Summary, each behavior of the trait is represented by an "incomplete" function that
 * is ultimately defined by the implementing type; Any type with the Summary trait will have access
 * to the functions defined within the trait block*/
pub trait Summary {
    fn default(&self) -> String {
        String::from("(Placeholder text)")
    }
    //fn compare(&self) -> {}
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

//===================================
// Calling code

/**Creates two structs that implement summarize() and default() traits, prints them*/
pub fn traits_1() {
    //Instantiates a news article to summarize
    let news_article = NewsArticle {
        headline: String::from("The oppression of indiginous communities"),
        location: String::from("North Dakota"),
        author: String::from("Peter Schmitz"),
        content: String::from("This is gonna be super long bro Im not entirely sure you're ready for this yet"),
    };
    let news_summary = news_article.summarize();
    //Instantiates a tweet to summaryze
    let tweet = Tweet {
        username: String::from("pschmitz"),
        content: String::from("This is a tweet so its gonna be a bit shorter than a news article. Its mostly jokes."),
        reply: false,
        retweet: true,
    };
    let tweet_summary = tweet.default();
    println!("News article summary: {}\nTweet summary: {}", news_summary, tweet_summary);
}

/**Illustrates traits as function parameters*/
pub fn traits_2(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

/**Does the same thing as traits_2() but without the syntax sugar; This form more clearly defines
 * the generic function parameter*/
pub fn traits_3<T: Summary>(item: &T) {
    println!("More breaking news! {}", item.summarize());
    println!("The thing we thought: {}", item.default());
}


