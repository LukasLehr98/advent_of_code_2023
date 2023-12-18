use hashbrown::HashMap;
use itertools::Itertools;
use std::fs::read_to_string;

fn main() {
    let mut memo = HashMap::new();
    let input = read_to_string("input/input12.txt").unwrap();
    let results = input.lines().map(|line| {
        let (springs, rest) = line.split_once(" ").unwrap();
        let sizes = rest.split(",").map(|size| {
            size.parse::<usize>().unwrap()
        }).collect::<Vec<_>>();

        let p1 = count_ways(&mut memo, springs.as_bytes(), None, &sizes);
        memo.clear();

        let new_springs = (0..5).map(|_| springs).join("?");
        let new_sizes = (0..5).flat_map(|_| &sizes).copied().collect::<Vec<_>>();
        memo.clear();
        let p2 = count_ways(&mut memo, new_springs.as_bytes(), None, &new_sizes);
        (p1, p2)
    }).fold((0, 0), |(p1, p2), (a, b)| (p1 + a, p2 + b));

    println!("Day 12 Part 1: {}, Part 2: {}", results.0, results.1);
}

fn count_ways(memo: &mut HashMap<(usize, usize, usize), usize>, springs: &[u8], within: Option<usize>, remaining: &[usize]) -> usize {
    if springs.is_empty() {
        return match (within, remaining.len()) {
            (None, 0) => 1,
            (Some(x), 1) if x == remaining[0] => 1,
            _ => 0,
        };
    }
    if within.is_some() && remaining.is_empty() {
        return 0;
    }

    let key = (springs.len(), within.unwrap_or(0), remaining.len());
    if let Some(&result) = memo.get(&key) {
        return result;
    }

    let ways = match (springs[0], within) {
        (b'.', Some(x)) if x != remaining[0] => 0,
        (b'.', Some(_)) => count_ways(memo, &springs[1..], None, &remaining[1..]),
        (b'.', None) => count_ways(memo, &springs[1..], None, remaining),
        (b'#', Some(_)) => count_ways(memo, &springs[1..], within.map(|x| x + 1), remaining),
        (b'#', None) => count_ways(memo, &springs[1..], Some(1), remaining),
        (b'?', Some(x)) => {
            let mut ans = count_ways(memo, &springs[1..], within.map(|x| x + 1), remaining);
            if x == remaining[0] {
                ans += count_ways(memo, &springs[1..], None, &remaining[1..])
            }
            ans
        }
        (b'?', None) => count_ways(memo, &springs[1..], Some(1), remaining)
            + count_ways(memo, &springs[1..], None, remaining),
        _ => unreachable!(),
    };
    memo.insert(key, ways);
    ways
}
