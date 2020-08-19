
// This function borrows a slice
fn analyze_slice(slice: &[i32]) {
    println!("The first elemenet of the slice: {}", slice[0]);
    println!("The slice has {} elements", slice.len());
}

fn main() {
    // Fixed-size array ( type signature is superflous )
    let arr1: [i32; 6] = [1, 2, 3, 4, 5, 6];
    println!("arr1: {:?}", arr1);

    // All elements can be initialized to the same value
    let arr2: [i32; 100] = [0; 100] ;
    println!("arr1: {:?}", arr1);

    // Indexing starts at 0
    println!("The first element of arr1: {}", arr1[0]);
    println!("The second element of arr1: {}", arr1[1]);

    // `len` returns the size of the array
    println!("arr1 size: {}", arr1.len());

    // Arrays are stack allocated.
    print!("arr1 occupies {} bytes", std::mem::size_of_val(&arr1));

    // Arrays can be automatically borrowed as slices
    println!("Borrow the whole array arr1 as slice");
    analyze_slice(&arr1);

    // Slices can point to an section of an array
    // They are of the form [starting_index..ending index]
    // starting_index is the first position in the slice
    // ending_index is one more than the last position in the slice
    println!("{:?}", &arr2[1..5]);

}