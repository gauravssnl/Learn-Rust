pub struct AveragedCollection {
    list: Vec<i32>,
    average: f64,
}

impl AveragedCollection {
    pub fn new(list: Vec<i32>) -> Self {

        Self {
            list,
            average: 0f64,
        }
    }
    pub fn add(&mut self, value: i32) {
        self.list.push(value);
        self.update_average();
    }

    pub fn remove(&mut self) -> Option<i32> {
        let result = self.list.pop();
        match result {
            Some(value) => {
                self.update_average();
                Some(value)
            },
            None => None,
        }
    }

    pub fn average(&self) -> f64 {
        self.average
    }

    pub fn update_average(&mut self) {
        let total: i32 = self.list.iter().sum();
        self.average = total as f64 / self.list.len() as f64;
    }
}

fn main() {
    let list = vec![];
    let mut ac = AveragedCollection::new(list);
    ac.add(1);
    println!("Current Average: {}", ac.average());
    ac.add(2);
    println!("Current Average: {}", ac.average());
    ac.add(3);
    println!("Current Average: {}", ac.average());
    ac.add(4);
    println!("Current Average: {}", ac.average());
    let removed_item = ac.remove();
    println!("Removed item: {:?}", removed_item);
    println!("Current Average: {}", ac.average());
    let removed_item = ac.remove();
    println!("Removed item: {:?}", removed_item);
    println!("Current Average: {}", ac.average());
    let removed_item = ac.remove();
    println!("Removed item: {:?}", removed_item);
    println!("Current Average: {}", ac.average());
    let removed_item = ac.remove();
    println!("Removed item: {:?}", removed_item);
    println!("Current Average: {}", ac.average());
    let removed_item = ac.remove();
    println!("Removed item: {:?}", removed_item);
    println!("Current Average: {}", ac.average());
}