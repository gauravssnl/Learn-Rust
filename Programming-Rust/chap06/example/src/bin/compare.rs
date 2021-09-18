use std::cmp::Ordering;

#[derive(Debug)]
struct Student {
    name: String,
    age: u32,
}

fn main() {
    let mut v = vec![];
    v.push(Student { name: "Gaurav".to_string(), age: 76});
    v.push(Student { name: "GS Su".to_string(), age: 56});
    v.push(Student { name: "Adam".to_string(), age: 30});
    v.push(Student { name: "Akash".to_string(), age: 20});

    fn cmp_by_name_then_age(a: &Student, b: &Student) -> Ordering {
        a.name.cmp(&b.name)
        .reverse()
        .then(a.age.cmp(&b.age))
    }

    v.sort_by(cmp_by_name_then_age);
    println!("{:?}", v);
}