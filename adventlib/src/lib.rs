use std::fmt::Debug;
use std::{fs, str::FromStr};

/// Accepts a given day number as an integer, and returns a string containing the puzzle input for that day.
///
/// The expectation is that the terminal executing this function is located within a 'dayX' folder.
pub fn read_puzzle_input_to_string(day_number: i32) -> String {
    let day_str = format!(
        "{}{}{}",
        "../puzzle_inputs/day",
        day_number.to_string(),
        ".txt"
    );
    let input = fs::read_to_string(day_str).expect(
        "Failed to parse puzzle input. Be sure to run this program from within a DayX folder.",
    );
    input
}

/// Generic function that splits a string into a vector and converts to generic types
pub fn split_string_to_vector<T: FromStr>(input: &str, delimiter: &str) -> Vec<T>
    where
        <T as FromStr>::Err: Debug,
{
    let in_vector: Vec<&str> = input.split(delimiter).collect();
    let mut out_vector: Vec<T> = Vec::new();
    for string in in_vector.iter() {
        out_vector.push(string.parse::<T>().unwrap());
    }
    out_vector
}

// Prints a pretty header for puzzle
pub fn print_puzzle_header(day_num: i32){
    println!("--------------------\n       Day {}     \n--------------------", day_num);
}