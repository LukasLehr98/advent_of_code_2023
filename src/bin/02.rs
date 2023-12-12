use std::fs::read_to_string;


pub fn main() {
    let binding = read_to_string("input/input2.txt").unwrap();
    let input = binding.split("\n").collect::<Vec<&str>>();
    let mut part1 = 0;

    let part2 : i32 = input.into_iter().map(|line| {
        let l = line;
        let mut parts = l.split_once("Game ")
            .unwrap().1.split(|c| c == ':' || c == ';' || c == ',' || c == '\n');
        let game_number : i32 = parts.next().unwrap().trim().parse().unwrap();

        let mut minRed = 1;
        let mut minGreen = 1;
        let mut minBlue = 1;

        for part in parts {
            let tokens: Vec<&str> = part.trim().split_whitespace().collect();
            let quantity: i32 = tokens[0].parse().unwrap();
            let color = tokens[1];

            match color {
                "red" => if quantity > minRed {
                     minRed = quantity 
                    }
                "green" => if quantity >= minGreen { 
                    minGreen = quantity 
                }
                "blue" => if quantity >= minBlue { 
                    minBlue = quantity }
                _ => ()
            }
        };        
        let power = minRed * minGreen * minBlue;
        if is_game_possible((minRed, minGreen, minBlue)) { part1 += game_number } 
        power

    }).map(|num| num).sum();

    println!("Day 2: Part1: {}, Part2: {}", part1, part2);
}


fn is_game_possible(rgb :(i32, i32, i32)) -> bool{
    let inputs = (12, 13, 14);
    rgb.0 <= inputs.1 && rgb.1 <= inputs.1 && rgb.1 <= inputs.1
}