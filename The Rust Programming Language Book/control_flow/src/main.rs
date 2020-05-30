use std::io;

fn main() {
    let mut num: String = String::new();
    println!("Enter a number: ");
    io::stdin()
        .read_line(&mut num)
        .expect("Failed to read the input");
    // if expression
    let num: i32 = num.trim().parse().expect("Failed to parse input as number");
    let result = if num % 2 == 0 {
        let res = "Even";
        res
    } else {
        "Odd"
    };
    println!("The number {} is {}", num, result);

    // returning values from loop

    let mut counter = 0;
    let mut sum = 0;
    let result = loop {
        counter += 1;
        if counter > 10 {
            break sum; // return values from loop
        } else {
            sum += counter;
        }
    };
    println!("The result is {}", result);
    let arr = [1, 2, 3, 4, 5];
    for element in arr.iter() {
        println!("The element is {}", element);
    }

    // print number from 1 to 20
    for num in 1..20 { // range of number
        println!("The number is {}", num);
    }

    // print the range in reverse order
     for num in (1..20).rev() { // range of number
        println!("The number is {}", num);
    }

}
