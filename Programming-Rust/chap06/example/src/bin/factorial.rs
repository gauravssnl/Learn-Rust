fn factorial(n : i64) -> i64 {
    (1..=n).product()
}

fn main() {
    let x = 0;
    println!("Factorail of {} = {}", x, factorial(x));

    let x = 6;
    println!("Factorail of {} = {}", x, factorial(x));

   for i in 1..0 {
       println!("{}",i);
   }
}