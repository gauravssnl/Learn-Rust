pub trait Draw {
    fn draw(&self);
}

pub struct Screen {
    // if we use generics we could have stored only 1 type of componets
    // but if we use trait objects, we can store different type of components
    pub components: Vec<Box<dyn Draw>>,  // a vector of trait object
}

impl Screen {
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}

// declare a public Button component

pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String,
}

impl Draw for Button {
    fn draw(&self) {
        println!("Drawing a button of width {} and height {} whose label is '{}'", self.width, self.height, self.label);
    }
   
}