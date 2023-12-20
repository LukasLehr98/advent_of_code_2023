use std::{fs::read_to_string, usize};

use itertools::Itertools;

fn main(){
    let input = read_to_string("input/input17.txt").unwrap();
    
    let part1 = a_star(&input, (0, 0), (0 , 0));

    println!("{}", part1);
}

fn a_star(input: &str, start : (usize, usize), mut end : (usize, usize)) -> u32{
    let grid = parse_to_grid(input);
    end = (grid.len()-1, grid[0].len()-1);
    let mut open : Vec<Node> = vec![Node { parent : None, cost: 0, estimate: u32::MAX, pos: start, count: 0}];
    let mut closed : Vec<Node> = Vec::new();
    let mut optimal: Option<Node> = None;

    while !open.is_empty(){
        
        open.sort_by(|a, b| {
            (a.cost + manhattan(a.pos, end)).cmp(&(b.cost + manhattan(b.pos, end)))
        });
        
        let current = open.remove(0);
        if current.pos == (1, 12) {
            println!()
        }

        if current.count > 3 {
            continue;
        }

        closed.push(current.clone());

        if current.pos == end {
            optimal = Some(current.clone());
            break;
        }

        for pos in get_valid_children(&current.pos, grid.len()){
           
            if current.parent.clone().is_some_and(|parent| parent.pos == pos){
                continue;
            }

            let mut node = Node {
                parent : Some(Box::new(current.clone())),
                cost: current.cost + grid[pos.0][pos.1],
                estimate: manhattan(pos, end),
                pos: pos,
                count: 0,
            };
            node.count = get_straight_line_count(&node, node.pos);

             if node.count > 3 {
                continue;
            }

            closed.clone().into_iter().filter(|closed_node| {
                closed_node.count == node.count && closed_node.pos == node.pos
            }).for_each(|mut n| { if n.cost > node.cost { n.cost = node.cost}});

            let filtered = open.clone().into_iter().filter(|open_node| {
                open_node.count == node.count && &open_node.pos == &node.pos}).collect_vec();

            if filtered.len() != 0 {
                let found = filtered.first().unwrap();
                if found.cost < node.cost {
                    continue
                }
            }

            open.push(node);
        }
    }

    let mut path : Vec<(usize, usize)> = Vec::new();

    let mut cost = 0;
    
    if optimal == None {
        0
    } else {
        cost = optimal.clone().unwrap().cost;
        
        while optimal.clone().is_some(){
            let temp = optimal.unwrap().clone();
            path.push(temp.pos);
            if temp.parent.is_some() {
                optimal =  Some(*temp.parent.unwrap());
            } else {
                break;
            }
        }

        path.reverse();

        for i in &path {
            println!("{}{}", i.0, i.1)
        }

        for i in 0..grid.len(){
            for j in 0..grid[0].len(){
                if path.contains(&(i, j)) {
                    print!("#")
                } else {
                    print!(".")
                }
            }
            println!()
        }
        cost
    }
    
} 

fn get_valid_children(pos : &(usize, usize), limit : usize) -> Vec<(usize, usize)>{
    let neighbors = vec![
            (pos.0.checked_sub(1), Some(pos.1)),
            (Some(pos.1), pos.1.checked_sub(1)),
            (Some(pos.0+1), Some(pos.1)),
            (Some(pos.0), Some(pos.1+1))
        ];

    neighbors.into_iter().filter(|pos| {
        pos.0.is_some_and(|x| x < limit) && pos.1.is_some_and(|x| x < limit)
    }).map(|pos| {
        (pos.0.unwrap(), pos.1.unwrap())
    }).collect_vec()
}

fn parse_to_grid(input : &str) -> Vec<Vec<u32>>{
    input.lines().map(|line|{
        line.chars().map(|c| {
            c.to_digit(10).unwrap()
            }).collect()
    }).collect()
}

fn manhattan(coords: (usize, usize), target: (usize, usize)) -> u32{
    (coords.0.abs_diff(target.0) + coords.1.abs_diff(target.1)) as u32
}

fn get_straight_line_count(mut node : &Node, pos: (usize, usize)) -> u32{
    let mut straight_line : (bool, bool) = (true, true);
    let mut straight_line_count : u32 = 0;
    let mut node = Some(Box::new(node.clone()));

    while node.is_some(){
        let loop_node = *node.unwrap();
        if loop_node.pos == (0, 0) { break; }

        if loop_node.pos.0 != pos.0 {
            straight_line.0 = false
        }
        if loop_node.pos.1 != pos.1 {
            straight_line.1 = false
        }

        if straight_line == (false, false) { break; }

        node = loop_node.parent.clone();

        if node.is_some(){
            if node.clone().unwrap().pos == (0, 0) { continue; }
        }
        straight_line_count += 1;

    }

    straight_line_count
}

#[derive(PartialEq, Clone, Eq)]
struct Node{
    parent: Option<Box<Node>>,
    cost : u32,
    estimate: u32,
    pos: (usize, usize),
    count: u32
}