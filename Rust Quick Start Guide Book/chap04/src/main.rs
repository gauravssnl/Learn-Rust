fn main() {
    let source1 = DemoStruct {
        id: 31,
        name: String::from("Rust"),
        probability: 0.42,
    };

    let DemoStruct {
        id: x,
        name: y,
        probability: z,
    } = source1;
    println!("{}, {}, {}", x, y, z);

    let source1 = DemoStruct {
        id: 32,
        name: String::from("Rust2"),
        probability: 0.42,
    };
    // use directly variable name if they are similar to struct member declaration
    // ignore the rest members
    let DemoStruct { id, name, .. } = source1;
    println!("{}, {}", id, name);
    let source1 = DemoStruct {
        id: 33,
        name: String::from("Rust3"),
        probability: 0.42,
    };
    let DemoStruct { id, name: z, .. } = source1;
    println!("{}, {}", id, z);

    // Using if let expressions to test whether a pattern matches
    let source2 = DemoStruct {
        id: 34,
        name: String::from("Rust2"),
        probability: 0.42,
    };

    if let DemoStruct {
        id: 34,
        name: y,
        probability: z,
    } = source2
    {
        println!("When id is 34, name is {} and probability is {}", y, z);
    } else {
        println!("id does not match");
    }

    let source2 = DemoStruct {
        id: 34,
        name: String::from("Rust2"),
        probability: 0.42,
    };
    if let DemoStruct {
        id: 35, name: y, ..
    } = source2
    {
        println!("When id is 34, name is {} and probability is {}", y, z);
    } else {
        println!("id does not match");
    }

    // we can use multiple if let after else block
    let source2 = DemoStruct {
        id: 34,
        name: String::from("Rust2"),
        probability: 0.42,
    };
    if let DemoStruct {
        id: 35, name: y, ..
    } = source2
    {
        println!("When id is 34, name is {} and probability is {}", y, z);
    } else if let DemoStruct {
        id: 33, name: y, ..
    } = source2
    {
        println!("When id is 33, name is {} and probability is {}", y, z);
    } else {
        println!("id does not match any of the above conditions");
    }

    if let Ok(v) = might_fail(37) {
        println!("Odd succeeded, name is {}", v.name)
    } else {
        println!("Odd failed");
    }

    if let Ok(v) = might_fail(38) {
        println!("Even succeeded, name is {}", v.name)
    } else {
        println!("Even failed");
    }

    // Using match to choose one of several patterns
    // here the same function runs twice and that's inefficient
    if let Ok(v) = might_fail(37) {
        println!("Odd succeeded, name is {}", v.name)
    } else if let Err(x) = might_fail(37) {
        println!("Odd failed, message is '{}'", x);
    }

    let result = might_fail(37);
    if let Ok(v) = result {
        println!("Odd succeeded, name is {}", v.name)
    } else if let Err(x) = result {
        println!("Odd failed, message is '{}'", x);
    }

    // the better approach is to use match
    match might_fail(39) {
        Ok(v) => println!("Odd succeeded, name is {}", v.name),
        Err(e) => println!("Odd failed, message is '{}'", e),
    }

    let n = 23;
    match n {
        1..=30 => println!("Value {} is between 1 and 30(inclusive)", n), // include 30
        _ => println!("Value {} is not between 1 and 30(inclusive)", n),
    }

    // Moving and borrowing in pattern matches
    let source5 = DemoStruct {
        id: 41,
        name: String::from("A Surprising Thing"),
        probability: 0.93,
    };
    //   To tell Rust that we want to borrow a value that is matched by a variable in a pattern,
    //   we need to use a new keyword: ref
    if let DemoStruct {
        id: 41,
        name: ref x,
        probability: _,
    } = source5
    {
        println!("Extracted name: {}", x);
    }

    println!("Source5 name : {}", source5.name);

    /* The ref keyword and the & symbol are closely related.
    The difference between them is syntactic: we apply the ref keyword to the variable where
    the borrow will be stored, while we apply the & symbol to the variable where the value to be
    borrowed is already stored. We choose which one to use based on which end of the borrow
    we're writing, and we don't need both. */
    let ref borrowed1 = source5;
    let borrowed2 = &source5;
    // If you care about object identity/the memory address, you can use the Pointer formatter, {:p}
    println!("{:p}", borrowed1);
    println!("{:p}", borrowed2);
    println!("{:?}", borrowed1);
    println!("{:?}", borrowed2);

    let mut source5 = DemoStruct {
        id: 41,
        name: String::from("A Surprising Thing"),
        probability: 0.93,
    };
    let ref mut borrowed1 = source5;
    println!("{:p}", borrowed1);
    println!("{:?}", borrowed1);
    borrowed1.name = String::from("PyRust");
    println!("{:?}", borrowed1);
    println!("{:p}", borrowed1);

    let mut borrowed2 = &mut source5;
    println!("{:p}", borrowed2);
    println!("{:?}", borrowed2);
    borrowed2.name = String::from("PyRust1");
    println!("{:?}", borrowed2);
    println!("{:p}", borrowed2);

    // the value we're trying to send it is a borrow of a borrow of a DemoStruct, so it
    // dereferences that value once and passes that to the function parameter
    let ref broowed_borrow = &source5;
    borrow_demostruct(broowed_borrow);

    // Nested patterns
    let (_, (_, x, _, _), _) = ((5, 6, 7), (8, 9, 10, 11), (12, 13, 14, 15));
    println!("x is {}", x);

    match might_fail(38) {
        Ok(DemoStruct {
            id: 38,
            ref name,
            probability: _,
        }) => println!("Even succeeded with the proper id: name is {}", name),
        Ok(DemoStruct {
            ref id,
            ref name,
            probability: _,
        }) => println!(
            "Even succeeded with the wrong id: id is {}, name is {}",
            id, name
        ),
        Err(_) => println!("Even failed! Woe is me."),
    }

    let data = (String::from("Rust"), String::from("Python"));
    println!("{:?}", data);
    let (mut x, mut y) = data;
    println!("{}, {}", x, y);
    x = "JS".to_string();
    y = "PY".to_string();
    println!("{}, {}", x, y);
    let mut data = (String::from("Rust"), String::from("Python"));
    let (x, y) = data;
    println!("{}, {}", x, y);
    data.0 = "Hello".to_string();
    data.1 = "World".to_string();
    println!("{:?}", data);
    println!("{}, {}", x, y);
    let mut data = (String::from("Rust"), String::from("Python"));
    println!("{:?}", data);
    // swap the values of tuple
    let (ref mut x, ref mut y) = data;
    println!("{}, {}", x, y);
    *x = String::from("Python");
    *y = String::from("Rust");
    println!("{}, {}", x, y);
    println!("{:?}", data);

    // Storing a matched value and comparing it to a pattern
    if let (1, x @ (_, _), _) = (1, (2, 3), (4, 5, 6)) {
        println!("Matched x to {:?}", x);
    }

    let (x, y) = (1.4, 2);
    println!("{}, {}", x, y);

    // Moving and borrowing in pattern matches
    #[allow(unused_variables)]
    let source5 = DemoStruct {
        id: 41,
        name: String::from("A Surprising Thing"),
        probability: 0.93,
    };

    // warning: floating-point types cannot be used in patterns
    // uncomment the below code to see the warning/error
    // if let DemoStruct {
    //     id: 41,
    //     name: ref x,
    //     probability: 0.93,
    // } = source5
    // {
    //     println!("Extracted name: {}", x);
    // }
    #[allow(unused_variables)]
    let t = 5;
    #[allow(unused_variables)]
    let source6 = DemoStruct {
        id: 7,
        name: String::from("oops"),
        probability: 0.26,
    };
    // irrefutable if-let pattern warning
    // uncomment the code to see the warning
    // if let DemoStruct {id: t, name: _, probability:_}  = source6 {
    //     println!("The pattern matched, x is {}", t);
    // }

    // refutable pattern, but matches when we didn't want it to:
    #[allow(unused_variables)]
    let x = 5;
    let source6 = DemoStruct {
        id: 7,
        name: String::from("oops"),
        probability: 0.26,
    };
    if let DemoStruct {
        id: 7,
        name: x,
        probability: _,
    } = source6
    {
        println!("The pattern matched, x is {}", x);
    }

    // solution : apply match gaurd to the pattern
    // This lets us add non-pattern matching criteria to a match branch.
    let x = 5;
    let source7 = DemoStruct {
        id: 7,
        name: String::from("oops"),
        probability: 0.26,
    };
    // Note : `fn` calls are not allowed in patterns
    match source7 {
        DemoStruct {
            id: y,
            name: _,
            probability: _,
        } if y == x => println!("The pattern with match guard matched, y is {}", y),
        _ => println!("The pattern with match guard did not match"),
    }
}

// Variable assignment with pattern matching
#[derive(Debug)]
pub struct DemoStruct {
    pub id: u64,
    pub name: String,
    pub probability: f64,
}

pub fn might_fail(id: u64) -> Result<DemoStruct, &'static str> {
    if id % 2 == 0 {
        Ok(DemoStruct {
            id,
            name: String::from("An Even ID"),
            probability: 0.5,
        })
    } else {
        Err("Only even IDs are allowed")
    }
}

pub fn borrow_demostruct(x: &DemoStruct) {
    print!("Borrowed id: {}", x.id);
}
