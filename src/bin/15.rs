use std::fs::read_to_string;

use hashbrown::HashMap;

fn main(){
    let input = read_to_string("input/input15.txt").unwrap();
    println!("Day 15: Part 1: {}, Part 2: {}", part1(&input), part2(&input));
}

fn part1(input : &str) -> u32{
    input.split(",").map(| line | {
        get_hash(line)
    }).sum()
}

fn part2(input : &str) -> u32{

    let mut map : HashMap<u32, Vec<(&str, u32)>> = HashMap::new();

    input.split(",").for_each(|line| {
        if line.contains("=") {
            let (label, focal_string) = line.split_once("=").unwrap();
            let label_hash = get_hash(label);
            let focal = focal_string.parse::<u32>().unwrap();
            if map.contains_key(&label_hash) {
                let mut list: Vec<(&str, u32)> = map.get(&label_hash).unwrap().to_vec();
                let mut inserted = false;
                for i in 0..list.len(){
                    if list[i].0 == label {
                        list[i] = (label, focal);
                        inserted = true;
                    }
                }
                if !inserted {
                    list.push((label, focal));
                }
                map.insert(label_hash, list);

            } else {
                map.insert(label_hash, vec![(label, focal)]);
            }
        } else {
            let label = line.split_once("-").unwrap().0;
            let label_hash = get_hash(label);
            if map.contains_key(&label_hash) {
                let mut list = map.get(&label_hash).unwrap().to_vec();
                let mut removed = false;
                for i in 0..list.len() {
                    if list[i].0 == label {
                        list.remove(i);
                        removed = true;
                        break;
                    }
                }
                if removed {
                    map.insert(label_hash, list);
                }

            }
        }
        
    });

    let mut value = 0;
    for key in map.keys(){
        let mut focusing_power = 0;
        let list = map.get(key).unwrap();
        for i in 0..list.len(){
            focusing_power += (key + 1) * (i as u32+1) * list[i].1
        }
        value += focusing_power
    }
    value
}

fn get_hash(input :&str) -> u32{
    let mut value = 0;
    input.chars().for_each(|c| {
        value += c as u32;
        value = value * 17;
        value = value % 256 
    });
    value
}

