use std::fs::read_to_string;
use hashbrown::HashMap;

fn main(){
    let input = read_to_string("input/input19.txt").unwrap();

    let (parts, map) = parse(&input);

    let part_range = PartRange {
        x: NumRange {min: 1, max: 4000},
        m: NumRange {min: 1, max: 4000},
        a: NumRange {min: 1, max: 4000},
        s: NumRange {min: 1, max: 4000},
    };

    let part2 = step(&map, part_range, "in");

    println!("Day 19: Part 1: {} Part 2: {}", part1(parts, map), part2);
}

fn part1(parts: Vec<Part>, workflows: HashMap<&str, Vec<Instruction>>) -> u32{
    let mut accepted: Vec<Part> = Vec::new();
    let mut declined: Vec<Part> = Vec::new();

    let sum = parts.into_iter().map(|part| {
        let mut local_key: String = String::from("in");
        while local_key != "A" && local_key != "R"{
            
            let flow = workflows.get(local_key.clone().as_str()).unwrap().clone();
            for i in 0..flow.len(){
                let instruction = flow[i].clone();
                match instruction.condition {
                    Some(comparer) => {
                        if comparer.compare(&part) {
                            local_key = instruction.destination.clone();
                            break;
                        }
                    },
                    None => {
                        local_key = instruction.destination.clone();
                        break;
                    }
                }
            }
        }

        if local_key == "A" {
            accepted.push(part);
            part.get_part_total()
        } else {

            declined.push(part);
            0
        }
    }).sum();

    sum
}

fn step(workflows: &HashMap<&str, Vec<Instruction>>, mut part_ranges: PartRange, current: &str) -> u64{
    let mut cur: String = current.to_string();
    let mut acc = 0;

    while cur != "A" && cur != "R" {
        let flow = workflows.get(cur.clone().as_str()).unwrap().clone();
        for i in 0..flow.len(){
            let instruction = flow[i].clone();
            match instruction.condition {
                Some(comparer) => {
                    let part_range = part_ranges.get_part_from_char(comparer.category);
                    if part_range.contains(comparer.num.into()){
                        match comparer.comparer {
                            '<' => {
                                let new_part_ranges: PartRange = part_ranges.clone()
                                    .set_part_range_with_char(comparer.category, 
                                        NumRange {min: part_range.min.clone(), max: (comparer.num -1).into()});

                                part_ranges = part_ranges.set_part_range_with_char(comparer.category, 
                                    NumRange {min: comparer.num.into(), max: part_range.max});
                                        
                                acc += step(&workflows, new_part_ranges, &instruction.destination)
                            } 
                            '>' => {
                                let new_part_ranges: PartRange = part_ranges.clone()
                                    .set_part_range_with_char(comparer.category, 
                                        NumRange {min: (comparer.num+1).into() , max: part_range.max.clone()});

                                part_ranges = part_ranges.set_part_range_with_char(comparer.category, 
                                    NumRange {min: part_range.min, max: comparer.num.into()});

                                acc += step(&workflows, new_part_ranges, &instruction.destination)
                            }
                            _ => {}
                        }
                    } 
                },
                None => {
                    cur = instruction.destination;
                    break;
                }
            }
        }
    }
    if cur == "R" {
        acc
    } else {
        acc + part_ranges.get_product()
    }
}

fn parse(input : &str) -> (Vec<Part>, HashMap<&str, Vec<Instruction>>){
    let (workflow_list, parts) = input.split_once("\r\n\r\n").unwrap();

    let mut workflow_map: HashMap<&str, Vec<Instruction>> = HashMap::new();
    workflow_list.lines().for_each(|line| {
        let (name, instructions) = line.split_once("{").unwrap();
        let instructions = &instructions.replace("}", "");

        let flow = instructions.split(",").map(| instruction | {
            if instruction.contains(":"){
                let (comparison, dest) = instruction.split_once(":").unwrap();
                let (category_comparer,num) = comparison.split_at(2);
                let (category, comparer) = category_comparer.split_at(1);
                Instruction {
                    destination: dest.to_string(),
                        condition : Some(Comparison {
                            category: category.parse::<char>().unwrap(),
                            comparer: comparer.parse::<char>().unwrap(),
                            num : num.parse::<u32>().unwrap()
                        })
                    }
            } else {
                Instruction {
                    destination : instruction.to_string(),
                    condition: None
                }
            }
        }).collect::<Vec<Instruction>>();
        
        workflow_map.insert(name, flow);
    });


    let parts : Vec<Part> = parts.lines().map(|part_string| {
        let mut part = Part::new();
        let part_string = part_string.replace("{", "").replace("}", "");
        part_string.split(",").for_each(|attribute| {
            let (part_char, num_str) = attribute.split_once("=").unwrap();
            let num = num_str.parse::<u32>().unwrap();
            match part_char {
                "x" => part.x = num,
                "m" => part.m = num,
                "a" => part.a = num,
                "s" => part.s = num,
                _ => {}
            }
        });
        part
    }).collect();

    (parts, workflow_map)
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Part {
    x : u32,
    m : u32,
    a : u32,
    s : u32
}

impl Part{
    pub fn new() -> Part{
        Part {
            x: 0,
            m: 0,
            a: 0,
            s: 0,
        }
    }

    pub fn get_part_from_char(&self, c: char) -> u32{
        match c {
            'x' => { self.x },
            'm' => { self.m },
            'a' => { self.a },
            's' => { self.s },
            _ => 0
        }
    }

    pub fn get_part_total(self) -> u32{
        self.x + self.m + self.a + self.s
    }
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Instruction {
    destination: String,
    condition: Option<Comparison>
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Comparison{
    category: char,
    comparer: char,
    num: u32
}

impl Comparison {
    pub fn compare(self, part: &Part) -> bool{
        match self.comparer {
            c if c == '>' => {
                let part_num = part.get_part_from_char(self.category);
                part_num > self.num
            }
            c if c == '<' => {
                let part_num = part.get_part_from_char(self.category);
                part_num < self.num
            }
            _ => false
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct NumRange{
    min : u64,
    max: u64,
}

impl NumRange{
    pub fn contains (&self, num: u64) -> bool{
        if num >= self.max ||num <= self.min {
            return false
        } else {
            return true
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct PartRange{
    x: NumRange,
    m: NumRange,
    a: NumRange,
    s: NumRange
}
impl PartRange{
    pub fn get_part_from_char(&self, c: char) -> NumRange{
        match c {
            'x' => { self.x },
            'm' => { self.m },
            'a' => { self.a },
            's' => { self.s },
            _ => NumRange{min: 0, max: 0}
        }
    }

    pub fn set_part_range_with_char(mut self, c :char, range: NumRange) -> PartRange{
        match c {
            'x' => { self.x = range },
            'm' => { self.m = range },
            'a' => { self.a = range },
            's' => { self.s = range },
            _ => {}
        }
        return self
    }

    pub fn get_product(self) -> u64{
        (self.x.max - self.x.min +1) * (self.m.max -self.m.min +1) * (self.a.max - self.a.min +1) * (self.s.max - self.s.min +1) 
    }
}