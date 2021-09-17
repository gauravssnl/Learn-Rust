fn main() {
    let mut v = Vec::with_capacity(2);
    v.push(1);
    v.push(10);
    println!("{:?}", v);
    v.reverse();
    println!("{:?}", v);
    v.sort();
    println!("{:?}, len = {}", v, v.len()); 
    
    let mut v = Vec::<i32>::new();
    v.push(20);
    v.insert(1, 10);
    println!("{:?}, len = {}", v, v.len()); 
    println!("Popped: {:?}, After: {:?}", v.pop(), v);

    let mut v = vec![10, 20, 30];
    println!("{:?}, len = {}", v, v.len()); 

    let second = v.swap_remove(1);
    println!("{:?}", second);

    let last = v.pop();
    print!("{:?}", last);
    v.push(400);
    v.push(500);
    let third = std::mem::replace(&mut v[2], 500);
    println!("{:?}", third);

    let mut v = vec![10, 20, 30];
    for x in &mut v {
        *x += 200;
    }

    println!("{:?}, len = {}", v, v.len()); 
}