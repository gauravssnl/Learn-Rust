/*
Given a list of integers, use a vector and return the mean (the average value), median (when sorted, the value in the middle position), and mode (the value that occurs most often; a hash map will be helpful here) of the list.
*/
use std::collections::HashMap;

fn main() {
    let mut arr = [20, 30, 12, 11, 25, 36, 80, 67, 32, 20, 31, 20, 1];
    let mut sum = 0;
    let mut element_map = HashMap::new();

    for x in arr.iter() {
        sum += x;
        let count = element_map.entry(x).or_insert(0);
        *count += 1;
    }
    println!("Average: {}", sum / arr.len());
    println!("{:?}", element_map);
    let max_count_of_element: i32;
    max_count_of_element = match element_map.values().max() {
        Some(x) => *x,
        None => 0,
    };

    println!("max_count_of_element: {:?}", max_count_of_element);
    for (key, val) in element_map.iter() {
        // println!("key: {}, val: {}", key, val);
        if *val == max_count_of_element {
            println!("mode: {}", key);
        }
    }

    arr.sort();
    println!("arr: {:?}", arr);
    let arr_len = arr.len();
    let median = if arr_len % 2 == 0 {
        arr[arr_len / 2]
    } else {
        (arr[arr_len / 2] + arr[arr_len / 2 + 1]) / 2
    };
    println!("median: {}", median)
}
