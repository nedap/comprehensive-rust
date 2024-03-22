#![allow(dead_code)]

// ANCHOR: solution


// ANCHOR: divide
fn divide(numerator: f64, denominator: f64) -> Result<f64, DivideError> {
// ANCHOR_END: divide
    if denominator == 0.0 {
        Err(DivideError::DivideByZero)
    } else {
        Ok(numerator / denominator)
    }
}


// ANCHOR: DivideError
enum DivideError {
    // ANCHOR_END: DivideError
    DivideByZero
}

// ANCHOR: main
fn main() {
    let a = 5.0;
    let b = 8.0;

    match divide(a, b) {
// ANCHOR_END: main
        Ok(result) => println!("Result: {:?}", result),
        Err(DivideError::DivideByZero) => println!("Can not divide by zero!!!")
    }
}

// ANCHOR_END: solution
