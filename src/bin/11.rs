use std::fs::read_to_string;

fn main() {
    let input = read_to_string("input/input11.txt").unwrap();
    let cols = expand(input.as_str());
    let part1 = solve(input.as_str(), cols.clone(), 2);
    let part2 = solve(input.as_str(), cols.clone(), 1000000);

    println!("part 1: {}, Part 2: {}", part1, part2);
}

fn expand(input :&str) -> Vec<bool>{
    let mut columns: Vec<bool> = Vec::new();

    for (_i, l) in input.split("\n").into_iter().enumerate(){
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
    let mut current_line: usize = 0;
    let mut current_col: usize = 0;

    for (_i, l) in input.split("\n").into_iter().enumerate(){
        let mut new_line : Vec<char> = Vec::new();
        for (j, char) in l.chars().enumerate(){

            if char == '#' { galaxies.push((current_line, current_col)) }
            
            else if !columns[j] { current_col += zoom -1 }

            new_line.push(char);
            current_col += 1
        }
        if !new_line.contains(&'#') { 
            current_line += zoom -1;
            }
        current_line += 1;
        current_col = 0;
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