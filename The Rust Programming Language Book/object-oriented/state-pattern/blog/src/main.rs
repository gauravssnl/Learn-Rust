use blog::Post;

fn main() {
    let mut post = Post::new();
    let text = "Rust is very cool";

    post.add_text(text);
    assert_eq!("", post.content());

    post.request_review();
    assert_eq!("", post.content());

    post.approve();
    assert_eq!(text, post.content());
}
