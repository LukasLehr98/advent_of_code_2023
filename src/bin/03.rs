use std::fs::read_to_string;

pub fn main() {
    // let lines: Lines<BufReader<File>> = BufReader::new(File::open("./input3.txt").unwrap()).lines();
    let input = read_to_string("input/input3.txt").unwrap();

    let char_vectors: Vec<Vec<char>> = input.split("\n").map(|line| line.chars().collect() ).collect();
    let mut acc: i32 = 0;
    let mut gearacc: i32 = 0;

    for (i, line) in char_vectors.iter().enumerate() {
        let temp = get_valid_numbers(char_vectors.get(i.saturating_sub(1)).or(None), line.clone(), char_vectors.get(i + 1).or(None));
        acc += temp.0;
        gearacc += temp.1;
    }

    println!("Day 3: Part 1: {} Part 2: {}", acc, gearacc);
}

fn get_valid_numbers(prev: Option<&Vec<char>>, current : Vec<char>, next: Option<&Vec<char>>) -> (i32, i32){
    let mut acc = 0;
    let mut gearAcc = 0;
    let mut num :String = String::from("");
    let mut valid : bool = false;
    
    for (i, char) in current.iter().enumerate(){
        match char {
            dot if *dot == '.' => {
                if !(num.len() == 0) && valid{
                    acc += num.chars().collect::<String>().parse::<i32>().unwrap();
                }
                valid = false;
                num = String::from("");
            },
            digit if digit.is_digit(10) => {
                num.push(*digit);
                if !valid{
                    let start = match i {
                        x if x == 0 => 0,
                        x => x - 1,
                    };
                    let end = match i < current.len() - 1 {
                        true => i + 1,
                        false => i,
                    };
                    match prev {
                        Some(ref p) => for val in &p[start..=end] {
                            if !(*val == '.') && !(val.is_digit(10)) {
                                valid = true;
                            }
                        }
                        None => {}
                    }
                    match next {
                        Some(ref n) => for val in &n[start..=end] {
                                if !(*val == '.') && !(val.is_digit(10)) {
                                    valid = true;
                                }
                        }
                        None => {}
                    }
                }
            },
            _special => {
                if !(num.len() == 0){
                    acc += num.chars().collect::<String>().parse::<i32>().unwrap();
                    num = String::from("");
                }
                valid = true;
                let mut numbers: Vec<i32> = get_surrounding_numbers(Some(current.clone()), i);
                let prev_numbers : Vec<i32> = get_surrounding_numbers(prev.cloned(), i);
                let next_numbers : Vec<i32> = get_surrounding_numbers(next.cloned(), i);

                numbers.extend(prev_numbers);
                numbers.extend(next_numbers);

                if numbers.len() == 2 {
                    let numbers = (numbers.get(0).unwrap(), numbers.get(1).unwrap());
                    gearAcc += numbers.0 * numbers.1
                }
            }
        }
    }
    if !num.is_empty() && valid{
        acc += num.chars().collect::<String>().parse::<i32>().unwrap();
    }
    (acc, gearAcc)
}

fn get_surrounding_numbers(line : Option<Vec<char>>, index : usize) -> Vec<i32>{
    let mut current : String =  String::from("");
    let mut acc : Vec<i32> = Vec::new();
    let mut valid = false;

    if line == None{
        return acc 
    }

    for (i, char) in line.unwrap().iter().enumerate(){
        match char {
            c if c.is_digit(10) => {
                current.push(*c);
                if !valid{
                    let min = i.checked_sub(current.len());
                    valid = (min.unwrap_or(0) <= index) && (i + 1 >= index)
                }
            }
            _c => {
                if valid{
                    acc.push(current.chars().collect::<String>().parse::<i32>().unwrap())
                }
                valid = false;
                current = String::from("");
            }
        }
    }
    if valid && (current.len() != 0) {
        acc.push(current.chars().collect::<String>().parse::<i32>().unwrap())
    }
    acc
}