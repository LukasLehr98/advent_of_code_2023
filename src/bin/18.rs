use itertools::Itertools;
use std::fs::read_to_string;

fn main(){
    let input = read_to_string("input/input18.txt").unwrap();

    println!("Day 18: Part 1: {} Part 2: {}", part_1(&input), part_2(&input))
}

    fn part_1(input: &str) -> isize {
        1 + input
            .trim()
            .lines()
            .fold((0isize, 0isize), |(area, lat), line| {
                let (dir, dist, _) = line.split(' ').collect_tuple().unwrap();
                let dist = dist.parse::<isize>().unwrap();
                match dir {
                    "R" => (area - dist * (lat), lat),
                    "L" => (area + dist * (lat + 1), lat),
                    "U" => (area, lat - dist),
                    "D" => (area + dist, lat + dist),
                    _ => (area, lat),
                }
            })
            .0
    }

    fn part_2(input: &str) -> isize {
        1 + input
            .trim()
            .lines()
            .fold((0isize, 0isize), |(area, lat), line| {
                let (_, _, hex) = line.split(' ').collect_tuple().unwrap();
                assert_eq!(hex.len(), 9);
                let dist = isize::from_str_radix(&hex[2..7], 16).unwrap();
                let dir = isize::from_str_radix(&hex[7..8], 16).unwrap();
                match dir {
                    0 => (area - dist * (lat), lat),
                    1 => (area + dist, lat + dist),
                    2 => (area + dist * (lat + 1), lat),
                    3 => (area, lat - dist),
                    _ => (area, lat),
                }
            })
            .0
    }
