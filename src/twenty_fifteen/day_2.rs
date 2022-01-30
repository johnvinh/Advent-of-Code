use std::cmp;

pub struct Present {
    width: i32,
    length: i32,
    height: i32,
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

    pub fn solve(input: Vec<&str>) -> i32 {
        let mut sum: i32 = 0;
        for line in input {
            let dimensions: Vec<&str> = line.split("x").collect();
            let present: super::Present = super::Present {
                width: dimensions[0].trim().parse().expect("Failed to read width"),
                height: dimensions[1].trim().parse().expect("Failed to read length"),
                length: dimensions[2].trim().parse().expect("Failed to read height"),
            };
            sum += calculate_surface_area(&present) + calculate_extra(&present);
        }
        sum
    }
}

pub mod part_2 {

}