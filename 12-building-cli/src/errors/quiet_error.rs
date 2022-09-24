use std::error::Error;

#[derive(Debug)]
pub struct QuietError {}

impl std::fmt::Display for QuietError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return write!(f, "[QuietError]");
    }
}

impl Error for QuietError {}
