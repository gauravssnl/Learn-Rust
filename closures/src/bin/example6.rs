struct Cacher<T>
    where T: Fn(u32) -> u32 {
        caclulation: T,
        value: Option<u32>,
}

impl<T> Cacher<T> 
    where T: Fn(u32) -> u32 {
        fn new(caclulation: T) -> Cacher<T> {
            Cacher {
                caclulation,
                value: None
            }
        }
        fn value(&mut self, arg: u32) -> u32 {
            match self.value {
                Some(v) => v,
                None => {
                    let v = (self.caclulation)(arg);
                    self.value = Some(v);
                    v
                }
            }
        }
    }

fn main() {
    
}