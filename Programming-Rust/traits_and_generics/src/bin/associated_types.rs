trait MyIterator {
    type Item;
    fn next(&mut self) -> Option<Self::Item>;
}

struct Student {
    name: String,
}

struct StudentList {
    students: Vec<Student>,
    curr: usize,
}

impl MyIterator for StudentList {
    type Item = String;
    fn next(&mut self) -> Option<Self::Item> {
        if self.curr < self.students.len() {
            let res = Some(self.students[self.curr].name.clone());
            self.curr += 1;
            return res;
        }
        None
    }
}

/// Loop over an iterator, storing the values in a new vector.
fn collect_into_vector<I: Iterator>(iter: I) -> Vec<I::Item> {
    let mut results = Vec::new();
    for value in iter {
        results.push(value);
    }
    results
}

/// Print out all the values produced by an iterator
use std::fmt::Debug;

fn dump<I>(iter: I)
where
    I: Iterator,
    I::Item: Debug, //  <I as Iterator>::Item: Debug also works
{
    for (index, value) in iter.enumerate() {
        println!("{}: {:?}", index, value);
    }
}

fn dump1<I>(iter: I)
where
    I: Iterator,
    <I as Iterator>::Item: Debug,
{
    for (index, value) in iter.enumerate() {
        println!("{}: {:?}", index, value);
    }
}

fn dump2<I>(iter: I)
where
    I: Iterator<Item = String>
{
    for (index, value) in iter.enumerate() {
        println!("{}: {:?}", index, value);
    }
}

fn dum32<I>(iter: &mut dyn Iterator<Item=String>)
{
    for (index, value) in iter.enumerate() {
        println!("{}: {:?}", index, value);
    }
}
fn main() {}
