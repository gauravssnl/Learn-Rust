fn  main() {
    let some_u8_value = 025u8;
    println!("some_u8_value: {}", some_u8_value);
    match some_u8_value {
        1 => println!("one"),
        3 => println!("three"),
        5 => println!("five"),
        _ => ()
    }

}