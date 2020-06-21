// Implementing an Object-Oriented Design Pattern - State Pattern
// Using different types for diffeerent State

pub struct Post {
    content: String,
}

pub struct DrafPost {
    content: String,
}

pub struct PendingReviewPost {
    content: String,
}

impl Post {
    pub fn new() -> DrafPost {
        DrafPost {
            content: String::new(),
        }
    }

    // we will use this Post method  once it is approved
    pub fn content(&self) -> &str {
        &self.content
    }
}

impl DrafPost {
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
