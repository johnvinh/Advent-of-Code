pub fn get_floor_difference(c: char) -> i32 {
    match c {
        '(' => 1,
        ')' => -1,
        _ => 0
    }
}

pub fn solve(input: String) -> i32 {
    let mut floor: i32 = 0;
    for character in input.chars() {
        floor += get_floor_difference(character);
    }
    floor
}