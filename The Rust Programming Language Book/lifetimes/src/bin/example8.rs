// Lifetime Annotations in Struct Definitions

#[derive(Debug)]
struct ImportantExcerpt<'a> {
    part: &'a str,
}

// Lifetime Annotations in Method Definitions

impl<'a> ImportantExcerpt<'a> {
    // third lifetime elision rule applies:
    fn level(&self) -> i32 {
        3
    }

     // first lifetime elision rule applies:
    // third lifetime elision rule applies:
    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention Please: {}", announcement);
        self.part
    }
}

fn main() {
    let novel = "Rust is cool. Python is cool....".to_string();
    let first_part = novel.split('.')
        .next()
        .expect("Could not found a '.' ");
    let ie = ImportantExcerpt {part: first_part};
    println!("ie: {:?}", ie);
    
    println!("level: {}", ie.level());

    let announcement = "How are you?".to_string();
    ie.announce_and_return_part(&announcement);
}
