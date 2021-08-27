use std::error::Error;

use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result;

#[derive(Debug)]
pub struct SystemError {
    details: String,
}

impl SystemError {
    pub fn new(details: &str) -> SystemError {
        SystemError {
            details: details.to_string(),
        }
    }
}

impl Display for SystemError {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{}", self.details)
    }
}

impl Error for SystemError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new() -> std::result::Result<(), Box<dyn std::error::Error>> {
        let test_error = SystemError::new("some_test_error");
        assert_eq!(test_error.details.as_str(), "some_test_error");
        assert_eq!(format!("{}", test_error), "some_test_error");
        Ok(())
    }
}
