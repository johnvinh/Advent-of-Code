use advent_of_code::twenty_fifteen::day_3;
use std::fs;

fn main() {
    let file_contents = fs::read_to_string("input.txt").expect("Failed to read file");
    let answer: i32 = day_3::part_1::solve(file_contents).unwrap();
    println!("The answer is {}", answer);
}
