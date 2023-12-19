use std::fs::read_to_string;
use hashbrown::HashSet;

fn main(){
    let input = read_to_string("input/input16.txt").unwrap();

    println!("Day 16: Part 1 {}, Part 2: {}", part1(&input), part2(&input))
}

fn parse_to_grid(input : &str) -> Vec<Vec<char>>{
    input.lines().map(|line|{
        line.chars().collect()
    }).collect()
}

fn part2(input: &str) -> usize{
    let grid: Vec<Vec<char>> = parse_to_grid(input);

    let mut starts : Vec<Vec<Beam>> = Vec::new();
    
    for i in 0..grid[0].len(){
        starts.push(vec![Beam::new((0, i), Direction::Down)])
    }

    for i in 0..grid[0].len(){
        starts.push(vec![Beam::new((grid.len()-1, i), Direction::Up)])
    }

    for i in 0..grid.len(){
        starts.push(vec![Beam::new((i, 0), Direction::Right)])
    }

    for i in 0..grid.len(){
        starts.push(vec![Beam::new((i, grid.len()-1), Direction::Left)])
    }

    get_energized_tiles(starts, grid, false)
}

fn part1(input: &str) -> usize{
    let grid = parse_to_grid(input);
    get_energized_tiles(vec![vec![Beam::new((0, 0), Direction::Right)]], grid, false)
}

fn get_energized_tiles(starts : Vec<Vec<Beam>>, grid: Vec<Vec<char>>, printing: bool) -> usize{
    starts.into_iter().map(| mut beams | {
    let mut visited : HashSet<((usize, usize), usize)> = HashSet::new();

    loop {
        match beams.pop(){
           Some(mut beam) => {
               if visited.len() == 0{
                   beam.direction = beam.reflect(grid[beam.current.0][beam.current.1]).0
               }
               visited.insert((beam.current, beam.direction.get_literal()));
               loop {
                   match beam.get_valid_coords_from_direction(grid[0].len().try_into().unwrap()){
                       Some(coords) => {
                           visited.insert((beam.current, beam.direction.get_literal()));
                           
                           beam.current = coords;
                           let char = grid[beam.current.0][beam.current.1];
                           let dirs = beam.reflect(char);
                           
                           beam.direction = dirs.0;
                           
                           if visited.contains(&(coords, beam.direction.get_literal())) && visited.len() != 1{                                
                               break;
                           }
                           
                           match dirs.1 {
                               Some(dir) =>  {

                                   if visited.contains(&(coords, dir.get_literal())) && visited.len() != 1{
                                       break;
                                   }

                                   beams.push(Beam::new(coords, dir))

                               }
                               None => {}
                           }
                       }
                       None => {
                           visited.insert((beam.current, beam.direction.get_literal()));
                           break;
                       }
                   }
               };
           }
           None => { break }
       }
   }
   
   let trimmed : HashSet<(usize, usize)>= visited.clone().into_iter().map(|x| {
       x.0
   }).collect();

   if printing {
       for i in 0..grid.len(){
           for j in 0..grid[0].len() {
               match trimmed.contains(&(i, j, )) {
                   true => { 
                       print!("#")
                   } 
                   false => { 
                       print!{"."} 
                   }
               }
           }
           println!();
       }
   }
   
   trimmed.len()

    }).max().unwrap()
}

#[derive(PartialEq, Clone, Copy, Eq)]
enum Direction {
    Right,
    Left,
    Up,
    Down
}

impl Direction {
    pub fn get_literal(&self) -> usize{
        match self {
            Self::Right => 1,
            Self::Down => 2,
            Self::Left => 3,
            Self::Up => 4,

        }
    }
}

#[derive(PartialEq, Clone, Eq)]
struct Beam{
    current: (usize, usize),
    direction: Direction
}

impl Beam {
    pub fn new(current: (usize, usize), direction: Direction) -> Beam{
        return Beam {
            current:current,
            direction: direction
        }
    }

    pub fn get_valid_coords_from_direction(&self, limit: i32) -> Option<(usize, usize)> {
        let temp_coords = (self.current.0 as i32, self.current.1 as i32);
        let coords = match self.direction {
            Direction::Down => (temp_coords.0+1, temp_coords.1),
            Direction::Up => (temp_coords.0-1, temp_coords.1),
            Direction::Right => (temp_coords.0, temp_coords.1+1),
            Direction::Left => (temp_coords.0, temp_coords.1-1),
        };

        if coords.1 >= limit || coords.0 >= limit || coords.1 < 0 || coords.0 < 0 {
            return None;
        } else {
            Some((coords.0 as usize, coords.1 as usize))
        }
    }

    fn reflect(&self, mirror: char) -> (Direction, Option<Direction>){
        match mirror {
            '|' => match self.direction {
                Direction::Down => (Direction::Down, None),
                Direction::Up => (Direction::Up, None),
                Direction::Left => (Direction::Down, Some(Direction::Up)),
                Direction::Right => (Direction::Down, Some(Direction::Up))
            },
            '-' => match self.direction {
                Direction::Down => (Direction::Left, Some(Direction::Right)),
                Direction::Up => (Direction::Left, Some(Direction::Right)),
                Direction::Left => (Direction::Left, None),
                Direction::Right => (Direction::Right, None)
            },
            '/' => match self.direction {
                Direction::Down => (Direction::Left, None),
                Direction::Up => (Direction::Right, None),
                Direction::Left => (Direction::Down, None),
                Direction::Right => (Direction::Up, None)
            },
            '\\' => match self.direction {
                Direction::Down => (Direction::Right, None),
                Direction::Up => (Direction::Left, None),
                Direction::Left => (Direction::Up, None),
                Direction::Right => (Direction::Down, None)
            }
            _ => match self.direction {
                Direction::Down => (Direction::Down, None),
                Direction::Up => (Direction::Up, None),
                Direction::Left => (Direction::Left, None),
                Direction::Right => (Direction::Right, None)
            }
        }
    }
}