use std::env;

pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}

impl Config {
    pub fn build<'a>() -> Result<Config, &'a str> {
        let args: Vec<String> = env::args().collect();

        if args.len() < 2 {
            return Err("search string is required");
        }

        if args.len() < 3 {
            return Err("file path is required");
        }

        let query = args[1].clone();
        let file_path = args[2].clone();

        let ignore_case = env::var("IGNORE_CASE").is_ok();

        return Ok(Self {
            query,
            file_path,
            ignore_case,
        });
    }
}
