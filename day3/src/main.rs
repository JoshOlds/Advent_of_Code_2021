use adventlib;
use crate::input::PUZZLE_INPUT;

mod input;

fn main() {
    adventlib::input_helpers::print_puzzle_header(3);
    adventlib::measure_execution_time_us(run, 10000);
}

fn run(do_print: bool) {
    let input_vec: Vec<&str> = PUZZLE_INPUT.split("\n").collect();

    // Part 1
    let cutoff = input_vec.len() as i32;
    let half_size = cutoff / 2;

    let (mut bit0, mut bit1, mut bit2, mut bit3, mut bit4, mut bit5,
        mut bit6, mut bit7, mut bit8, mut bit9, mut bit10, mut bit11)
        = (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0);

    // Naive checking of each bit and counting
    for line in &input_vec {
        if line.chars().nth(0).unwrap() == '1' { bit0 += 1; }
        if line.chars().nth(1).unwrap() == '1' { bit1 += 1; }
        if line.chars().nth(3).unwrap() == '1' { bit3 += 1; }
        if line.chars().nth(2).unwrap() == '1' { bit2 += 1; }
        if line.chars().nth(4).unwrap() == '1' { bit4 += 1; }
        if line.chars().nth(5).unwrap() == '1' { bit5 += 1; }
        if line.chars().nth(6).unwrap() == '1' { bit6 += 1; }
        if line.chars().nth(7).unwrap() == '1' { bit7 += 1; }
        if line.chars().nth(8).unwrap() == '1' { bit8 += 1; }
        if line.chars().nth(9).unwrap() == '1' { bit9 += 1; }
        if line.chars().nth(10).unwrap() == '1' { bit10 += 1; }
        if line.chars().nth(11).unwrap() == '1' { bit11 += 1; }
    }

    // If the '1' count is more than half, add the corresponding value to gamma
    let mut gamma_rate: u32 = 0;
    if bit0 > half_size { gamma_rate += 2048; }
    if bit1 > half_size { gamma_rate += 1024; }
    if bit2 > half_size { gamma_rate += 512; }
    if bit3 > half_size { gamma_rate += 256; }
    if bit4 > half_size { gamma_rate += 128; }
    if bit5 > half_size { gamma_rate += 64; }
    if bit6 > half_size { gamma_rate += 32; }
    if bit7 > half_size { gamma_rate += 16; }
    if bit8 > half_size { gamma_rate += 8; }
    if bit9 > half_size { gamma_rate += 4; }
    if bit10 > half_size { gamma_rate += 2; }
    if bit11 > half_size { gamma_rate += 1; }

    // Epsilon is the bitwise not of gamma, and need to truncate the extra bits
    let epsilon_rate = !gamma_rate & 4095;

    if do_print{ println!("Part 1: Gamma: {}, Epsilon: {}, Result: {}", gamma_rate, epsilon_rate, epsilon_rate * gamma_rate) };

    // Part 2
    let mut filtered_vec = input_vec.clone();

    let mut iteration = 0;
    while filtered_vec.len() > 1 {
        if find_most_common_bit(&filtered_vec, iteration)
        {
            filtered_vec = filtered_vec.into_iter().filter(|line|line.chars().nth(iteration).unwrap() == '1').collect();
        }
        else
        {
            filtered_vec = filtered_vec.into_iter().filter(|line|line.chars().nth(iteration).unwrap() == '0').collect();
        }
        iteration += 1;
    }
    let oxygen_rating: u32 = u32::from_str_radix(filtered_vec[0], 2).unwrap();

    filtered_vec = input_vec.clone();
    iteration = 0;
    while filtered_vec.len() > 1 {
        if !find_most_common_bit(&filtered_vec, iteration)
        {
            filtered_vec = filtered_vec.into_iter().filter(|line|line.chars().nth(iteration).unwrap() == '1').collect();

        }
        else
        {
            filtered_vec = filtered_vec.into_iter().filter(|line|line.chars().nth(iteration).unwrap() == '0').collect();
        }
        iteration += 1;
    }
    let co2_rating: u32 = u32::from_str_radix(filtered_vec[0], 2).unwrap();
    if do_print{ println!("Part 2: Oxygen: {}, CO2: {}, Result: {}", oxygen_rating, co2_rating, oxygen_rating * co2_rating) };
}

/// Returns true when most common bit at address is 1, false if 0
fn find_most_common_bit(input_vec: &Vec<&str>, bit_address: usize) -> bool
{
    let mut bit_count = 0;
    for line in input_vec
    {
        if line.chars().nth(bit_address).unwrap() == '1' { bit_count += 1; }
    }
    bit_count >= input_vec.len() - bit_count
}
