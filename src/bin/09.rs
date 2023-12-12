use std::fs::read_to_string;

fn main() {
    let input = read_to_string("input/input9.txt").unwrap();
    println!("Day 9: Part 1: {}, Part 2: {}", part1(&input), part2(&input));
}

fn part1(input: &str) -> i64 {
    input.split("\n").map(|line| {
        PatternList::new(line.split_ascii_whitespace().map(|num| num.parse::<i64>().unwrap()).collect::<Vec<i64>>()).get_next_value(0)
    }).sum()
}

fn part2(input: &str) -> i64 {
    input.split("\n").map(|line| {
        PatternList::new(line.split_ascii_whitespace().map(|num| num.parse::<i64>().unwrap()).rev().collect::<Vec<i64>>()).get_next_value(0)
    }).sum()
}

struct PatternList {
    numbers: Vec<i64>,
    diffs: Option<Box<PatternList>>,
}

impl PatternList {
    pub fn new(list: Vec<i64>) -> PatternList{
        let mut diffs :Vec<i64> = Vec::new();
        let mut contains_non_zero = false;
        for (i, num) in list.iter().enumerate() {            
            if i < list.len()-1 {
                if !contains_non_zero {
                    if num != &0 {
                        contains_non_zero = true;
                    }
                }
                diffs.push(list[i+1] - num);
            }
        }

        if contains_non_zero {
            return PatternList {
                numbers: list,
                diffs: Some(Box::new(PatternList::new(diffs))),
            };
        } else {
            return PatternList {
                numbers: diffs,
                diffs: None
            }
        }
    }

    pub fn get_next_value(&self, acc : i64) -> i64 {
        match &self.diffs {
            Some(d) => d.get_next_value(acc +*self.numbers.last().unwrap()),
            None => acc,
        }
    }
 }