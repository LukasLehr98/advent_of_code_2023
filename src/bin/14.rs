use std::fs::read_to_string;

fn main(){
    let input = read_to_string("input/input14.txt").unwrap();
    let part1_grid = roll(parse_grid(&input));

    let mut grid = parse_grid(&input);
    let mut cycle_weights : Vec<usize> = Vec::new();

    for _cycle in 0..=1000{
        cycle_weights.push(get_load(&grid));

        for _direction in 0..4{
            grid = roll(grid);
            grid = rotate_grid(&grid);
        }
    }

    let mut cycle_length = 0;

    for i in (0..cycle_weights.len() - 2).rev(){
        if cycle_weights[i] == cycle_weights[cycle_weights.len()-1] {
            cycle_length = (cycle_weights.len()-1) - i;
            break;
        }
    }

    let mut result = 0;
    let remainder = 1000000000 % cycle_length;

    for i in (0..cycle_weights.len() - 1).rev() {
        if i % cycle_length == remainder {
            result = cycle_weights[i];
            break;
        }
    }

    println!("Day 14: Part 1: {}, Part 2: {}", get_load(&part1_grid), result);

}

fn parse_grid(input_str: &str) -> Vec<Vec<char>> {
    input_str
        .lines()
        .collect::<Vec<_>>()
        .iter()
        .map(|line| line.trim())
        .filter(|line| !line.is_empty())
        .map(|line| line.chars().collect())
        .collect()
}

fn rotate_grid(grid: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut result: Vec<Vec<char>> = vec![vec!['.'; grid.len()]; grid[0].len()];
    for col in 0..grid[0].len() {
        for row in 0..grid.len() {
            result[col][grid[0].len() - 1 - row] = grid[row][col];
        }
    }
    result
}

fn roll(mut grid: Vec<Vec<char>>) -> Vec<Vec<char>>{
    for row in 0..grid.len(){
        for col in 0..grid[0].len(){
            match grid[row][col] {
                'O' => {
                    for new_row in (0..=row).rev(){
                        let mut moved = false;
                        if (grid[new_row][col]) == '#' || (grid[new_row][col]) == 'O'{
                            if new_row != row {
                                grid[row][col] = '.';
                                grid[new_row+1][col] = 'O';
                                moved = true;
                            }
                        }
                        else if new_row == 0 {
                            grid[row][col] = '.';
                            grid[0][col] = 'O';
                            moved = true;
                        }
                        if moved { break; }
                    }
                }
                _ => {},
            }
        }
    }
    grid
}

fn get_load(grid : &Vec<Vec<char>>) -> usize{
    let mut weight = 0;

    for i in 0..grid.len(){
        for j in 0..grid[0].len(){
            if grid[i][j] == 'O'{
                weight += grid.len() - i;
            }
        }
    }
    weight
}