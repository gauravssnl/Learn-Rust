#![allow(unused_variables)]
fn main() {
    enum Coin {
        Penny,
        Nickel,
        Dime, 
        Quarter
    }

    fn value_in_cents(coin: Coin) -> u8 {
        match coin {
            Coin::Penny => {
                println!("Lucky 1");
                1
            },
            Coin::Nickel => 5,
            Coin::Dime => 10,
            Coin::Quarter => 25
        }
    }
}
