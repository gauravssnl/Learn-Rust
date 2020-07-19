// declarative macros with macro_rules!

fn main() {
    #[macro_export]
    macro_rules! vec  {
        ( $($x:expr ),* ) => {
            {
                let mut temp_vec = Vec::new();
                $(
                    temp_vec.push($x);
                ) *
                temp_vec
            }

        };
    }

    let test_vec = vec![1, 2, 3, 4, 5];
    println!("test_vec = {:?}", test_vec);
}
