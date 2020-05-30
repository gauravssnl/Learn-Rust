// Lifetime Annotations in Struct Definitions

#[derive(Debug)]
struct ImportantExcerpt<'a> {
    part: &'a str,
}

fn main() {
    let novel = "Rust is cool. Python is cool....".to_string();
    let first_part = novel.split('.')
        .next()
        .expect("Could not found a '.' ");
    let ie = ImportantExcerpt {part: first_part};
    println!("ie: {:?}", ie);

}
