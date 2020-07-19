fn main() {
    // These types use () as initializer syntax, which looks like a function call
    // The initializers are actually implemented as functions returning an instance that’s constructed from their arguments.
    #[derive(Debug)]
    enum Status {
        Value(u32),
        Stop,
    }

    // we can use these intializers functions as function paramters that implement the closure trait
    // thus we can specify the initializer functions as arguments for methods that take closures
    let list_of_statuses: Vec<Status> = (0u32..20).map(Status::Value).collect();
    println!("{:?}", list_of_statuses);
}
