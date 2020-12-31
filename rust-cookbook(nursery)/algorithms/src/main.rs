use rand::Rng;

fn main() {
    // Generate random numbers
    let mut rng = rand::thread_rng();

    let n1: u8 = rng.gen();
    let n2: u16 = rng.gen();
    println!("Random u8: {}", n1);
    println!("Random u16: {}", n2);
    println!("Random i32: {}", rng.gen::<i32>());
    println!("Random float: {}", rng.gen::<f32>());

    // Generate random numbers within a range
    println!("Integer: {}", rng.gen_range(0, 10)); // will not include 10
    println!("Float: {}", rng.gen_range(0.0, 10.0));
    // Uniform can obtain values with uniform distribution. - maybe faster
    use rand::distributions::{Distribution, Uniform};
    let mut rng = rand::thread_rng();
    let die = Uniform::from(1..7);
    loop {
        let throw = die.sample(&mut rng);
        println!("Roll the die: {}", throw);
        if throw == 6 {
            break;
        }
    }
}
