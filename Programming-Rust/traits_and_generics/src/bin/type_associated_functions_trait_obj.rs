trait StringSet {
    fn new() -> Self
    where
        Self: Sized;

    fn from_slice(string: &[&str]) -> Self
    where
        Self: Sized;

    fn contains(&self, string: &str) -> bool;

    fn add(&mut self, string: &str) -> Self;
}

// Return the set of words in `document` that aren't in `wordlist`.
fn unknown_words<S: StringSet>(document: &[String], wordlist: &S) -> S {
    let mut unknowns = S::new();
    for word in document {
        if !wordlist.contains(word) {
            unknowns.add(word);
        }
    }
    unknowns
}

fn main() {
    // let set1 = SortedStringSet::new();
}
