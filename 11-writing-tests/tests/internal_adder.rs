use writing_tests::internal_adder;

mod common;

// run in isolation
// cargo test --test internal_adder

#[test]
fn it_adds_two() {
    common::setup();

    // cannot test it's private
    // assert_eq!(4, internal_adder::internal_adder(2, 2));
    assert_eq!(4, internal_adder::add_two(2));
}
