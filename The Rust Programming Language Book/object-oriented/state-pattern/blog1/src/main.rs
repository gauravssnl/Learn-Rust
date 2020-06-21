use blog1::Post;

fn main() {
    let mut post = Post::new();
    let text = "Rust is very cool";

    post.add_text(text);
    // assert_eq!("", post.content());  // uncomment this to see error DrafPost has no content method

    let post = post.request_review();
    // assert_eq!("", post.content());     // uncomment this to see error PendingReviewPost has no content method

    let post = post.approve(); // returns Post struct
    assert_eq!(text, post.content()); // approved Post has conent() method
}
