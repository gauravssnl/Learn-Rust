// All the Places Patterns Can Be Used

fn main() {
    // Conditional if let Expressions
    let favorite_color: Option<&str> = None;
    let is_tuesday = false;
    let age: Result<u8, _> = "34".parse();

    if let Some(color) = favorite_color {
        println!("Using your favorite color, {}, as the background", color);
    } else if is_tuesday {
        println!("Tuesday is green day!");
    } else if let Ok(age) = age {
        // if let can also introduce shadowed variables in the same way that match arms can
        if age > 30 {
            println!("Using purple as the background color");
        } else {
            println!("Using orange as the background color");
        }
    } else {
        // handles all others case / default case
        println!("Using blue as the background color");
    }

    // while let Conditional Loops
    let mut stack = Vec::new();

    stack.push(1);
    stack.push(2);
    stack.push(3);

    while let Some(top) = stack.pop() {
        println!("Element at the top of the stack: {}", top);
    }

    // for Loops
    // in 'for x in y' the x is the pattern.
    let v = vec!['a', 'b', 'c', 'd'];

    for (index, value) in v.iter().enumerate() {
        println!("Element at index {} : {}", index, value);
    }

    // let Statements
    let x = 5;
    let (x, y, z) = (1, 2, 3);
    // if we want to ignore some value on RHS, we can use _
    let (a, b, _) = (3, 4, 5);

    let point = (2, 3);
    print_coordinates(&point);
}

// Function Parameters
fn foo(x: i32) {
    println!("x: {}", x);
}

fn print_coordinates(&(x, y): &(i32, i32)) {
    println!("Current Locations: ({}, {})", x, y);
}
