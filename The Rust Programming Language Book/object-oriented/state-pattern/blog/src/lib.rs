// Implementing an Object-Oriented Design Pattern - State Pattern

pub struct Post {
    state: Option<Box<dyn State>>,
    content: String,
}

impl Post {
    pub fn new() -> Post {
        Post {
            state: Some(Box::new(Draft {})),
            content: String::new(),
        }
    }

    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }

    pub fn content(&self) -> &str {
        self.state.as_ref().unwrap().content(self)
    }

    pub fn request_review(&mut self) {
        // To consume the old state, the request_review method needs to take ownership of the state value.
        // take() of Option<T> takes the value out of the option, leaving a None in its place.
        // because Rust doesn’t let us have unpopulated fields in structs
        // this lets us move the state value out of Post rather than borrowing it.
        if let Some(s) = self.state.take() {
            self.state = Some(s.request_review())
        }
    }

    pub fn approve(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.approve())
        }
    }
}

trait State {
    // This syntax means the method is only valid when called on a Box holding the type.
    fn request_review(self: Box<Self>) -> Box<dyn State>;
    fn approve(self: Box<Self>) -> Box<dyn State>;
    // default implementation for Draft, PendingReview states
    fn content<'a>(&self, _post: &'a Post) -> &'a str {
        ""
    }
}

struct Draft {}

impl State for Draft {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        Box::new(PendingReview {})
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        self // a post in Draft status should not be approved
    }
}

struct PendingReview {}

impl State for PendingReview {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self // post is already in PendingReview state
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        Box::new(Published {}) // set Post state to Published after the review
    }
}

struct Published {}

impl State for Published {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self // post is already in approved state
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        self // post is already in approved state
    }

    // override deafult content() of trait State  as we need to return the actual content
    fn content<'a>(&self, post: &'a Post) -> &'a str {
        &post.content
    }
}
