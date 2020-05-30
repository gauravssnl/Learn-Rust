fn main() {
    let mut s = String::from("Hello World");
    let first_word_index = first_word(&s);
    println!("The first word of '{}' ends at index '{}", s, first_word_index);
    s.clear(); // empties the string, but first_word_index contains old value
}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (index, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return index;
        }
    }
    s.len()
}
