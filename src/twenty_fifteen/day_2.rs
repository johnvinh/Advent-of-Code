// Advent of Code 2015 Day 2
// https://adventofcode.com/2015/day/2

use std::cmp;

pub struct Present {
    width: i32,
    length: i32,
    height: i32,
}

pub enum Part {
    Part1,
    Part2,
}

pub mod part_1 {
    pub fn calculate_surface_area(present: &super::Present) -> i32 {
        (2 * present.length * present.width) + (2 * present.width * present.height) + (2 * present.height * present.length)
    }

    pub fn calculate_extra(present: &super::Present) -> i32 {
        let length_width: i32 = present.length * present.width;
        let width_height: i32 = present.width * present.height;
        let height_length: i32 = present.height * present.length;
        // Return the minimum of the 3 values
        super::cmp::min(super::cmp::min(length_width, width_height), height_length)
    }
}

pub mod part_2 {
    pub fn calculate_cubic_volume(present: &super::Present) -> i32 {
        present.width * present.height * present.length
    }

    pub fn calculate_smallest_perimeter(present: &super::Present) -> i32 {
        let mut measures: Vec<i32> = vec!(present.width, present.height, present.length);
        measures.sort();
        (2 * measures[0]) + (2 * measures[1])
    }
}

pub fn solve(input: Vec<&str>, part: &Part) -> i32 {
    let mut sum: i32 = 0;
    for line in input {
        let dimensions: Vec<&str> = line.split("x").collect();
        let present: Present = Present {
            width: dimensions[0].trim().parse().expect("Failed to read width"),
            height: dimensions[1].trim().parse().expect("Failed to read height"),
            length: dimensions[2].trim().parse().expect("Failed to read length"),
        };
        match part {
            Part::Part1=> sum += part_1::calculate_surface_area(&present) + part_1::calculate_extra(&present),
            Part::Part2=> sum += part_2::calculate_cubic_volume(&present) + part_2::calculate_smallest_perimeter(&present),
        }
    }
    sum
}