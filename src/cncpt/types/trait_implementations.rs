////===================================
//// News article
//
//**Struct NewsArticle implements the Metadata trait*/
//pub struct NewsArticle {
//    pub headline: String,
//    pub location: String,
//    pub author: String,
//    pub content: String,
//}
////Implements the Metadata trait for the NewsArticle type
//impl crate::cncpt::traits::Metadata for NewsArticle {
//    fn author(&self) -> String {
//        self.author
//    }
//    fn summarize(&self) -> String {
//        format!("{}, by {} ({})", self.headline, self.author, self.location)
//    }
//}
//
//
////===================================
//// Tweet
//
//**Struct Tweet implements the Metadata trait*/
//pub struct Tweet {
//    pub username: String,
//    pub content: String,
//    pub reply: bool,
//    pub retweet: bool,
//}
////Implements the Metadata trait for the Tweet type
//impl crate::cncpt::traits::Metadata for Tweet {
//    fn author(&self) -> String {
//        self.username
//    }
//    fn summarize(&self) -> String {
//        format!("{}: {}", self.username, self.content)
//    }
//}
