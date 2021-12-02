use adventlib;

mod input;

fn main() {
    adventlib::print_puzzle_header(1);

    let input_vec: Vec<i32> = adventlib::split_string_to_vector(input::PUZZLE_INPUT, "\n");
    let mut last_num = input_vec[0];
    let mut count = 0;

    // Part one calculation
    for num in &input_vec
    {
        if *num > last_num{
            count += 1 ;
        }
        last_num = *num;
    }
    println!("Part 1: {}", count);

    // Part 2 calculation
    count = 0;
    let triplet_sums: Vec<i32> = input_vec.windows(3).map(|w| w[0] + w[1] + w[2]).collect();
    for (current, next) in triplet_sums.iter().zip(triplet_sums.iter().skip(1))
    {
        if *current < *next
        {
            count += 1;
        }
    }

    println!("Part 2: {}", count);

}
