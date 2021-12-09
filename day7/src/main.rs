use std::fs::File;
use std::io::Write;
use crate::input::{PUZZLE_INPUT};
use crate::input_gen::PUZZLE_INPUT_GEN;
use crate::triangle_nums::TRIANGLE_NUMS; //, TEST_INPUT};

mod input;
mod input_gen;
mod triangle_nums;

// Pre-Generate Triangle numbers file
static GENERATE_TRIANGLE_NUMS: bool = false;
static GENERATE_PUZZLE_INPUT_VECTOR: bool = false;

fn main() {
    if GENERATE_TRIANGLE_NUMS{ write_triangle_nums_to_file() };
    if GENERATE_PUZZLE_INPUT_VECTOR{ adventlib::input_helpers::generate_puzzle_input_static::<i32>(PUZZLE_INPUT, ",", "day7/src/input_gen.rs"); }
    adventlib::input_helpers::print_puzzle_header(7);
    adventlib::measure_execution_time_us(run, 10000);
}

fn run(do_print: bool) {
    // Use pre-generated array of puzzle input
    let (best_pos, shortest_dist) = solve_min_dist_position_part1(&PUZZLE_INPUT_GEN);
    if do_print{println!("Part 1: Best Position: {}, Fuel Spent: {}", best_pos, shortest_dist);}

    let (best_pos, shortest_dist) = solve_min_dist_position_part2(&PUZZLE_INPUT_GEN);
    if do_print{println!("Part 2: Best Position: {}, Fuel Spent: {}", best_pos, shortest_dist);}
}

fn solve_min_dist_position_part1(crab_positions: &[i32; 1000]) -> (i32, i32)
{
    let mut best_pos: i32 = 0;
    let mut shortest_dist = i32::MAX;
    let mut current_dist;

    for pos in crab_positions
    {
        current_dist = 0;
        for val in crab_positions
        {
            current_dist += (*val - *pos).abs();
        }
        if current_dist < shortest_dist
        {
            shortest_dist = current_dist;
            best_pos = *pos;
        }
    }
    (best_pos, shortest_dist)
}

fn solve_min_dist_position_part2(crab_positions: &[i32; 1000]) -> (i32, i32)
{
    let mut best_pos: i32 = 0;
    let mut shortest_dist = i32::MAX;
    let mut current_dist;

    for pos in crab_positions
    {
        current_dist = 0;
        for val in crab_positions
        {
            current_dist += TRIANGLE_NUMS[(*val - *pos).abs() as usize];
            if current_dist > shortest_dist { break; }
        }
        if current_dist < shortest_dist
        {
            shortest_dist = current_dist;
            best_pos = *pos;
        }
    }
    (best_pos, shortest_dist)
}

/// Self generating code file, neat!
fn write_triangle_nums_to_file()
{
    let mut file = File::create("day7/src/triangle_nums1.rs").unwrap();
    let mut output = String::new();
    output += "pub static TRIANGLE_NUMS: [i32; 2001] = [";
    for val in 0..=2000
    {
        let mut triangle_num = 0;
        compute_triangle_number(val, &mut triangle_num);
        output += &*triangle_num.to_string();
        if val != 2000 { output += ","; }
    }
    output += "];";
    file.write_all(&*output.into_bytes()).unwrap();
}

fn compute_triangle_number(num: i32, result: &mut i32)
{
    if num == 0 { return; }
    *result += num;
    compute_triangle_number(num - 1, result);
}