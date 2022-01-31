mod twenty_fifteen;
use std::fs;

fn main() {
    let file_contents = fs::read_to_string("input.txt").expect("Failed to read file");
    let lines: Vec<&str> = file_contents.split("\n").collect();
    let part: twenty_fifteen::day_2::Part = twenty_fifteen::day_2::Part::Part2;
    let result: i32 = twenty_fifteen::day_2::solve(lines, &part);
    println!("The answer is: {}",  result);
}
