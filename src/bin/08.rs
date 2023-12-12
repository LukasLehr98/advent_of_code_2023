use std::fs::read_to_string;
use std::collections::HashMap;
pub fn main(){
    let input = read_to_string("input/input8.txt").unwrap();
    let part1 = part1(input.as_str());
    let part2 = part2(input.as_str());

    println!("Day 8: Part 1: {}  Part 2: {}", part1, part2);

}

fn parse_input(input: &str) -> (Vec<char>, HashMap<String, (String, String)>) {
    let (steps, map) = input.split_once("\n").unwrap();

    let nodes: HashMap<String, (String, String)> = map
        .lines()
        .map(|line| {
            let t = line.split_once(" = ").unwrap();
            let binding = t.1.chars().filter(| c | c != &'(' && c != &')').collect::<String>();
            let (b, c) = binding.split_once(", ").unwrap();
            (t.0.to_owned(), b.to_owned(), c.to_owned())
        })
        .map(|(a, b, c)| (a.to_owned(), (b.to_owned(), c.to_owned())))
        .collect::<HashMap<_, _>>();

    (steps.trim().chars().collect::<Vec<char>>(), nodes)
}

fn part1(input: &str) -> usize {
    let (steps, nodes) = parse_input(input);

    let mut curr_node = "AAA";
    let mut curr_steps = 0;

    loop {
        if curr_node == "ZZZ" {
            break;
        }

        let side = steps[curr_steps % steps.len()];
        curr_node = if side == 'L' {
            nodes.get(curr_node).unwrap().0.as_str()
        } else {
            nodes.get(curr_node).unwrap().1.as_str()
        };
        curr_steps += 1;
    }

    curr_steps
}

fn part2_iter(input: &str, start: &str) -> usize {
    let (steps, nodes) = parse_input(input);

    let mut curr_node = start;
    let mut curr_steps = 0;

    loop {
        if curr_node.ends_with("Z"){
            break;
        }

        let side = steps[curr_steps % steps.len()];
        curr_node = if side == 'L' {
            nodes.get(curr_node).unwrap().0.as_str()
        } else {
            nodes.get(curr_node).unwrap().1.as_str()
        };
        curr_steps += 1;
    }

    curr_steps
}

fn lcm_list(numbers: Vec<usize>) -> usize {
    numbers.into_iter().reduce(|a, b| {
        lcm(a, b)
    }).unwrap()
}

fn lcm(first: usize, second: usize) -> usize {
    first * second / gcd(first, second)
}

fn gcd(first: usize, second: usize) -> usize {
    let mut max = first;
    let mut min = second;
    if min > max {
        let val = max;
        max = min;
        min = val;
    }

    loop {
        let res = max % min;
        if res == 0 {
            return min;
        }

        max = min;
        min = res;
    }
}					

fn part2(input: &str) -> usize{
    let mut answers : Vec<usize> = Vec::new();

    let (_steps, nodes) = parse_input(&input);

    for i in nodes.keys().filter(|x| x.ends_with("A")) {
        answers.push(part2_iter(input, &i));
    }

    lcm_list(answers)
}
