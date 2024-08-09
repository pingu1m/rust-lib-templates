use {{ crate_name }}::preludes::*; // Assuming this imports everything needed for basic usage

fn main() {
    let result = sample_function(5);
    match result {
        Ok(n) => println!("Success: {}", n),
        Err(e) => println!("Error: {:?}", e),
    }
}
