use std::error::Error;
use std::process;

use minigrep::config::Config;

fn main() -> Result<(), Box<dyn Error>> {
    // we use unwrap here to get config if there is no error
    let config = Config::build().unwrap_or_else(|err| {
        eprintln!("minigrep unable to parse arguemnts");
        eprintln!("  > {err}");
        process::exit(1);
    });

    // here we do not need the return just the error
    // so we pattern match on the run result
    match minigrep::run(config) {
        // do nothing, successful run
        Ok(_) => {}

        // handle errors
        Err(error) => {
            if error.is::<minigrep::errors::QuietError>() {
                // do nothing, it's a quiet error!
                process::exit(2);
            }

            eprintln!("minigrep encountered an error");
            eprintln!("  > {error}");
            process::exit(2);
        }
    }

    return Ok(());
}
