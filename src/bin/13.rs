use std::{fs::read_to_string};
fn main(){
    let input = read_to_string("input/input13.txt").unwrap();
    let solution = solve(input.as_str());

    println!("Day 13: Part 1: {} Part 2: {}", solution.0, solution.1);
}

fn solve(input : &str) -> (i32, i32){

    let mut part1 = 0;
    let mut part2 = 0;

    input.split("\r\n\r\n").for_each(| pattern | {
        let mut pattern : Vec<Vec<char>> = pattern.split("\n").map(|line| {
            line.chars().filter(|x| x != &'\r').collect::<Vec<char>>()
        }).collect::<Vec<Vec<char>>>();

        let mut old_score = 0;
        let mut score = 0;

        for line in 0..pattern.len(){
            for char in 0..pattern[0].len(){

                let old_vert = get_vertical(&pattern, 0);
                let old_horizontal = get_horizontal(&pattern, 0);
                
                pattern[line][char] = flip_char(pattern[line][char]);
                
                let vert = get_vertical(&pattern, old_vert.try_into().unwrap());
                if vert > 0 && vert != old_vert {
                    println!("Vert {}", vert);
                    score = vert;
                }

                let horizontal = get_horizontal(&pattern, old_horizontal.try_into().unwrap()) * 100;

                if horizontal > 0 && horizontal != old_horizontal{
                    score = horizontal;
                }

                pattern[line][char] = flip_char(pattern[line][char]);

                old_score = old_vert + (old_horizontal*100);

                if score > 0 { 
                    break; 
                }
            }
            if score > 0 { break; }
        }

        part1 += old_score;
        part2 += score
    });

    (part1, part2)
}

fn get_horizontal(grid : &Vec<Vec<char>>, prev : usize) -> i32{
    let mut reflection = 0;
    let len = grid.len();
    for i in 1..len{
        if grid[i] == grid[i-1]{
            let mut valid = true;
            for j in i+1..len{
                match grid.get(j)  {
                            Some(_x) => match (i+i-1).checked_sub(j) {
                                Some(val) => {
                                    if grid[val] == grid[j] {
                                        valid = true;
                                    } else {
                                        valid = false;
                                        break;
                                    }
                                },
                                None => {
                                    break;
                                }
                            },
                            None => {},
                        }
                        if j >= (len-1) {break;}
            }
            if valid && i != prev {
                reflection = i
            }
        }
    }
    reflection.try_into().unwrap()
}

fn get_vertical(grid : &Vec<Vec<char>>, prev: usize) -> i32 {
    let mut reflection = 0;
    let element_length = grid[0].len();
    let length = grid.len();


    for column in 1..element_length{
        let mut is_match = true;
        for row in 0..length {
            if grid[row][column] != grid[row][column-1] {
                is_match = false
            }
            if !is_match {
                break;
            }
        }
        if is_match {
            for last in column+1..element_length{
                match (column+column-1).checked_sub(last) {
                    Some(first) => { 
                        for i in 0..length {
                            if grid[i][first] != grid[i][last] {
                                is_match = false;
                                break;
                            }
                        }
                    },
                    None => break,
                };

                if column > element_length - 1 {break;}
            }
            if is_match && column != prev{
                reflection = column;
            }
        }
        if reflection > 0 {break;}
    }

    reflection.try_into().unwrap()
}

fn flip_char(c: char) -> char{
    match c {
        '.' => '#',
        '#' => '.',
        _ => ' '
    }
}