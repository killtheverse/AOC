use std::fs;

use aoc2022::day_10;

fn main() {
    let filepath = "inputs/day_10.txt";
    let input = fs::read_to_string(filepath)
    .expect("Unable to read input file.");
    
    day_10::part_1(&input);
    day_10::part_2(&input);
}
