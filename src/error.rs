use core::fmt;
use std::error::Error;

pub type CliResult<T> = Result<T, Box<std::error::Error>>;

#[derive(Debug, Clone)]
pub enum CliError {
    ArgumentError(String),
}

impl fmt::Display for CliError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.clone() {
            CliError::ArgumentError(why) => write!(f, "ArgumentError: {}", why),
        }
    }
}

impl Error for CliError {
    fn description(&self) -> &str {
        "Cli Error."
    }
}
