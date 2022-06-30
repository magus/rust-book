pub fn greet(name: &str) -> String {
    return format!("Hello {}!", name);
    // return String::from("hello!");
}

#[cfg(test)]
mod tests {
    use crate::greeting::greet;

    #[test]
    fn greet_contains_name() {
        let name = "Carol";

        let result = greet(name);

        assert!(
            result.contains("Carol"),
            "\n\n🚨 [result={}] did not contain [name={}]",
            result,
            name
        );
    }
}
