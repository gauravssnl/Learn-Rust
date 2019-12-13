use std::collections::HashMap;

fn main() {
    let mut scores: HashMap<String, i32>  = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 20);
    println!("scores: {:?}", scores);

    // get value 
    println!("{:?}", scores.get("Blue"));
    // iterating over key & value
    for (key, val) in scores.iter() {
        println!("Key: {}, Value: {}", key, val);
    }

    // overwriting key
    scores.insert("Blue".to_string(), 30);
    println!("scores: {:?}", scores);

    // Only Inserting a Value If the Key Has No Value
    scores.entry(String::from("Yellow")).or_insert(20); // returns Entry enum
    println!("scores: {:?}", scores);
    scores.entry(String::from("Red")).or_insert(20);
    println!("scores: {:?}", scores);

    // Updating a value based on old value
    let text = String::from("My name is Gaurav");
    let count = 0;
    let mut word_map = HashMap::new();
    for word in text.split_whitespace() {
        println!("word: {}", word);
        let count = word_map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("{:?}", word_map);

    // Creating a hash map from a list of teams and a list of scores
    let teams = vec!["Red", "Green"];
    let scores = vec![30, 40];
    println!("{:?}", teams.iter());
    println!("{:?}", teams.iter().zip(scores.iter()));
    let team_scores: HashMap<_, _> = teams.iter().zip(scores.iter()).collect();
    println!("team_scores: {:?}", team_scores);
}
