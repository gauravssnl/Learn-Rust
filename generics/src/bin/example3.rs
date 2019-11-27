enum Occupation<T> {
    Salary(T),
    Designation,
}

fn main() {
    let student = Occupation::Salary(0);

    let engineer = Occupation::Salary(2000.0);
}