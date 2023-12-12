use std::fs::read_to_string;

fn main() {

    let input = read_to_string("input/input6.txt").unwrap();
    let (times_string, distances_string) = input.split_once("\n").unwrap();

    let times : Vec<i64> = times_string.split_once(':').unwrap().1.trim().split(' ').filter(|x| !x.is_empty()).map(|x| x.trim().parse::<i64>().unwrap()).collect();
    let distances : Vec<i64> = distances_string.split_once(':').unwrap().1.trim().split(' ').filter(|x| !x.is_empty()).map(|x| x.trim().parse::<i64>().unwrap()).collect();
    
    let part1 = get_wins(times, distances);

    let part2_times = times_string.chars().filter(|x| x.is_digit(10)).collect::<String>().parse::<i64>().unwrap();
    let part2_distances = distances_string.chars().filter(|x| x.is_digit(10)).collect::<String>().parse::<i64>().unwrap();
    let part2 = get_wins(vec![part2_times], vec![part2_distances]);

    println!("Day 6: Part1: {}, Part2: {}", part1, part2);
}

fn get_wins(times: Vec<i64>, distances: Vec<i64>) -> i64 {
    let mut wins_multiplied = 1;

    for i in 0..times.len(){
        let mut wins = 0;

        for j in 0..times[i] {
            
            let distance_raced = (times[i] - j) * j;

            if distance_raced > distances[i] {
                wins += 1;
            }
        }
        wins_multiplied = wins_multiplied * wins;
    }

    wins_multiplied
}


