use std::thread;
use std::time::Duration;

fn simulate_expensive_calculation(intensity: u32) -> u32 {
    println!("Calculating slowly...");
    thread::sleep(Duration::from_secs(2));
    intensity
}

fn main() {
    let simulated_user_specified_value = 10;
    let simulated_random_number = 7;

    generate_workout(simulated_user_specified_value, simulated_random_number);
}

fn generate_workout(intensity: u32, random_number: u32) {
    let mut expensive_result = Cacher::new(|num| {
        println!("Calculating slowly...");
        thread::sleep(Duration::from_secs(3));
        num
    });

   

    if intensity < 25 {
        println!("Today do {} pushups!", expensive_result.value(intensity));

        println!("Today do {} situps!", expensive_result.value(intensity));
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!("Today run for {} minutes!", expensive_result.value (intensity));
        }
    }
}

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


// Limitations of the Cacher Implementation
#[test]
fn call_with_different_values() {
    let mut c = Cacher::new(|a| a);

    let v1 = c.value(1);
    let v2 = c.value(2);

    assert_eq!(v2, 2);
}