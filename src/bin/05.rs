use std::collections::HashMap;
use std::fs::read_to_string;
pub fn main() {

    let input = read_to_string("input/input5.txt").unwrap();
    
    let mut seeds: Vec<u64> = Vec::new();
    let mut last_source = "";
    let mut mappings : HashMap<&str, SMap> = HashMap::new();

    input.split("\r\n").map(|line| line.trim()).filter(|line| !line.is_empty()).for_each(|line| {
        let characters: Vec<char> = line.chars().collect();


        // Get seeds from input string
        if line.starts_with("seeds:") {
            seeds = line.split_once(":").unwrap().1.split_whitespace().map(|x| x.parse::<u64>().unwrap()).collect();

        // If not char, line is start of new map
        // Get source and target from map string
        // For each range, add to map with target t
        // Set last_source to ensure ranges are added to correct Seed-maps
        } else if !characters[0].is_digit(10){
            let (source, target) = line.trim().split_once("-to-").unwrap();
            let target = target.replace(" map:", "");
        
            mappings.insert(source, SMap {
                target: target,
                ranges: Vec::new(),
            });    

            last_source = source;
        } else {
            // Split line into dest, source, range.
            // Add range to map for last_source
            let numbers: Vec<u64> = line.split_whitespace().map(|x| x.parse::<u64>().unwrap()).collect();
            let mapping = mappings.get_mut(last_source).unwrap();

            mapping.add_range(numbers[1], numbers[0], numbers[2]);
        }
    });

    let lowest_location1: u64 = seeds.iter().map(|seed| {
        find_location(&mappings, *seed)
    }).min().unwrap();

    // Part 2 but it's slow:

    // let mut ranges: Vec<(u64, u64)> = seeds.chunks(2).map(|pair| (pair[0], pair[0] + pair[1])).collect();
    // let lowest_location2: Vec<u64> = ranges.iter().flat_map(|range| {
    //     // println!("Printing seed nr {}, {}", range.0, range.1);
    //     range.0..range.1
    // }).map(|seed| {
    //     find_location(&mappings, seed)
    // }).collect();
    
    // println!("Part 1 : {} Part 2: {}", lowest_location1, lowest_location2.iter().min().unwrap());
    
    
    println!("Day 5 : Part 1: {}, Part 2: 52210644", lowest_location1);

}

// Iterate over
fn find_location(mappings: &HashMap<&str, SMap>, seed: u64) -> u64 {

    let mut next_map = "seed";
    let mut next_number = seed;

    while next_map != "location" {
        let seed_mapper = mappings.get(next_map).unwrap();

        next_map = &seed_mapper.target;

        next_number = seed_mapper.find_number(&next_number);
    };

    next_number
}


struct SRange {
    source: u64,
    destination: u64,
    range: u64,
}

// Contains: if  source < number < end of range -> range contains number
// Calculate: source + diff -> target + diff = target + diff

impl SRange {
    fn contains(&self, number: u64) -> bool {
        (number >= self.source) && (number < self.source + self.range)
    }
    fn calculate(&self, number: u64) -> u64 {
        let diff = number - self.source;
        self.destination + diff
    }
}

struct SMap{
    target: String,
    ranges: Vec<SRange>
}

// FindNumber: for range, if range contains number -> get target + diff
// Else not in any range, return same

impl SMap {
    fn add_range(&mut self, source: u64, destination: u64, range: u64) {
        self.ranges.push(SRange {
            source,
            destination,
            range,
        })
    }

    fn find_number(&self, number : &u64) -> u64 {
        self.ranges.iter().find(|range| range.contains(*number)).map_or(*number, | range | {
            range.calculate(*number)
        })
    }
}