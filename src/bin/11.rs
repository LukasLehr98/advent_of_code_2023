use std::{fs::read_to_string};

fn main() {
    let input = read_to_string("input/input11.txt").unwrap();
    let cols = expand(input.as_str());
    let part1 = solve(input.as_str(), cols.clone(), 2);
    let part2 = solve(input.as_str(), cols.clone(), 1000000);

    println!("part 1: {}, Part 2: {}", part1, part2);
}

fn expand(input :&str) -> Vec<bool>{
    let mut columns: Vec<bool> = Vec::new();

    for (i, l) in input.split("\n").into_iter().enumerate(){
        if columns.len() == 0 {
            columns = vec![false; l.len()];
          }

        for (j, char) in l.chars().enumerate(){
            if char == '#' { 
                columns[j] = true;
             } }
    }

    columns
}

fn solve(input: &str, columns: Vec<bool>, zoom : usize) -> usize {
    let mut galaxies : Vec<(usize, usize)> = Vec::new();
    let mut currentLine: usize = 0;
    let mut currentCol: usize = 0;

    for (i, l) in input.split("\n").into_iter().enumerate(){
        let mut newLine : Vec<char> = Vec::new();
        for (j, char) in l.chars().enumerate(){

            if char == '#' { galaxies.push((currentLine, currentCol)) }
            
            else if !columns[j] { currentCol += zoom -1 }

            newLine.push(char);
            currentCol += 1
        }
        if !newLine.contains(&'#') { 
            currentLine += zoom -1;
            }
        currentLine += 1;
        currentCol = 0;
    };
    
    galaxies.reverse();


    let mut distance = 0;
    let mut current_index : (usize, usize) = galaxies.last().unwrap().clone();

    while galaxies.len() > 1 {
        for i in &galaxies {
            if i == &current_index { continue }
            let local_dist = current_index.0.abs_diff(i.0) + current_index.1.abs_diff(i.1);

            distance += local_dist
        }
        galaxies.pop();
        current_index = galaxies.last().unwrap().clone();
    }

    distance
}