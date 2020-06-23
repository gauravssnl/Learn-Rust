fn main() {
    let mut num = 5;
    // Dereferencing a Raw Pointer

    // here * is not a dereference operator, it's a part of the type name
    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;
    println!("r1: {:?}", r1);
    println!("r2: {:?}", r2);

    let address = 0x012345usize;
    let r = address as *const i32;
    println!("r: {:?}", r);

    unsafe {
        println!("r1 value is : {}", *r1);
        println!("r2 value is : {}", *r2);
        // uncomment the below line to see STATUS_ACCESS_VIOLATION as r does not have valid data
        // println!("r value is : {}", *r);
    }

    // Calling an Unsafe Function or Method
    unsafe {
        dangerous();
    }

    let mut v = vec![1, 2, 3, 4, 5, 6];

    let r = &mut v[..];

    let (a, b) = r.split_at_mut(3);
    assert_eq!(a, &mut [1, 2, 3,]);
    assert_eq!(b, &mut [4, 5, 6,]);

    let (a, b) = split_at_mut(&mut v[..], 3);
    assert_eq!(a, &mut [1, 2, 3,]);
    assert_eq!(b, &mut [4, 5, 6,]);
    println!("a: {:?}", a);
    println!("b: {:?}", a);
}

// Unsafe function
unsafe fn dangerous() {
    println!("call to an unsafe function");
}

// Creating a Safe Abstraction over Unsafe Code

fn split_at_mut(slice: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = slice.len();
    assert!(mid <= len); // assertion that the mid index is within the slice
                         // get mutable raw pointer of the slice - returns *mut i32
    let ptr = slice.as_mut_ptr();

    /* Rust’s borrow checker can’t understand that we’re borrowing different parts of the slice;
    it only knows that we’re borrowing from the same slice twice.  */
    // (&mut slice[..mid], &mut slice[mid..])      // uncomment this line to see the error
    unsafe {
        (
            std::slice::from_raw_parts_mut(ptr, mid), // 1st paramter is a raw pointer and 2nd is length
            std::slice::from_raw_parts_mut(ptr.add(mid), len - mid),
        )
    }
}
