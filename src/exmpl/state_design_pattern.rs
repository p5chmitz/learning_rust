// Faithfully implements the state design pattern
pub mod original {
    // Something like a "traditional" OOP state design pattern
    // This example illustrates a blog post with multiple states as trait objects
    // Using an enum instead of structs means that every place that checks the value of
    // the enum will need a match-like expression to handle every possible variant.
    // This could get repetitive. Structs dont require the bloat.
    // Transformations are encapsulated entirely within the Post struct, as per
    // OOP and the state design pattern. However this means that invalid states are
    // possible because nothing is checked at compile time.
    pub struct Post {
        state: Option<Box<dyn State>>,
        content: String,
    }

    impl Post {
        // Creates a new post instance in a Draft state with an empty content string
        pub fn new() -> Post {
            Post {
                state: Some(Box::new(Draft {})),
                content: String::new(),
            }
        }
        // Adds content to a Draft post
        // This makes adding text intentional instead of being
        // able to mutate the struct instance
        pub fn add_text(&mut self, text: &str) {
            self.content.push_str(text);
        }
        // Retrieves the content of a Draft
        pub fn content(&self) -> &str {
            self.state.as_ref().unwrap().content(self)
        }
        // First state change method! Moves Post from Draft to Pending Review
        // Calls take() on Option to consume the value and leaves None.
        // The request_review method on Post is the same no matter its state value.
        // Each state is responsible for its own rules.
        pub fn request_review(&mut self) {
            if let Some(s) = self.state.take() {
                // Consumes the current state and returns a new state
                self.state = Some(s.request_review())
            }
        }
        // Changes state from PendingReview to Published
        pub fn approve(&mut self, counter: u8) {
            if let Some(s) = self.state.take() {
                self.state = Some(s.approve(counter))
            }
        }
        // Changes state from PendingReview to Draft
        pub fn reject(&mut self) {
            if let Some(s) = self.state.take() {
                self.state = Some(s.reject())
            }
        }
    }

    // The heart of the state design patter is... the state
    trait State {
        // Required methods
        // Parmeter syntax takes ownership of Box<Self>, invalidating the old
        // state so the state value of the Post can transform into a new state.
        fn request_review(self: Box<Self>) -> Box<dyn State>;
        fn approve(self: Box<Self>, counter: u8) -> Box<dyn State>;
        fn reject(self: Box<Self>) -> Box<dyn State>;

        // Provided methods
        // Required for deref conversion on Post's content() method.
        // This represents a default method.
        // Requires lifetimes because its taking a reference to a post
        // as an argument and returning a reference to part of that post,
        // so the lifetime of the returned reference is related to the
        // lifetime of the post argument.
        fn content<'a>(&self, post: &'a Post) -> &'a str {
            ""
        }
    }

    // Post states are implemented as structs which define the Post structs behaviors
    struct Draft {}
    impl State for Draft {
        fn request_review(self: Box<Self>) -> Box<dyn State> {
            Box::new(PendingReview {})
        }
        fn approve(self: Box<Self>, counter: u8) -> Box<dyn State> {
            self
        }
        fn reject(self: Box<Self>) -> Box<dyn State> {
            self
        }
    }

    struct PendingReview {}
    impl State for PendingReview {
        fn request_review(self: Box<Self>) -> Box<dyn State> {
            self
        }
        fn approve(self: Box<Self>, counter: u8) -> Box<dyn State> {
            if counter < 2 {
                self
            } else {
                Box::new(Published {})
            }
        }
        fn reject(self: Box<Self>) -> Box<dyn State> {
            Box::new(Draft {})
        }
    }

    struct Published {}
    impl State for Published {
        fn request_review(self: Box<Self>) -> Box<dyn State> {
            self
        }
        fn approve(self: Box<Self>, counter: u8) -> Box<dyn State> {
            self
        }
        fn reject(self: Box<Self>) -> Box<dyn State> {
            self
        }
        // Published is the only state that returns content
        fn content<'a>(&self, post: &'a Post) -> &'a str {
            &post.content
        }
    }

    #[cfg(test)]
    pub mod tests {
        use super::Post;

        #[test]
        fn e2e() {
            let mut post = Post::new();

            post.add_text("I ate garbage for lunch today");
            assert_eq!("", post.content());

            post.request_review();
            assert_eq!("", post.content());

            // External counter required for multiple approvals
            let mut counter: u8 = 1;

            // Approval 1
            post.approve(counter);
            assert_eq!("", post.content());
            counter += 1;

            // Approval 2
            post.approve(counter);
            assert_eq!("I ate garbage for lunch today", post.content());
        }
    }
}

