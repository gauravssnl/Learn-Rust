fn main() {
    // Sort a Vector of Integers
    let mut vec = vec![1, 5, 2, 7, 9, 6, 3];
    vec.sort();
    println!("vec: {:?}", vec);

    // Sort a Vector of Floats
    let mut vec = vec![1.1, 1.15, 5.5, 1.123, 2.0];
    vec.sort_by(|a, b| a.partial_cmp(b).unwrap());
    println!("vec: {:?}", vec);

    // Sort a Vector of Structs
    #[derive(Debug, Ord, Eq, PartialOrd, PartialEq)]
    struct Person {
        name: String,
        age: u32,
    }

    impl Person {
        fn new(name: String, age: u32) -> Self {
            Person { name, age }
        }
    }

    let mut people = vec![
        Person::new("Zoe".to_string(), 25),
        Person::new("Al".to_string(), 60),
        Person::new("John".to_string(), 1),
    ];

    // Sort people by derived natural order (Name and age)
    people.sort();
    println!("people: {:?}", people);

    // Sort people by age in descending order
    people.sort_by(|a, b| b.age.cmp(&a.age));
    println!("people: {:?}", people);
}
