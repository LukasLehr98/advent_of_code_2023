use std::fs::read_to_string;
fn main(){
    let input = read_to_string("input/input13.txt").unwrap();
    let part1 = solve(input.as_str());

    println!("{}", part1);
}

fn solve(input : &str) -> i32{
    input.split("\r\n\r\n").map(| pattern | {
        let pattern : Vec<Vec<char>> = pattern.split("\n").map(|line| {
            line.chars().filter(|x| x != &'\r').collect::<Vec<char>>()
        }).collect::<Vec<Vec<char>>>();
        let vert = get_vertical(&pattern);
        let horizontal = get_horizontal(&pattern) * 100;

        println!("Vert : {}", vert);
        println!("Horizontal: {}", horizontal);

        horizontal + vert
    }).sum()
}

fn get_horizontal(grid : &Vec<Vec<char>>) -> i32{
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
            if valid {
                reflection = i
            }
        }
    }
    reflection.try_into().unwrap()
}

fn get_vertical(grid : &Vec<Vec<char>>) -> i32 {
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
            if is_match {
                reflection = column;
            }
        }
        if reflection > 0 {break;}
    }

    reflection.try_into().unwrap()
}
