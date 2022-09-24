use std::error::Error;
use std::fs;

use crate::config::Config;
use crate::errors::QuietError;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let file_content = fs::read_to_string(config.file_path)?;

    let mut no_match = true;

    let result_lines = if config.ignore_case {
        search_case_insensitive(&config.query, &file_content)
    } else {
        search(&config.query, &file_content)
    };

    for result in result_lines {
        no_match = false;

        println!("{result}");
    }

    if no_match {
        return Err(QuietError {}.into());
    }

    Ok(())
}

pub fn search<'a>(query: &str, file_content: &'a str) -> Vec<&'a str> {
    let mut result = vec![];

    for line in file_content.lines() {
        if line.contains(query) {
            result.push(line)
        }
    }

    return result;
}

pub fn search_case_insensitive<'a>(query: &str, file_content: &'a str) -> Vec<&'a str> {
    let mut result = vec![];

    let query_lowercase = &query.to_lowercase();

    for line in file_content.lines() {
        if line.to_lowercase().contains(query_lowercase) {
            result.push(line)
        }
    }

    return result;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn no_file() {
        let config = Config {
            query: String::from(""),
            file_path: String::from("./does-not-exist.txt"),
            ignore_case: false,
        };

        let err = run(config).unwrap_err();
        assert_eq!("No such file or directory (os error 2)", err.to_string());
    }

    #[test]
    fn no_match() {
        let config = Config {
            query: String::from("never-find-me-haha"),
            file_path: String::from("./Cargo.toml"),
            ignore_case: false,
        };

        let err = run(config).unwrap_err();
        assert_eq!("[QuietError]", err.to_string());
    }

    #[test]
    fn one_result() {
        let query = "duct";
        let file_content = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(vec!["safe, fast, productive."], search(query, file_content));
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let file_content = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, file_content)
        );
    }
}
