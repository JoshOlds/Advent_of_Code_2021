use adventlib;
use crate::input::PUZZLE_INPUT;

mod input;

fn main() {
    adventlib::input_helpers::print_puzzle_header(2);
    adventlib::measure_execution_time_us(run, 10000);
}

fn run(do_print: bool) {
    let input_vec: Vec<&str> = PUZZLE_INPUT.split("\n").collect();

    // Part 1
    let mut horizontal = 0;
    let mut depth = 0;
    for line in &input_vec
    {
        if line.chars().next().unwrap() == 'f'
        {
            horizontal += line[8..].parse::<u32>().unwrap();
        }
        else if line.chars().next().unwrap() == 'd'
        {
            depth += line[5..].parse::<u32>().unwrap();
        }
        else if line.chars().next().unwrap() == 'u'
        {
            depth -= line[3..].parse::<u32>().unwrap();
        }
    }
    if do_print {
        println!("Part 1 depth: {}", depth);
        println!("Part 1 horizontal: {}", horizontal);
        println!("Part 1 answer: {}", depth * horizontal);
    }

    // Part 2
    horizontal = 0;
    depth = 0;
    let mut aim = 0;
    for line in &input_vec
    {
        if line.chars().next().unwrap() == 'f'
        {
            let val = line[8..].parse::<u32>().unwrap();
            horizontal += val;
            depth += aim * val;
        }
        else if line.chars().next().unwrap() == 'd'
        {
            aim += line[5..].parse::<u32>().unwrap();
        }
        else if line.chars().next().unwrap() == 'u'
        {
            aim -= line[3..].parse::<u32>().unwrap();
        }
    }
    if do_print {
        println!("Part 2 depth: {}", depth);
        println!("Part 2 horizontal: {}", horizontal);
        println!("Part 2 answer: {}", depth * horizontal);
    }
}