use crate::input::PUZZLE_INPUT;
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;

mod input;
mod line;

pub static VISUALIZE_OUTPUT: bool = false;

fn main() {
    adventlib::input_helpers::print_puzzle_header(5);
    adventlib::measure_execution_time(run, 1000);
}

fn run(do_print: bool) {
    // Populate vector of Lines
    let mut lines: Vec<line::Line> = Vec::with_capacity(500);
    let input_vec: Vec<&str> = PUZZLE_INPUT.split("\n").collect();
    for input in input_vec
    {
        let mut points_iter = input.split(" -> ");
        let mut d1 = points_iter.next().unwrap().split(",");
        let mut d2 = points_iter.next().unwrap().split(",");
        let p1 = line::Point{
            x: d1.next().unwrap().parse().unwrap(),
            y: d1.next().unwrap().parse().unwrap()
        };
        let p2 = line::Point{
            x: d2.next().unwrap().parse().unwrap(),
            y: d2.next().unwrap().parse().unwrap()
        };
        let line = line::Line{start: p1, end: p2};
        lines.push(line);
    }

    // Part 1
    // filter only horizontal or vertical
    let straight_lines = lines.iter().filter(|line| line.is_vertical() || line.is_horizontal());
    // store touched coordinates in a hashmap
    let mut coord_hash :HashMap<line::Point, usize> = HashMap::with_capacity(110000);

    // Iterate over lines and populate map frequency using points as keys
    for line in straight_lines
    {
        if line.start.x == line.end.x // vertical line
        {
            // Loop must run from smallest number to biggest number
            let start = if line.start.y < line.end.y { line.start.y } else { line.end.y };
            let end = if line.start.y < line.end.y { line.end.y } else { line.start.y };
            for val in start..=end
            {
                *coord_hash.entry(line::Point{x: line.start.x, y: val}).or_insert(0) += 1;
            }
        }
        else if line.start.y == line.end.y // horizontal line
        {
            // Loop must run from smallest number to biggest number
            let start = if line.start.x < line.end.x { line.start.x } else { line.end.x };
            let end = if line.start.x < line.end.x { line.end.x } else { line.start.x };
            for val in start..=end
            {
                *coord_hash.entry(line::Point{x: val, y: line.start.y}).or_insert(0) += 1;
            }
        }
    }

    // Count the frequency of points that overlap
    let mut overlap_count = 0;
    for (_point, freq) in coord_hash.iter()
    {
        if *freq > 1 { overlap_count += 1;}
    }
    if do_print{println!("Part 1: {}", overlap_count);}


    // Part 2
    // Filter out the diagonal lines
    let diagonal_lines = lines.iter().filter(|line| !line.is_vertical() && !line.is_horizontal());

    // Iterate over lines and populate map frequency using points as keys
    for line in diagonal_lines
    {
        let start = if line.start.x < line.end.x { &line.start } else { &line.end };
        let end = if line.start.x < line.end.x { &line.end } else { &line.start };
        let dist = end.x - start.x;

        if start.y < end.y
        {
            for i in 0..=dist
            {
                *coord_hash.entry(line::Point{x: start.x + i, y: start.y + i}).or_insert(0) += 1;
            }
        }
        else
        {
            for i in 0..=dist
            {
                *coord_hash.entry(line::Point{x: start.x + i, y: start.y - i}).or_insert(0) += 1;
            }
        }
    }

    overlap_count = 0;
    for (_point, freq) in coord_hash.iter()
    {
        if *freq > 1 { overlap_count += 1;}
    }
    if do_print{println!("Part 2: {}", overlap_count);}

    // Write out to file for visualization
    if VISUALIZE_OUTPUT
    {
        let mut file = File::create("day5/map.txt").unwrap();
        file.write_all(get_map_as_string(&coord_hash).as_bytes()).unwrap();
    }
}

fn get_map_as_string(map: &HashMap<line::Point, usize>) -> String
{
    let mut str = String::with_capacity(1002000);
    let mut count = 0;
    for y in 0..1000
    {
        for x in 0..1000
        {
            match map.get(&line::Point{x:x,y:y})
            {
                Some(val) => {
                    str += &*val.to_string();
                    if *val > 1 {count += 1;}
                },
                None => str += "0"
            }
        }
        str += "\n";
    }
    println!("Count! {}", count);
    str
}
