use adventlib;
use crate::input::PUZZLE_INPUT;
use std::hash::BuildHasherDefault;
use std::collections::HashMap;
use seahash::SeaHasher;

mod input;

type SeaHashBuilder = BuildHasherDefault<SeaHasher>;

fn main() {
    adventlib::input_helpers::print_puzzle_header(4);
    adventlib::measure_execution_time_us(run, 1000);
}

#[derive(Default, PartialEq)]
pub struct Board
{
    grid: [[usize; 5]; 5],
    has_won: bool,
    numbers_for_recheck: usize,
}

/// A 5x5 bingo board
impl Board{
    pub fn from_vec_slice(input: &[&str]) -> Self
    {
        let mut board = Board::default();
        for row_idx in 0..5
        {
            let row = input[row_idx];
            let mut col_idx = 0;
            for val_str in row.split(" ")
            {
                match val_str.parse()
                {
                    Ok(val) => {
                        board.grid[row_idx][col_idx] = val;
                        col_idx += 1;
                    },
                    Err(_e) => {} // skip the blank extra lines
                }
            }
        }
        board
    }

    pub fn check_for_win(&mut self, marked_numbers_hash: &HashMap<usize, bool, SeaHashBuilder>) -> bool
    {
        // check rows
        let mut count;
        let mut max_count = 0; // Track how many matches we have, and track that x more numbers must be added for a potential win
        for row in 0..5
        {
            count = 0;
            for col in 0..5
            {
                if marked_numbers_hash.contains_key(&self.grid[row][col])
                {
                    count += 1;
                    if count > max_count {max_count = count;}
                }
                else { break; } // Skip if we miss one
            }
            if count == 5 {return true;}
        }
        // check cols
        for col in 0..5
        {
            count = 0;
            for row in 0..5
            {
                if marked_numbers_hash.contains_key(&self.grid[row][col]) {
                    count += 1;
                    if count > max_count {max_count = count;}
                }
                else { break; } // Skip if we miss one
            }
            if count == 5 {return true;}
        }
    self.numbers_for_recheck = 5 - max_count;
    return false;
    }

    pub fn sum_unmarked(&self, marked_numbers_hash: &HashMap<usize, bool, SeaHashBuilder>) -> usize
    {
        let mut sum = 0;
        for row in 0..5
        {
            for col in 0..5
            {
                if marked_numbers_hash.contains_key(&self.grid[row][col]) == false
                {
                    sum += &self.grid[row][col];
                }
            }
        }
        sum
    }
}

fn run(do_print: bool) {
    let input_vec: Vec<&str> = PUZZLE_INPUT
        .split("\n")
        .filter(|line| line.is_empty() == false)
        .collect();

    let bingo_nums : Vec<usize> = input_vec[0]
        .split(",")
        .map(|val| val.parse().unwrap())
        .collect();

    // Parse all boards
    let board_amount = (input_vec.len() - 1) / 5;
    let mut boards : Vec<Board> = Vec::with_capacity(board_amount);
    for i in 0..board_amount
    {
        let start = (i * 5) + 1;
        let end = start + 5;
        boards.push(Board::from_vec_slice(&input_vec[start..end]));
    }

    // Keep adding marked numbers until we get a winning board
    let mut marked_numbers_hash : HashMap<usize, bool, SeaHashBuilder> = HashMap::with_capacity_and_hasher(100, SeaHashBuilder::default());
    let (mut done, mut first) = (false, false);
    let mut win_count = 0;
    let boards_amt = boards.len();

    for (idx, val) in bingo_nums.iter().enumerate()
    {
        marked_numbers_hash.insert(*val, true);
        for board in boards.iter_mut()
        {
            if board.numbers_for_recheck > 0 {board.numbers_for_recheck -= 1;}
        }

        if idx >= 4 // Can't have a bingo without 5 numbers...
        {
            for board in boards.iter_mut()
            {
                if board.has_won { continue; } // Short circuit if this board has already won
                if board.numbers_for_recheck > 0 { continue; } // Short circuit if this board doesn't need to be rechecked
                if board.check_for_win(&marked_numbers_hash)
                {
                    win_count += 1;
                    board.has_won = true;
                    // Only do on first bingo!
                    if !first {
                        let unmarked_score = board.sum_unmarked(&marked_numbers_hash);
                        if do_print { println!("Part 1: {}", unmarked_score * val); }
                        first = true;
                    }
                    // On final bingo!
                    if win_count == boards_amt
                    {
                        let unmarked_score = board.sum_unmarked(&marked_numbers_hash);
                        if do_print {
                            println!("Part 2: {}", unmarked_score * val);
                            println!("Final Bingo on marked number: {} of {}", idx, &bingo_nums.len());
                        }
                        done = true;
                        break;
                    }
                }
            }
            if done { break; }
        }
    }
}

