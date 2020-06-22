// Pattern Syntax

fn main() {
    let x = Some(5);
    let y = 10;

    match x {
        Some(50) => println!("Got 50"),
        Some(y) => println!("Matched y = {:?}", y), // y in arm shadows outer y here
        _ => println!("Default case, x = {:?}", x), // here x refers to the x defined in outer scope
    }
    println!("At the end: x = {:?}, y = {:?}", x, y);

    // Multiple Patterns
    let x = 1;

    match x {
        1 | 2 => println!("one or two"),
        3 => println!("three"),
        _ => println!("anything"),
    }

    // Matching Ranges of Values with ..=
    let x = 5;

    match x {
        1..=5 => println!("one through five"),
        _ => println!("Something else"),
    }

    // Ranges are only allowed with numeric values or char values

    let x = 'c';

    match x {
        'a'..='j' => println!("Char between a and j(including j)"),
        'k'..='z' => println!("Char between k and z(including z)"),
        _ => println!("Something else"),
    }

    // Destructuring to Break Apart Values

    // Destructuring Structs
    struct Point {
        x: i32,
        y: i32,
    }

    let p = Point { x: 1, y: 2 };
    let Point { x: a, y: b } = p;
    // Destructuring a struct’s fields into separate variables
    println!("a: {} , b: {}", a, b);

    // Destructuring struct fields using struct field shorthand
    let Point { x, y } = p;
    println!("x: {} , y: {}", x, y);

    // Destructuring and matching literal values in one pattern
    let p = Point { x: 0, y: 5 };
    match p {
        Point { x, y: 0 } => println!("Point lies on the x-axis at {}", x),
        Point { x: 0, y } => println!("Point lies on the y-axis at {}", y),
        Point { x, y } => println!("Neither on x-axis nor on y-axis: ({}, {})", x, y),
    }

    // Destructuring Enums

    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(i32, i32, i32), // (red, green, blue)
    }

    let msg = Message::ChangeColor(220, 130, 160);
    match msg {
        Message::Quit => println!("The quit variant has no data to destructure."),
        Message::Move { x, y } => {
            println!("Move in the x direction {} and in the y direction {}", x, y);
        }
        Message::Write(text) => println!("Text message: {}", text),
        Message::ChangeColor(red, green, blue) => println!(
            "Change the color to the red {}, green {}, and blue {}",
            red, green, blue
        ),
    }

    // Destructuring Nested Structs and Enums
    enum Color {
        Rgb(i32, i32, i32),
        Hsv(i32, i32, i32),
    }

    enum NewMessage {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(Color),
    }

    let msg = NewMessage::ChangeColor(Color::Hsv(0, 150, 200));
    match msg {
        NewMessage::ChangeColor(Color::Rgb(red, green, blue)) => println!(
            "Change the color to the red {}, green {}, and blue {}",
            red, green, blue
        ),
        NewMessage::ChangeColor(Color::Hsv(hue, saturation, value)) => println!(
            "Change the color to the hue {}, saturation {}, and value {}",
            hue, saturation, value
        ),
        _ => (), // do nothing
    }

    // Destructuring Structs and Tuples
    let ((feet, inches), Point { x, y }) = ((3, 10), Point { x: 10, y: 20 });
    println!("feet: {}, inches: {}, x: {} and y: {}", feet, inches, x, y);

    // Ignoring Values in a Pattern

    // Ignoring an Entire Value with _
    fn foo(_: i32, y: i32) {
        println!("This function only uses y paramter: {}", y);
    }

    foo(3, 4);

    // Ignoring Parts of a Value with a Nested _
    let mut setting_value = Some(5);
    let new_setting_value = Some(10);

    match (setting_value, new_setting_value) {
        (Some(_), Some(_)) => println!("Can't overwrite an existing customized value"),
        _ => {
            setting_value = new_setting_value;
        }
    }

    // Ignoring multiple parts of a tuple
    let numbers = (2, 4, 6, 8, 10, 12);

    match numbers {
        (first, _, third, _, fifth, _) => {
            println!("Some numbers: {}, {}, {}", first, third, fifth);
        }
    }

    // Ignoring an Unused Variable by Starting Its Name with _
    let _x = 5;
    let y = 6; // compiler will warn about this unused variable

    // An unused variable starting with an underscore still binds the value, which might take ownership of the value
    let s = Some(String::from("Hello!"));

    if let Some(_) = s {
        // if we use Some(_s) we will get error as the vaue of s will be moved
        println!("found a string");
    }

    println!("{:?}", s);

    // Ignoring Remaining Parts of a Value with ..
    struct Point1 {
        x: i32,
        y: i32,
        z: i32,
    }

    let origin = Point1 { x: 0, y: 0, z: 0 };

    match origin {
        Point1 { x, .. } => println!("x is {}", x),
    }

    // Matching only the first and last values in a tuple and ignoring all other values
    let numbers = (10, 20, 30, 40, 50, 60);

    match numbers {
        (first, .., last) => println!(
            "First and last numbers are {} and {} respectively",
            first, last
        ),
    }

    // An attempt to use .. in an ambiguous way
    // the following code won't
    /*   match numbers {
        (.., second, ..) => println!("Second number is {}", second),
    } */

    // Extra Conditionals with Match Guards ( An additional if condition after the pattern)
    let num = Some(4);

    match num {
        Some(x) if x < 5 => println!("x = {} is less than 5", x),
        Some(x) => println!("x = {} is greater than 5", x),
        None => (),
    }

    // Using a match guard to test for equality with an outer variable
    let x = Some(10);
    let y = 10;

    match x {
        Some(50) => println!("Got 50"),
        Some(n) if n == y => println!("The value of n: {} matches with y: {}", n, y),
        _ => println!("Default case: x = {:?}", x),
    }

    println!("at the end: x = {:?} , y = {}", x, y);

    // Combining multiple patterns with a match guard
    let x = 5;
    let y = false;

    match x {
        4 | 5 | 6 if y => println!("yes"), // works like (4 | 5 | 6) if y
        _ => println!("no"),
    }

    // @ Bindings
    //The at operator (@) lets us create a variable that holds a value at the same time we’re testing that value to see
    // whether it matches a pattern

    enum Message1 {
        Hello { id: i32 },
    }

    let message = Message1::Hello { id: 5 };

    match message {
        // we can also name variable id_variable as id also
        Message1::Hello {
            id: id_variable @ 3..=7,
        } => println!("Found an id in range: {}", id_variable),
        Message1::Hello { id: 10..=12 } => println!("Found an id in another range"),
        Message1::Hello { id } => println!("Found some another id: {}", id),
    }
}
