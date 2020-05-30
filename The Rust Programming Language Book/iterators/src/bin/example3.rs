fn main() {
    let v1 = vec![1, 2, 3];

    let mut v1_iter = v1.iter();

    assert_eq!(v1_iter.next(), Some(&1));
    assert_eq!(v1_iter.next(), Some(&2));
    assert_eq!(v1_iter.next(), Some(&3));
    assert_eq!(v1_iter.next(), None);

    //  consuming adaptors
    let  v1_iter = v1.iter();
    let sum: i32 = v1_iter.sum();
    println!("Sum: {}", sum);
    // println!("{:?}", v1_iter); // this line will throw error as sum() takes ownership of iterator

    // itertaor adaptors
    let v1: Vec<i32> = vec![1,2 ,3, 4];
    let v2: Vec<i32> = v1.iter().map(|x| x+1).collect();
    println!("v2: {:?}", v2);
    
}