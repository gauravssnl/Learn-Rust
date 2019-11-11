#![allow(unused_variables)]

// Patterns that Bind to Values
fn main() {
    #[derive(Debug)]
    enum UsState {
        Albama,
        Alaska,
        // -- snippet
    }
    #[derive(Debug)]
    enum Coin {
        Penny,
        Nickel,
        Dime, 
        Quarter(UsState)
    }

    fn value_in_cents(coin: Coin) -> u8 {
        match coin {
            Coin::Penny => {
                println!("Lucky 1");
                1
            },
            Coin::Nickel => 5,
            Coin::Dime => 10,
            Coin::Quarter(state) => {
                println!("state quarter from {:?}", state);
                25
            },
        }
        
    }

    println!("{}", value_in_cents(Coin::Quarter(UsState::Alaska)));
}
