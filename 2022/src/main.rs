use std::fs;

use aoc2022::day_02;

fn main() {
    let filepath = "inputs/day_02.txt";
    let input = fs::read_to_string(filepath)
    .expect("Unable to read input file.");
    
    day_02::part_1(&input);
    day_02::part_2(&input);
}
