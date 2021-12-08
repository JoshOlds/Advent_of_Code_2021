use adventlib;
mod input;

fn main() {
    adventlib::input_helpers::print_puzzle_header(1);
    adventlib::measure_execution_time_us(run, 10000);
}

fn run(do_print: bool) {
    let input_vec: Vec<i32> = adventlib::input_helpers::split_string_to_vector(input::PUZZLE_INPUT, "\n");
    let mut count = 0;

    // Part 1 calculation
    input_vec
        .windows(2)
        .for_each(|w| {
            if w[0] < w[1] { count += 1; }
        } );
    if do_print{ println!("Part 1: {}", count); }

    // Part 2 calculation
    count = 0;
    let triplet_sums: Vec<i32> = input_vec
        .windows(3)
        .map(|w| w[0] + w[1] + w[2])
        .collect();
    for (current, next) in triplet_sums.iter().zip(triplet_sums.iter().skip(1))
    {
        if *current < *next
        {
            count += 1;
        }
    }

    if do_print { println!("Part 2: {}", count);}
}
