// use `pub mod <name>` to define module, defined in <name>.rs
// anything marked `pub` in <name>.rs is exposed under crate::<name>

pub mod adder;
pub mod greeting;
pub mod guess;
pub mod internal_adder;
pub mod print_failures;
pub mod rectangle;

#[cfg(test)]
mod tests {
    // #[test]
    // fn failure() {
    //     panic!("purposeful failure");
    // }

    #[test]
    fn exploration() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}

// works at global scope as well
#[test]
fn global_test() {
    let result = 2 + 2;
    assert_eq!(result, 4);
}
