/*
Convert strings to pig latin. The first consonant of each word is moved to the end of the word and “ay” is added, so “first” becomes “irst-fay.” Words that start with a vowel have “hay” added to the end instead (“apple” becomes “apple-hay”). Keep in mind the details about UTF-8 encoding!
*/
fn main() {
    let str1 = String::from("This is a good boy");
    string_to_pig_latin(str1);
}

fn string_to_pig_latin(s: String) -> String {
    println!("{:?}", s.trim().split_whitespace());
    s
}
