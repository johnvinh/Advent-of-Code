// Advent of Code 2015 Day 3
// https://adventofcode.com/2015/day/3

pub mod part_1 {
    use std::collections::HashSet;
    use std::hash::{Hash, Hasher};
    use std::sync::PoisonError;

    #[derive(Debug, Eq, Hash)]
    pub struct Position {
        pub x: i32,
        pub y: i32,
    }

    impl Position {
        pub fn new() -> Position {
            Position {
                x: 0,
                y: 0,
            }
        }

        pub fn move_north(&mut self) {
            self.y += 1;
        }

        pub fn move_south(&mut self) {
            self.y -= 1;
        }

        pub fn move_east(&mut self) {
            self.x += 1;
        }

        pub fn move_west(&mut self) {
            self.x -= 1;
        }
    }

    impl PartialEq for Position {
        fn eq(&self, other: &Self) -> bool {
            (self.x == other.x) && (self.y == other.y)
        }
    }

    impl Clone for Position {
        fn clone(&self) -> Self {
            Position {
                x: self.x,
                y: self.y,
            }
        }
    }

    pub fn solve(input: String) -> Result<i32, &'static str> {
        // (X, Y) value key value pairs
        let mut current_position: Position = Position::new();
        let mut houses = HashSet::new();
        houses.insert(current_position.clone());
        for character in input.chars() {
            match character {
                '>' => current_position.move_east(),
                '<' => current_position.move_west(),
                '^' => current_position.move_north(),
                'v' => current_position.move_south(),
                _ => return Err("Invalid input"),
            }
            houses.insert(current_position.clone());
        }
        Ok(houses.len() as i32)
    }
}

#[cfg(test)]
mod tests {
    use super::part_1::*;

    #[test]
    pub fn test_position() {
        let mut position: Position = Position::new();
        position.move_west();
        assert_eq!(Position{x: -1, y: 0}, position);
        position.move_north();
        assert_eq!(Position{x: -1, y: 1}, position);
        position.move_east();
        assert_eq!(Position{x: 0, y: 1}, position);
        position.move_south();
        assert_eq!(Position{x: 0, y: 0}, position);
    }

    #[test]
    pub fn solve_test_cases() {
        assert_eq!(solve(String::from(">")).unwrap(), 2);
        assert_eq!(solve(String::from("^>v<")).unwrap(), 4);
        assert_eq!(solve(String::from("^v^v^v^v^v")).unwrap(), 2);
    }
}