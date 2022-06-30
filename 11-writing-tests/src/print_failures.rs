pub fn prints_and_returns_10(a: i32) -> i32 {
    println!("prints_and_returns_10({})", a);
    10
}

#[cfg(test)]
mod tests {
    use crate::print_failures::prints_and_returns_10;

    #[test]
    fn this_test_will_pass() {
        let value = prints_and_returns_10(4);
        assert_eq!(10, value);
    }

    // #[test]
    // fn this_test_will_fail() {
    //     let value = prints_and_returns_10(8);
    //     assert_eq!(5, value);
    // }

    // cargo test print_failures
    // cargo test print_failures -- --ignored
    // cargo test print_failures -- --include-ignored

    #[test]
    #[ignore]
    fn expensive_test() {
        // code that takes an hour to run
    }
}
