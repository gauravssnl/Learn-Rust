use serde::Serialize;
use serde_json;
use std::collections::HashMap;

pub fn save_configuration(config: &HashMap<String, String>) -> std::io::Result<()> {
    // Create a JSON serializer to write data to a file
    let writer = std::fs::File::create("test.config")?;
    let mut serializer = serde_json::Serializer::new(writer);
    config.serialize(&mut serializer)?;
    Ok(())
}

fn main() {
    let mut config = HashMap::new();
    config.insert("name".to_string(), "Gaurav".to_string());
    config.insert("lang".to_string(), "Rust".to_string());
    save_configuration(&config).unwrap();
}
