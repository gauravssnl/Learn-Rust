fn _dot(v1: &[i64], v2: &[i64]) -> i64 {
    let mut total = 0;
    for i in 0..v1.len() {
        total += v1[i] * v2[i];
    }
    total
}

use std::ops::Add;
use std::ops::AddAssign;
use std::ops::Mul;

fn dot<N: Default + AddAssign + Mul<Output = N> + Copy>(v1: &[N], v2: &[N]) -> N {
    let mut total = N::default();
    for i in 0..v1.len() {
        total += v1[i] * v2[i];
    }
    total
}

fn dot1<N: Default + Add<Output = N> + Mul<Output = N> + Copy>(v1: &[N], v2: &[N]) -> N {
    let mut total = N::default();
    for i in 0..v1.len() {
        total = total + v1[i] * v2[i];
    }
    total
}

use num::Num;

fn dot2<N: Num + Copy>(v1: &[N], v2: &[N]) -> N {
    let mut total = N::zero();
    for i in 0..v1.len() {
        total = total + v1[i] * v2[i];
    }
    total
}

fn main() {
    let v1 = [1, 2, 10, 16, 17];
    let v2 = [3, 6, 8, 9, 20];
    println!("{}", dot(&v1, &v2));

    let v1 = [1, 2, 10, 16, 17];
    let v2 = [3, 6, 8, 9, 20];
    println!("{}", dot1(&v1, &v2));

    let v1 = [1, 2, 10, 16, 17];
    let v2 = [3, 6, 8, 9, 20];
    println!("{}", dot2(&v1, &v2));
}
