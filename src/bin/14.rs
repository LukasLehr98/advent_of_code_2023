use std::fs::read_to_string;

use hashbrown::HashSet;

fn main(){
    let input = read_to_string("input/input14.txt").unwrap();
    let part1 = solve(&input);

    println!("Part 1: {}", part1);
}

fn solve(input : &str) -> usize{
    let mut weights : usize = 0;
    let mut rocks : HashSet<(usize, usize)> = HashSet::new();
    let input = input.split("\r\n");
    let length = &input.clone().count();

    println!("Length: {}", length);

    for (i, line) in input.into_iter().enumerate(){
        for (j, c) in line.chars().into_iter().enumerate(){
            match c {
                'O' => {
                    let mut inserted = false;
                    for looking_up in (0..=i).rev(){
                        if rocks.contains(&(looking_up,j)) {
                            inserted = rocks.insert((looking_up+1, j));
                            weights += length - (looking_up + 1);
                            break;
                        }
                    }
                    if !inserted { 
                        rocks.insert((i,j)); 
                        weights += length - i;
                    }
                },
                '#' => {
                    rocks.insert((i, j));
                }
                _ => {},
            }
        }
        println!(" {} ", weights);
    }
    weights
}
