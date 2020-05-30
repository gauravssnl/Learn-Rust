use tester;

mod common;

#[test]
fn it_adds_two() {
    assert_eq!(7, tester::add_two(5));
}

#[test]
fn it_adds_two_after_setup() {
    common::setup();
    assert_eq!(7, tester::add_two(5));
}