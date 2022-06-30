use writing_tests::adder;

mod common;

// run in isolation
// cargo test --test adder

#[test]
fn it_adds_two() {
    common::setup();
    assert_eq!(4, adder::add_two(2));
}
