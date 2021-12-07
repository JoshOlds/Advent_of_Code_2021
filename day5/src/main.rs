use crate::input::PUZZLE_INPUT;
use std::fs::File;
use std::io::prelude::*;
use crate::line::Line;

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
    let mut diag_lines: Vec<&line::Line> = Vec::with_capacity(500);
    let input_vec: Vec<&str> = PUZZLE_INPUT.split("\n").collect();
    parse_to_vector(&mut lines, input_vec);

    // Part 1
    // store touched coordinates in a 2D array
    let mut coord_array : [[u8; 1000]; 1000] = [[0; 1000]; 1000];
    let mut overlap_count = 0;

    // Iterate over lines and populate map frequency using points as keys
    let (mut start, mut end);
    for line in &lines
    {
        if line.is_vertical()
        {
            // Loop must run from smallest number to biggest number
            if line.start.y < line.end.y {
                start = line.start.y;
                end = line.end.y;
            } else {
                start = line.end.y;
                end = line.start.y;
            };
            for val in start..=end
            {
                let val  = &mut coord_array[line.start.x][val];
                if *val == 1 { overlap_count += 1;}
                *val += 1;
            }
        }
        else if line.is_horizontal()
        {
            // Loop must run from smallest number to biggest number
            if line.start.x < line.end.x {
                start = line.start.x;
                end = line.end.x;
            } else {
                start = line.end.x;
                end = line.start.x;
            };
            for val in start..=end
            {
                let val  = &mut coord_array[val][line.start.y];
                if *val == 1 { overlap_count += 1;}
                *val += 1;
            }
        }
        else {
            diag_lines.push(line);
        }
    }

    if do_print{println!("Part 1: {}", overlap_count);}


    // Part 2
    // Iterate over lines and populate map frequency using points as keys
    for line in diag_lines
    {
        // Justify lines so x start is always lower
        let start = if line.start.x < line.end.x { &line.start } else { &line.end };
        let end = if line.start.x < line.end.x { &line.end } else { &line.start };
        let dist = end.x - start.x;

        if start.y < end.y // Different math whether an up or downwards 45 degree line
        {
            for i in 0..=dist
            {
                let val = &mut coord_array[start.x + i][start.y + i];
                if *val == 1{ overlap_count += 1;}
                *val += 1;
            }
        }
        else
        {
            for i in 0..=dist
            {
                let val = &mut coord_array[start.x + i][start.y - i];
                if *val == 1{ overlap_count += 1;}
                *val += 1;
            }
        }
    }

    if do_print{println!("Part 2: {}", overlap_count);}

    // Write out to file for visualization
    if VISUALIZE_OUTPUT
    {
        let mut file = File::create("day5/map.txt").unwrap();
        file.write_all(get_array_as_string(&coord_array).as_bytes()).unwrap();
    }
}

fn parse_to_vector(lines: &mut Vec<Line>, input_vec: Vec<&str>) {
    for input in input_vec
    {
        let mut points_iter = input.split(" -> ");
        let mut d1 = points_iter.next().unwrap().split(",");
        let mut d2 = points_iter.next().unwrap().split(",");
        let p1 = line::Point {
            x: d1.next().unwrap().parse().unwrap(),
            y: d1.next().unwrap().parse().unwrap()
        };
        let p2 = line::Point {
            x: d2.next().unwrap().parse().unwrap(),
            y: d2.next().unwrap().parse().unwrap()
        };
        let line = line::Line { start: p1, end: p2 };
        lines.push(line);
    }
}

fn get_array_as_string(map: &[[u8; 1000]; 1000]) -> String
{
    let mut str = String::with_capacity(1002000);
    for y in 0..1000
    {
        for x in 0..1000
        {
            str += &*map[x][y].to_string();
        }
        str += "\n";
    }
    str
}
