use std::fs::read_to_string;

use itertools::Itertools;

fn main(){
    let input = read_to_string("input/input13.txt").unwrap();
    
    let part1 = solve(input.as_str());

    println!("{}", part1);
}

fn solve(input : &str) -> usize{
    input.split("\r\n\r\n").map(| pattern | {
        let pattern : Vec<Vec<u8>> = pattern.split("\n").map(|line| {
            line.chars().collect::<Vec<char>>()
        }).collect::<Vec<Vec<char>>>();


    });

    0

}
