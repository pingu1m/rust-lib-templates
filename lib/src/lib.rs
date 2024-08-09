pub mod errors;
pub mod preludes;

use errors::LibErrors;

/// A sample function that might fail, demonstrating error handling.
pub fn sample_function(input: i32) -> Result<i32, LibErrors> {
    if input < 0 {
        Err(LibErrors::Unknown)
    } else {
        Ok(input + 1)
    }
}

// Another function to demonstrate the use of the error type in a practical scenario.
pub fn risky_division(dividend: i32, divisor: i32) -> Result<i32, LibErrors> {
    if divisor == 0 {
        Err(LibErrors::Unknown)  // Ideally, define a more specific error for division by zero
    } else {
        Ok(dividend / divisor)
    }
}

// Example section for documentation, showcasing how to use the library functions
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample_function() {
        assert_eq!(sample_function(10).unwrap(), 11);
        assert!(sample_function(-1).is_err());
    }

    #[test]
    fn test_risky_division() {
        assert_eq!(risky_division(10, 2).unwrap(), 5);
        assert!(risky_division(10, 0).is_err());
    }
}
