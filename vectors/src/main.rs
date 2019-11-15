fn main() {
    let mut v1: Vec<i32> = Vec::new(); // as Vector is empty, we need to annotate data type
    println!("{:?}", v1);
    v1.push(1);
    v1.push(2);
    v1.push(3);
    println!("{:?}", v1);

    let mut v2 = vec![1, 2, 3, 4, 5];
    println!("{:?}", v2);
    let mut popped_element = v2.pop();
    println!("popped_element: {:?}", popped_element);
    println!("{:?}", v2);

    // Reading elements of vector
    let third: &i32 = &v2[2];
    println!("The third element is: {}", third);

    // preferred way
    match v2.get(2) {
        Some(third) =>  println!("The third element is: {}", third),
        None => println!("Third element not found"),
    }

    // Iterating over the values in vector

    for x in &v2 {
        println!("The element is {}", x);
    }
    println!("{}", "------------------------------------------");
    // using mutable reference to modify element while iterating 

    for x in &mut v2 {
        *x += 1;
        println!("The element is {}", x);
    }

    // Using Enum to store multiple types
    #[derive(Debug)]
    enum SpreadSheet {
        Int(i32),
        Float(f32),
        Text(String),
    }

    let row = vec![
        SpreadSheet::Int(5),
        SpreadSheet::Float(2.6),
        SpreadSheet::Text("Gaurav".to_string()),
    ];
    println!("row vector: {:?}", row);

}

