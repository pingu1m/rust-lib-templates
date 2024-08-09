use your_crate::preludes::*;

fn main() {
    let dividend = 10;
    let divisor = 0; // This should trigger an error

    match risky_division(dividend, divisor) {
        Ok(result) => println!("The result is {}", result),
        Err(e) => println!("Failed to divide: {:?}", e),
    }
}
