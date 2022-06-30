pub fn add_two(a: i32) -> i32 {
    return a + 2;
}

#[cfg(test)]
mod tests {
    use crate::adder::add_two;

    #[test]
    fn it_adds_two() {
        assert_eq!(add_two(2), 4);
    }

    #[test]
    fn add_two_2_is_not_5() {
        assert_ne!(add_two(2), 5);
    }
}
