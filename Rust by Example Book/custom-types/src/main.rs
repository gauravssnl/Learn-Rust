// Struct example

#[derive(Debug)]
struct Person<'a> {
    // The 'a defines lifetime
    name: &'a str,
    age: u8,
}

// A unit struct
struct Unit;

// A tuple struct
struct Pair(f32, f32);

// A struct with 2 fields
#[derive(Debug)]
struct Point {
    x: f32,
    y: f32,
}

// Structs can be resued as fields of another struct
#[derive(Debug)]
struct Rectangle {
    // A rectangle can be specified by where the top left and bottom right
    // corners in space.
    top_left: Point,
    bottom_right: Point,
}

fn main() {
    // Create struct with field init shorthand
    let name = "Peter";
    let age = 27;
    let peter = Person { name, age };
    println!("{:?}", peter);

    // Instantiate a Point
    let point: Point = Point { x: 20.4, y: 30.3 };

    // Acces the fields of the point
    println!("point coordinates: ({}, {})", point.x, point.y);

    // Make a new point using struct update syntax to use the fields of our other one
    let bottom_right = Point { x: 5.5, ..point };

    // bottom_right.y will be same as point.y
    println!("second point: ({}, {})", bottom_right.x, bottom_right.y);

    // Destructure the point using a `let` binding
    let Point {
        x: top_edge,
        y: left_edge,
    } = point;

    let rectangle = Rectangle {
        // struct instanatiation is an experssion  too
        top_left: Point {
            x: top_edge,
            y: left_edge,
        },
        bottom_right: bottom_right,
    };

    println!("rectangle: {:?}", rectangle);

    let _unit = Unit;

    // Instantiate a tuple struct
    let pair = Pair(1.0, 2.3);
    //Acess the fields of a tuple struct
    println!("pair contains {} and {}", pair.0, pair.1);

    // Destructuring a tuple struct
    let Pair(a, b) = pair;

    println!("a and b values are {} and {} respectively", a, b);
}
