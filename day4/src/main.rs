use adventlib;
use crate::input::PUZZLE_INPUT;

mod input;

fn main() {
    adventlib::input_helpers::print_puzzle_header(4);
    adventlib::measure_execution_time(run, 1);
}
#[derive(Default)]
struct Board
{
    grid: [[usize; 5]; 5],
}

impl Board{
    fn from_vec_slice(input: &Vec<&str>) -> Board
    {
        for row_idx in 0..5
        {
            let row = input[row_idx];
            for val_str in row.split(" ")
            {
                let val: usize = val_str.parse().unwrap()
            }
        }
        Board::default()
    }
}

fn run(do_print: bool) {
    let input_vec: Vec<&str> = PUZZLE_INPUT.split("\n").collect();

    let bingo_nums : Vec<usize> = input_vec[0].parse().unwrap();

}