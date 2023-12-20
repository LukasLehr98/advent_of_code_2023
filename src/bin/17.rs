use std::{fs::read_to_string, usize};

use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap};

fn main(){
    let input = read_to_string("input/input17.txt").unwrap();
    
    let part1 = part1(&input);
    let part2 = part2(&input);

    println!("Day 17: Part 1: {} Part 2: {}", part1.unwrap(), part2.unwrap());

}

fn parse_to_grid(input : &str) -> Vec<Vec<u32>>{
    input.lines().map(|line|{
        line.chars().map(|c| {
            c.to_digit(10).unwrap()
            }).collect()
    }).collect()
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
enum Direction {
    Up,
    Left,
    Down,
    Right,
}

impl Direction {
    fn next(&self) -> Direction {
        match self {
            Direction::Up => Direction::Left,
            Direction::Left => Direction::Down,
            Direction::Down => Direction::Right,
            Direction::Right => Direction::Up,
        }
    }
    fn prev(&self) -> Direction {
        match self {
            Direction::Up => Direction::Right,
            Direction::Left => Direction::Up,
            Direction::Down => Direction::Left,
            Direction::Right => Direction::Down,
        }
    }
}

fn step(y: usize, x: usize, dir: Direction) -> Option<(usize, usize)> {
    Some(match dir {
        Direction::Up => (y.checked_sub(1)?, x),
        Direction::Left => (y, x.checked_sub(1)?),
        Direction::Down => (y.checked_add(1)?, x),
        Direction::Right => (y, x.checked_add(1)?),
    })
}

fn solve<P, F, T>(data: &str, ok: P, next: F) -> Option<usize>
where
    P: Fn(usize) -> bool,
    F: Fn(Direction, usize) -> T,
    T: IntoIterator,
    <T as IntoIterator>::Item: Into<Direction>,
{
    let maze = parse_to_grid(&data);
    let mut queue: BinaryHeap<_> = [(Reverse(0), 0, 0, Direction::Right, 0)].into();
    let mut costs: HashMap<_, _> = [((0, 0, Direction::Right, 0), 0)].into();
    while let Some((Reverse(cost), y, x, direction @ direction0, distance)) = queue.pop() {
        let cost = cost + y + x;
        if y == maze.len() - 1 && x == maze[y].len() - 1 && ok(distance) {
            return Some(cost);
        }
        match costs.entry((y, x, direction, distance)) {
            std::collections::hash_map::Entry::Occupied(entry) => {
                if *entry.get() < cost {
                    continue;
                }
            }
            std::collections::hash_map::Entry::Vacant(_) => {
                #[cfg(debug_assertions)]
                panic!(
                    "missing state for ({}, {}, {}, {:?}, {})",
                    cost, y, x, direction, distance
                );
            }
        }
        for direction in next(direction, distance) {
            let direction = direction.into();
            let Some((y, x)) =
                step(y, x, direction).filter(|&(y, x)| y < maze.len() && x < maze[y].len())
            else {
                continue;
            };
            let distance = if direction == direction0 {
                distance + 1
            } else {
                1
            };
            let cost = cost + maze[y][x] as usize;
            match costs.entry((y, x, direction, distance)) {
                std::collections::hash_map::Entry::Occupied(mut entry) => {
                    if *entry.get() <= cost {
                        continue;
                    }
                    entry.insert(cost);
                }
                std::collections::hash_map::Entry::Vacant(entry) => {
                    entry.insert(cost);
                }
            }
            queue.push((Reverse(cost - y - x), y, x, direction, distance));
        }
    }
    None
}

pub fn part1(data: &str) -> Option<usize> {
    solve(
        data,
        |_| true,
        |direction, distance| {
            if distance < 3 {
                vec![direction.prev(), direction.next(), direction]
            } else {
                vec![direction.prev(), direction.next()]
            }
        },
    )
}

pub fn part2(data: &str) -> Option<usize> {
    solve(
        data,
        |distance| distance >= 4,
        |direction, distance| {
            if distance < 4 {
                vec![direction]
            } else if distance < 10 {
                vec![direction.prev(), direction.next(), direction]
            } else {
                vec![direction.prev(), direction.next()]
            }
        },
    )
}
