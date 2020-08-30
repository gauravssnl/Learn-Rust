// create an enum to classify a web event.

enum WebEvent {
    // An `enum` may be either unit-like
    PageLoad,
    PageUnload,
    // like tuple structs
    KeyPress(char),
    Paste(String),
    // or C-like structures
    Click { x: i64, y: i64 },
}

// A function that takes a `WebEvent` enum as an argument and return nothing
fn inspect(event: WebEvent) {
    match event {
        WebEvent::PageLoad => println!("page loaded"),
        WebEvent::PageUnload => println!("page unloaded"),
        // Destructure `c` from inside the enum
        WebEvent::KeyPress(c) => println!("pressed '{}'", c),
        WebEvent::Paste(s) => println!("pasted \"{}\"", s),
        // Destructure `Click` into `x` and `y`
        WebEvent::Click { x, y } => println!("clicked at x={}, y={}.", x, y),
    }
}

fn main() {
    let pressed = WebEvent::KeyPress('x');
    // `to_owned()` creates an owned `String` from a string slice
    let pasted = WebEvent::Paste("my text".to_owned());
    let click = WebEvent::Click { x: 20, y: 40 };
    let load = WebEvent::PageLoad;
    let unload = WebEvent::PageUnload;

    inspect(pressed);
    inspect(pasted);
    inspect(click);
    inspect(load);
    inspect(unload);

    // Type aliases
    enum VeryVerboseEnumOfThingsToDoWithNumbers {
        Add,
        Substract,
    }

    // create a type alias
    type Operations = VeryVerboseEnumOfThingsToDoWithNumbers;

    let _add = Operations::Add;
    let _substract = Operations::Substract;
}
