use std::fmt::Debug;
use std::{str::FromStr};
use std::fs::File;
use std::any::type_name;
use std::io::Write;
use ureq;

// Prints a pretty header for puzzle
pub fn print_puzzle_header(day_num: i32){
    println!("--------------------\n       Day {}     \n--------------------", day_num);
}

/// Generic function that splits a string into a vector and converts to generic types
pub fn split_string_to_vector<T: FromStr>(input: &str, delimiter: &str) -> Vec<T>
    where
        <T as FromStr>::Err: Debug,
{
    let in_vector: Vec<&str> = input.split(delimiter).collect();
    let mut out_vector: Vec<T> = Vec::with_capacity(in_vector.len());
    for string in in_vector.iter() {
        out_vector.push(string.parse::<T>().unwrap());
    }
    out_vector
}

/// Parses and then writes an importable Rust module that contains the puzzle input formatted as a vector.
pub fn generate_puzzle_input_static<T: FromStr + ToString>(input: &str, delimiter: &str, file_path: &str)
    where
        <T as FromStr>::Err: Debug,
{
    let data : Vec<T> = split_string_to_vector(input, delimiter);
    let mut file = File::create(file_path).unwrap();
    let mut output = String::new();
    output += &*format!("pub static PUZZLE_INPUT_GEN: [{}; {}] = [", type_name::<T>(), data.len());
    let len = &data.len();
    for (idx, val) in data.into_iter().enumerate()
    {
        output += &*val.to_string();
        if idx < len - 1 { output += ","; }
    }
    output += "];";
    file.write_all(&*output.into_bytes()).unwrap();
}

/// Fetch the puzzle input from the Advent site, returns as a string
/// This is currently failing due to not handling OAUTH login credentials
pub fn fetch_puzzle_input_from_advent(year_num: i32, day_num: i32) -> String
{
    let url = format!("http://adventofcode.com/{}/day/{}/input", year_num, day_num);
    match ureq::get(url.as_str()).call(){
        Ok(res) => match res.into_string()
        {
            Ok(body) => return body,
            Err(e) => eprintln!("Failed to parse advent response body to string! Err: {}", e)
        },
        Err(e) => eprintln!("No response received from advent! Err: {}", e)
    }
    let error_response = String::from("");
    error_response
}

