pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 {
            panic!("Guess value must be positive, got [{}].", value);
        } else if value > 100 {
            panic!("Guess value must be between 1-100, got [{}].", value);
        }

        Guess { value }
    }

    pub fn is(&self, value: i32) -> bool {
        return self.value == value;
    }
}

#[cfg(test)]
mod tests {
    use crate::guess::Guess;

    // tests can return Result<Ok, Err> too
    // this means we can use ? to catch and bubble unexpected errors
    // it also means we can perform other logic as long as
    // we return Ok() or Err()
    #[test]
    fn it_works() -> Result<(), String> {
        if 2 + 2 == 4 {
            Ok(())
        } else {
            Err(String::from("two plus two does not equal four"))
        }
    }

    #[test]
    // #[should_panic(expected = "Guess value must be between 1 and 100, got 200.")]
    // substring match on panic string
    #[should_panic(expected = "Guess value must be between 1-100")]
    fn greater_than_100() {
        Guess::new(200);
    }

    #[test]
    #[should_panic(expected = "Guess value must be positive")]
    fn zero() {
        Guess::new(0);
    }

    #[test]
    #[should_panic(expected = "Guess value must be positive")]
    fn negative_1() {
        Guess::new(-1);
    }

    #[test]
    fn is_tests_value() {
        let guess = Guess::new(43);
        assert_eq!(guess.is(17), false);
        assert_eq!(guess.is(43), true);
    }
}
