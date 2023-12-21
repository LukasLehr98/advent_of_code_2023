use std::fs::read_to_string;

use hashbrown::HashSet;

fn main(){
    let input = read_to_string("input/input21.txt").unwrap();
    let grid = parse_to_grid(&input);

    let part1 = walk(64, &grid);

    println!("{}", part1);
}

fn walk(max_steps: usize, grid : &Vec<Vec<char>>) -> usize{
    let start = get_start(grid);
    let mut positions : HashSet<(usize, usize)> = HashSet::new();
    positions.insert(start);
    let mut steps = 0;

    while steps < max_steps {
        let mut temp_set = HashSet::new();
        positions.clone().into_iter().for_each(|pos| {
            get_neighbors(grid.len(), pos).unwrap().into_iter().filter(|new_pos| {
                grid[new_pos.0][new_pos.1] != '#'
            }).for_each(| valid_pos | {
                temp_set.insert(valid_pos);
            })
        });

        positions = temp_set;
        steps += 1;
    }

    for i in 0..grid.len(){
        for c in 0..grid[0].len() {
            if positions.contains(&(i, c)) {
                print!("O")
            }
            else { print!("{}", grid[i][c]) }
        }
        println!()
    }

    positions.len()
}

fn parse_to_grid(input : &str) -> Vec<Vec<char>>{
    input.lines().map(|line|{
        line.chars().collect()
    }).collect()
}

fn get_start(grid: &Vec<Vec<char>>) -> (usize, usize){
    for i in 0..grid.len() {
        for j in 0..grid[0].len(){
            if grid[i][j] == 'S' { return (i, j); }
        }
    }
    (0, 0)
}

fn get_neighbors(limit: usize, coords: (usize, usize)) -> Option<Vec<(usize, usize)>>{
    let mut coord_list : Vec<(usize, usize)> = Vec::new();
    for x in coords.0.checked_sub(1).or(Some(coords.0)).unwrap()..=coords.0+1 {
        for y in coords.1.checked_sub(1).or(Some(coords.1)).unwrap()..=coords.1+1 {
            if (x, y) != coords && x < limit && y < limit && (x == coords.0 || y == coords.1) { coord_list.push((x, y))}
        }
    }

    if !coord_list.is_empty() { 
        return Some(coord_list); 
    }

    None
}