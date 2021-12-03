use std::time::Instant;

pub mod input_helpers;

/// Helper function to measure execution time and print to screen
/// The supplied function is intended to accept a bool that specifies whether this iteration of the function should print or not
pub fn measure_execution_time(func: fn(bool), iterations: u32) {
    let start = Instant::now();
    for _x in 0..iterations - 1{
        func(false);
    }
    func(true);
    let elapsed = (Instant::now() - start) / iterations;
    println!("Elapsed time microseconds (over {} iterations): {}",iterations, elapsed.as_micros());
}