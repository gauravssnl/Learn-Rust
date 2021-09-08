use gcd::*;

#[test]
fn test_gcd() {
    assert_eq!(compute(14, 15), 1);
    assert_eq!(compute(15, 14), 1);
}