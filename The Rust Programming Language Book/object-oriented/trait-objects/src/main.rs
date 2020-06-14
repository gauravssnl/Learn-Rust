use gui::{Button, Draw, Screen};

// decalre a new component unkown to gui library
struct SelectBox {
    width: u32,
    height: u32,
    options: Vec<String>,
}

// we need to implenet Draw trait to make SelectBox as trait object

impl Draw for SelectBox {
    fn draw(&self) {
        println!("Drawing a SelectBox of width {}  and height {}  whose options are {:?}", self.width, self.height, self.options);
    }
}

fn main() {
    let screen = Screen {
        components: vec![
            Box::new(
                SelectBox {
                    width: 25,
                    height: 10,
                    options: vec![String::from("Python"), String::from("Rust"),],
                }
            ),
            Box::new(
                Button {
                    width: 30,
                    height: 20,
                    label: "Submit".to_string(),
                }
            ),
        ],
    };

    screen.run();
}
