use std::{fs::read_to_string, collections::HashMap};

fn main() {
    let input = read_to_string("input/input7.txt").unwrap();

    let part1: i64 = solve(&input, 11);
    let part2: i64 = solve(&input, 1);
    
    println!("Day 7: Part 1 : {}, Part 2 : {}", part1, part2);
}

fn solve(file : &str, joker: i32) -> i64{
    let mut camel  = file.split("\n").map(|line| {
        let (card_str, wager_str) = line.split_once(" ").unwrap();

        let cards: Vec<i32> = card_str.chars().map(|char: char|{
         match char {
            x if x == 'J' => joker,
            x if x == 'A' => 14, 
            x if x == 'K' => 13,
            x if x == 'Q' => 12,
            x if x == 'T' => 10,
            x => x.to_digit(10).unwrap().try_into().unwrap()
         }
        }).collect::<Vec<i32>>();

        let wager = wager_str.trim().parse::<i64>().unwrap();

        let strength = get_strength_of_hand(&cards);

        CamelCards {
            cards, 
            wager, 
            strength
        }
    }).collect::<Vec<CamelCards>>();

    camel.sort_by(|a, b| {
        if a.strength == b.strength {
            for i in 0..a.cards.len(){
                if a.cards[i] == b.cards[i] {
                    continue;
                } else {
                    return a.cards[i].cmp(&b.cards[i]);
                }
            }
        }
        a.strength.cmp(&b.strength)
    });

    for i in 0..camel.len() {
        // println!("Number: {}, Wager: {}, Strength:{}", i + 1, camel[i].wager, camel[i].strength);
        let integer : i64 = i.try_into().unwrap();
        camel[i].wager =  camel[i].wager * (integer + 1);
    }

    camel.iter().map(|x| x.wager ).sum()
}


fn get_strength_of_hand(hand: &Vec<i32>) -> i32{
    let mut map: HashMap<i32, i32> = HashMap::new();

    let mut jokers : i32 = 0;

    for i in hand {
        if *i == 1 { 
            jokers += 1;
            continue;
        }
        match map.get(i){
            Some(x) => map.insert(*i, x + 1),

            None => map.insert(*i, 1),
         };
    }

    let mut best_set: i32 = 0;
    let mut second_set: i32 = 0;

    for i in map.keys(){
        if map.get(i).unwrap() >= &best_set { 

            second_set = best_set;
            best_set = *map.get(i).unwrap();
        }
        else if map.get(i).unwrap() > &second_set { 
            second_set = *map.get(i).unwrap()
        }
    }

    (best_set + jokers) * 2 + second_set

}

struct CamelCards {
    cards: Vec<i32>,
    wager: i64,
    strength: i32,
}
