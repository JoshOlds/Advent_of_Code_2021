use crate::input::{PUZZLE_INPUT};

mod input;

#[derive(Default)]
struct KnownDigits<'a>
{
    known_1 : &'a str,
    known_4 : &'a str,
    known_7 : &'a str,
    known_8 : &'a str,
}

fn main() {
    //if GENERATE_PUZZLE_INPUT_VECTOR{ adventlib::input_helpers::generate_puzzle_input_static::<i32>(PUZZLE_INPUT, ",", "day7/src/input_gen"); }
    adventlib::input_helpers::print_puzzle_header(8);
    adventlib::measure_execution_time_us(run, 10000);
}

fn run(do_print: bool) {

    // Lots of parsing of input
    let input_lines = PUZZLE_INPUT.split("\n");
    let mut signal_patterns: [[&str; 10]; 200] = [[""; 10]; 200];
    let mut digit_outputs: [[&str; 4]; 200] = [[""; 4]; 200];
    for (row_idx, line) in input_lines.enumerate()
    {
        let mut signal_chars = line.split(" ");
        // Unrolled loop for performance
        signal_patterns[row_idx][0] = signal_chars.next().unwrap();
        signal_patterns[row_idx][1] = signal_chars.next().unwrap();
        signal_patterns[row_idx][2] = signal_chars.next().unwrap();
        signal_patterns[row_idx][3] = signal_chars.next().unwrap();
        signal_patterns[row_idx][4] = signal_chars.next().unwrap();
        signal_patterns[row_idx][5] = signal_chars.next().unwrap();
        signal_patterns[row_idx][6] = signal_chars.next().unwrap();
        signal_patterns[row_idx][7] = signal_chars.next().unwrap();
        signal_patterns[row_idx][8] = signal_chars.next().unwrap();
        signal_patterns[row_idx][9] = signal_chars.next().unwrap();
        signal_chars.next();
        digit_outputs[row_idx][0] = signal_chars.next().unwrap();
        digit_outputs[row_idx][1] = signal_chars.next().unwrap();
        digit_outputs[row_idx][2] = signal_chars.next().unwrap();
        digit_outputs[row_idx][3] = signal_chars.next().unwrap();
    }

    // Count unique digits in outputs
    let mut count = 0;
    for digit in digit_outputs.iter_mut().flat_map(|r| r.iter_mut())
    {
        let len = digit.len();
        if len == 2 || len == 4 || len == 3 || len == 7 { count += 1; }
    }
    if do_print{println!("Part 1: {}", count); }

    // Part 2
    let mut sum = 0;
    let mut known_digits = KnownDigits::default();
    let mut signal_iter = signal_patterns.into_iter();
    let mut digit_iter = digit_outputs.into_iter();
    for _idx in 0..signal_iter.len()
    {
        let signals = signal_iter.next().unwrap();
        let digits = digit_iter.next().unwrap();
        // Decode the known digits
        for signal in signals
        {
            decode_known_digit(signal, &mut known_digits);
        }
        // Decode the 4-dig output based on the known digits
        for (idx, digit) in digits.into_iter().enumerate()
        {
            match idx{
                0 => sum += decode_digit(digit, &known_digits) * 1000,
                1 => sum += decode_digit(digit, &known_digits) * 100,
                2 => sum += decode_digit(digit, &known_digits) * 10,
                3 => sum += decode_digit(digit, &known_digits),
                _ => {}
            }
        }
    }

    if do_print{println!("Part 2: {}", sum); }

}

/// Returns the count of how many characters in the operator exist in the operand
fn contains_count(operand: &str, operator: &str) -> i32
{
    let mut count = 0;
    for char in operator.chars()
    {
        if operand.contains(char) { count += 1;}
    }
    count
}

/// Decodes known digits based on input string's length and stores in the supplied KnownDigit struct
fn decode_known_digit<'a>(digit: &'a str, known_digit_storage: &mut KnownDigits<'a>)
{
    match digit.len() {
        2 => known_digit_storage.known_1 = digit,
        4 => known_digit_storage.known_4 = digit,
        3 => known_digit_storage.known_7 = digit,
        7 => known_digit_storage.known_8 = digit,
        _ => {}
    }
}

/// Decodes any digit string using a pre-populated KnownDigit struct
fn decode_digit(digit: &str, known_digits: &KnownDigits) -> i32
{
    return match digit.len() {
        2 => 1,
        4 => 4,
        3 => 7,
        7 => 8,
        5 => decode_5digit(digit, known_digits),
        6 => decode_6digit(digit, known_digits),
        _ => { panic!("Decode digits found and invalid number of segments!") }
    }
}

/// Decodes a 5-character digit by checking how many segments of a known digit it contains
fn decode_5digit(operand: &str, known_digits: &KnownDigits) -> i32
{
    if contains_count(operand, known_digits.known_1) == 2 { return 3; }
    let match_4 = contains_count(operand, known_digits.known_4);
    if match_4 == 3 { return 5; }
    if match_4 == 2 { return 2; }
    return 3;
}

/// Decodes a 6-character digit by checking how many segments of a known digit it contains
fn decode_6digit(operand: &str, known_digits: &KnownDigits) -> i32
{
    if contains_count(operand, known_digits.known_4) == 4 { return 9; }
    if contains_count(operand, known_digits.known_7) == 3 { return 0; }
    return 6;
}

