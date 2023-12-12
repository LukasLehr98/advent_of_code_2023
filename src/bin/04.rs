use std::fs::read_to_string;


pub fn main() {
    let binding = read_to_string("input/input4.txt").unwrap();
    let input : Vec<&str>= binding.lines().collect();
    let mut sum = 0;
    let mut copies = vec![1; input.len()];

    for (i, line) in input.into_iter().enumerate() {
        let (winning, mine) = line.split_once(" | ").unwrap();
        let winning = winning.split(' ').filter(|x| !x.is_empty()).collect::<Vec<&str>>();
        let mine = mine.split(' ').filter(|x| !x.is_empty()).collect::<Vec<&str>>();

        let mut score = 0;
        let mut count = 0;

        for num in mine {
            if winning.contains(&num) {
                count += 1;
                if score == 0 {
                    score = 1;
                } else {
                    score *= 2;
                }
            }
        }
        for j in (i + 1)..(i + count + 1) {
            copies[j] += copies[i];
        }
        if score > 0 {
            sum += score;
        }
    }

    let copy_count: i32 = copies.iter().map(|x| *x).sum();
    println!("Day 4: Part 1: {}, Part 2: {}", sum, copy_count);
}
