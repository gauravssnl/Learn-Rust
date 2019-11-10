fn main() {
    let user1 = User {
        name: String::from("Gaurav"),
        email: String::from("gaurav.ssnl@gmail.com"),
        sign_in_count: 1,
        active: true,
    };
    println!("user1 name: {} ", user1.name);
    // entire instance must be mutable to change even one field
    // Struct update syntax
    let mut user2 = User {
        name: String::from("Gaurav"),
        email: String::from("gaurav.ssnl@gmail.com"),
        ..user1 // take remaining data from user1
    };
    user2.email = String::from("g@g.com");
    println!("user2 name: {} ", user2.name);
    println!("user2 email: {} ", user2.email);

    let user3 = build_user(String::from("gk@g.com"), String::from("GKSU"));
    println!("user3 name: {} ", user3.name);
    println!("user3 email: {} ", user3.email);

    let point1 = Point(1, 2, 3);
    let color1 = Color(100, 200, 100);
}

struct User {
    name: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn build_user(email: String, name: String) -> User {
    // using the Field Init Shorthand
    User {
        email,
        name,
        sign_in_count: 1,
        active: true,
    }
}

// Using Tuple Structs without Named Fields to Create Different Types
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

// Unit-Like Structs Without Any Fields
// useful in situations in which you need to implement a trait
// on some type but don’t have any data
struct EmptyStruct {}
