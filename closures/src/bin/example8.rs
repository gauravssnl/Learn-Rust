use  std::collections::HashMap;



struct Cacher<T> 
    where T: Fn(T) -> T {
    function : T,
    value_map: HashMap<i32, i32>
}

impl<T> Cacher<T> 
    where T: Fn(T) -> T {
    fn new(function: T) -> Cacher<T> {
        let value_map: HashMap<i32, i32>  = HashMap::new();
        Cacher {
            function,
            value_map
        }
    }

    fn value(&mut self, arg: i32) -> i32 {
        self.value_map.entry(arg).or_insert((self.function)(arg));
        self.value_map[&arg];
    } 
}

fn main() {

}