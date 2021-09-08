//! # GCD
//! 
//! A library for computing Greatest Common Divisor (GCD).
//!
/// Computes Greatest Common Divisor (GCD) of unsigned m & n.
/// 
/// # Example 
/// 
/// use gcd;
///
/// fn main() {
/// 
///     let res =  gcd::compute(30, 10);
/// 
///     assert_eq!(res, 10);
/// 
/// }
pub fn compute(mut n: u64, mut m: u64) -> u64 {
    assert!(n !=0 && m != 0);
    while m != 0 {
        if n > m {
            let temp = m;
            m = n;
            n = temp;
        }
        m = m % n;
    }
    n
}