trait Clone: Sized {
    fn clone(&self) -> Self;
    fn clone_from(&mut self, source: &Self) {
        *self = source.clone();
    }
}

#[derive(Debug, Clone)]
struct StudentId {
    id: u128,
    roll_num: u128,
}

#[derive(Debug)]
struct Student {
    name: String,
    age: u32,
    sid: StudentId,
}

impl Clone for Student {
    fn clone(&self) -> Self {
        Student {
            name: self.name.clone(),
            age: self.age.clone(),
            sid: self.sid.clone(),
        }
    }
}
fn main() {
    let mut s1 = Student {
        name: "Gaurav".to_string(),
        age: 25,
        sid: StudentId {
            id: 1,
            roll_num: 100,
        },
    };
    let mut s2 = s1.clone();
    s1.name = "Rust".to_string();
    s1.sid.roll_num = 101;
    println!("s1 now: {:?}", s1);
    println!("s2: {:?}", s2);
    s2.clone_from(&s1);
    println!("s2 now: {:?}", s2);
}