// Modifies the OG pattern to be more idiomatic
// Improvements include:
// - Encodes the states and transitions into different types rather than
//   encapsulating them such that outside code has no knowledge of them. This
//   results in compiler errors if you try to misuse the states which provides more
//   up-front safety. This also encodes the workflow into the type system.
// - Invalid states are not possible and checked at compile time
pub mod modified {
    // Published post
    pub struct Post {
        content: String,
    }
    // Unpublished draft post
    pub struct DraftPost {
        content: String,
    }
    // Unpublished pending review
    pub struct PendingReviewPost {
        content: String,
    }

    // Struct impelmentations
    impl Post {
        pub fn new() -> DraftPost {
            DraftPost {
                content: String::new(),
            }
        }
        pub fn content(&self) -> &str {
            &self.content
        }
    }
    impl DraftPost {
        pub fn add_text(&mut self, text: &str) {
            self.content.push_str(text);
        }
        pub fn request_review(self) -> PendingReviewPost {
            PendingReviewPost {
                content: self.content,
            }
        }
    }
    impl PendingReviewPost {
        pub fn approve(self) -> Post {
            Post {
                content: self.content,
            }
        }
    }

    // The heart of the state design patter is... the state
    trait State {
        // Required methods
        // Parmeter syntax takes ownership of Box<Self>, invalidating the old
        // state so the state value of the Post can transform into a new state.
        fn request_review(self: Box<Self>) -> Box<dyn State>;
        fn approve(self: Box<Self>, counter: u8) -> Box<dyn State>;
        fn reject(self: Box<Self>) -> Box<dyn State>;

        // Provided methods
        // Required for deref conversion on Post's content() method.
        // This represents a default method.
        // Requires lifetimes because its taking a reference to a post
        // as an argument and returning a reference to part of that post,
        // so the lifetime of the returned reference is related to the
        // lifetime of the post argument.
        fn content<'a>(&self, post: &'a Post) -> &'a str {
            ""
        }
    }

    // Post states are implemented as structs which define the Post structs behaviors
    struct Draft {}
    impl State for Draft {
        fn request_review(self: Box<Self>) -> Box<dyn State> {
            Box::new(PendingReview {})
        }
        fn approve(self: Box<Self>, counter: u8) -> Box<dyn State> {
            self
        }
        fn reject(self: Box<Self>) -> Box<dyn State> {
            self
        }
    }

    struct PendingReview {}
    impl State for PendingReview {
        fn request_review(self: Box<Self>) -> Box<dyn State> {
            self
        }
        fn approve(self: Box<Self>, counter: u8) -> Box<dyn State> {
            if counter < 2 {
                self
            } else {
                Box::new(Published {})
            }
        }
        fn reject(self: Box<Self>) -> Box<dyn State> {
            Box::new(Draft {})
        }
    }

    struct Published {}
    impl State for Published {
        fn request_review(self: Box<Self>) -> Box<dyn State> {
            self
        }
        fn approve(self: Box<Self>, counter: u8) -> Box<dyn State> {
            self
        }
        fn reject(self: Box<Self>) -> Box<dyn State> {
            self
        }
        // Published is the only state that returns content
        fn content<'a>(&self, post: &'a Post) -> &'a str {
            &post.content
        }
    }

    #[cfg(test)]
    pub mod tests {
        use super::Post;

        #[test]
        fn e2e() {
            let mut post = Post::new();

            post.add_text("I ate a salad for lunch today");

            // Shaddowing assignments save the returned instances
            let post = post.request_review();

            let post = post.approve();

            assert_eq!("I ate a salad for lunch today", post.content());
        }
    }
}
