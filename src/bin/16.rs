use std::fs::read_to_string;

fn main(){
    let input = read_to_string("input/input16.txt").unwrap();
}

/*
grid datatype = char, bool

split lines by newline
for each line {
    for each char {
        insert in grid
    }
}

make step function ->

takes current coordinate in grid, recursively starts new iterations of step if its split
marks each coordinate as energized

make directions enum

for each sign, define out-directions based on in-direciton
*/

enum Direction {
    Right,
    Left,
    Up,
    Down
}

fn reflect(mirror: char, direction : Direction) -> (Direction, Direction){
    match mirror {
        '|' => match direction {
            Direction::Down => (Direction::Down, Direction::Down),
            Direction::Up => (Direction::Up, Direction::Up),
            Direction::Left => (Direction::Down, Direction::Up),
            Direction::Right => (Direction::Down, Direction::Up)
        },
        '-' => match direction {
            Direction::Down => (Direction::Left, Direction::Right),
            Direction::Up => (Direction::Left, Direction::Right),
            Direction::Left => (Direction::Left, Direction::Left),
            Direction::Right => (Direction::Right, Direction::Right)
        },
        '/' => match direction {
            Direction::Down => (Direction::Left, Direction::Left),
            Direction::Up => (Direction::Right, Direction::Right),
            Direction::Left => (Direction::Down, Direction::Down),
            Direction::Right => (Direction::Up, Direction::Up)
        },
        '\\' => match direction {
            Direction::Down => (Direction::Right, Direction::Right),
            Direction::Up => (Direction::Left, Direction::Left),
            Direction::Left => (Direction::Up, Direction::Up),
            Direction::Right => (Direction::Down, Direction::Down)        
        }
        _ => match direction {
            Direction::Down => (Direction::Down, Direction::Down),
            Direction::Up => (Direction::Up, Direction::Up),
            Direction::Left => (Direction::Left, Direction::Left),
            Direction::Right => (Direction::Right, Direction::Right)

        }
    }
}