use crate::input::{PUZZLE_INPUT};
use crate::input_gen::PUZZLE_INPUT_GEN; //, TEST_INPUT};

mod input;
mod input_gen;

pub static BRUTE_FORCE: bool = false;

fn main() {
    adventlib::input_helpers::print_puzzle_header(6);
    adventlib::measure_execution_time_ns(run, 100000);
}

fn run(do_print: bool) {

    // Represent fish numbers by their age in an array
    let mut fish_array : [usize; 9] = [0; 9];
    // Populate starting fish
    for starting_fish in PUZZLE_INPUT_GEN
    {
        fish_array[starting_fish] += 1;
    }

    let mut part1 = simulate_fishies(&fish_array, 80);
    if do_print{println!("Part 1: {}", part1)}
    let mut part2 = simulate_fishies(&fish_array, 256);
    if do_print{println!("Part 2: {}", part2)}

    // Brute force implementation for fun!
    if BRUTE_FORCE
    {
        let mut fish_vec_u8 : Vec<u8> = adventlib::input_helpers::split_string_to_vector(PUZZLE_INPUT, ",");
        part1 = brute_force_fishies(&mut fish_vec_u8, 80, false);
        if do_print{println!("Part 1: {}", part1)}
        part2 = brute_force_fishies(&mut fish_vec_u8, 256, true);
        if do_print{println!("Part 2: {}", part2)}
    }

}

// This is infeasible and will use all system RAM before completing LOL
fn brute_force_fishies(fish_vec: &mut Vec<u8>, days: i32, print_progress: bool) -> usize
{
    let mut fish_to_add;
    for day in 0..days
    {
        if print_progress { println!("Day {}: Simulating fishies...", day);}
        fish_to_add = 0;
        for fish in fish_vec.iter_mut()
        {
            if *fish == 0{
                *fish = 7;
                fish_to_add += 1;
            }
            *fish -= 1;
        }
        for _new in 0..fish_to_add
        {
            fish_vec.push(8);
        }
    }
    fish_vec.len()
}

// A much more elegant fish simulation
fn simulate_fishies(fish_array: &[usize; 9], days: usize) -> usize
{
    let mut fish_count: usize = 0;
    let mut new_fish_arr: [usize; 9] = [0; 9];
    let mut last_fish_arr: [usize; 9] = *fish_array;
    for _day in 0..days
    {
        new_fish_arr[8] = last_fish_arr[0];
        new_fish_arr[7] = last_fish_arr[8];
        new_fish_arr[6] = last_fish_arr[7] + last_fish_arr[0];
        new_fish_arr[5] = last_fish_arr[6];
        new_fish_arr[4] = last_fish_arr[5];
        new_fish_arr[3] = last_fish_arr[4];
        new_fish_arr[2] = last_fish_arr[3];
        new_fish_arr[1] = last_fish_arr[2];
        new_fish_arr[0] = last_fish_arr[1];
        last_fish_arr = new_fish_arr;
    }
    for fish in new_fish_arr
    {
        fish_count += fish;
    }
    fish_count
}