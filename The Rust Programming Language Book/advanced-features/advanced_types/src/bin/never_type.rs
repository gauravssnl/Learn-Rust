#[allow(unused)]

// The Never Type that Never Returns ( ! type)
// Functions that return never are called diverging functions
// panic! and loop has the type !

fn bar() -> ! {
   panic!("Panic here");
}

fn main() {
    // bar();

    loop{
        let mut guess = String::new();
        std::io::stdin().read_line(&mut guess).unwrap();
        let guess = match guess.trim().parse::<u32>() { // turbofish syntax
            Ok(num) => num,
            Err(_) => continue,     // never type ( '!' )
        } ;

        if guess == 200 {
            print!("You win");
            break;
        }
    }
    
}