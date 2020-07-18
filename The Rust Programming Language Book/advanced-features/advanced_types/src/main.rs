// Using the Newtype Pattern for Type Safety and Abstraction
struct People(std::collections::HashMap<i64, String>); // we will store Person name & ID, but will expose only names

impl People {
    fn add(&mut self, name: String) {
        let map = &mut self.0;
        // we need to generate ID to store user name
        let id = map.len() as i64 + 1; // get hashamp keys length & add 1 to generate new ID
        map.insert(id, name);
    }
}

impl std::fmt::Display for People {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", &self.0.values())
    }
}
fn main() {
    let mut people = People(std::collections::HashMap::new());
    people.add("GS".to_string());
    people.add("SU".to_string());
    println!("people = {}", people);
}
