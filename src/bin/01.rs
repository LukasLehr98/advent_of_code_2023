use std::fs::read_to_string;


pub fn main() {
    let input = read_to_string("input/input1.txt").unwrap();

    let part1 : u32= input.as_str().split("\n").map(|line| get_calibration_values(line.to_string())).sum();
    let part2 = get_calibration_replace(input.as_str().split("\n").collect());
    println!("Day 1: Part 1:{}, Part 2:{}", part1, part2)
}

fn get_calibration_values(line : String) -> u32{
    let results_vec = line.chars().filter_map(|c: char| c.to_digit(10)).collect::<Vec<u32>>();
    return results_vec.first().unwrap() * 10 + results_vec.last().unwrap();
}

fn get_calibration_replace(lines : Vec<&str>) -> u32{
    lines.into_iter().map(|line| {
        line
        .replace("one", "one1one")
        .replace("two", "two2two")
        .replace("three", "three3three")
        .replace("four", "four4four")
        .replace("five", "five5five")
        .replace("six", "six6six")
        .replace("seven", "seven7seven")
        .replace("eight", "eight8eight")
        .replace("nine", "nine9nine")
    }).map(|line| {
        get_calibration_values(line)
    }).sum()
}