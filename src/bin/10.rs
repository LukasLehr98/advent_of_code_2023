use std::fs::read_to_string;

fn main() {
    let input = read_to_string("input/input10.txt").unwrap();

    let temp = solve(input.as_str());


    println!("Day 10: Part1 {}, Part2 {}", temp.0, temp.1);

}

fn solve(input : &str) -> (i32, i32){
    let mut map : Vec<Vec<Tile>> = Vec::new();
    let mut start : (usize, usize) = (0,0);
    for (x, line) in input.split("\n").into_iter().enumerate(){
        let mut row : Vec<Tile> = Vec::new();
        for (y, c) in line.chars().filter(|x| !x.is_whitespace()).into_iter().enumerate(){
            if c == 'S' {
                start = (x, y);
                // println!("Found start!: {}{}", start.0, start.1);
            }
            row.push(Tile {
                location : (x, y),
                sign : c,
                links : get_links(&c, (x, y)),
            });
        }
        map.push(row)
    }

    let y_max = &map[0].len() - 1;
    let x_max = &map.len() - 1;
    let current = &map[start.0][start.1];

    let steps: Vec<(i32, Vec<(usize, usize)>)> = current.links.clone().into_iter().map(| link |{
        let mut path : Vec<(usize, usize)> = Vec::new();
        let mut steps = 0;
        let mut prev = start.clone();
        let mut cur = &map[link.0][link.1];

        while cur.links.len() == 2 && cur.location != start{
            steps += 1;
            let new = cur.links.clone().into_iter().filter(|x| {
                x != &prev
            }).last().unwrap();

            path.push(new.clone());
            
            if new.0 > x_max || new.1 > y_max {
                continue;
            }

            prev = cur.location;

            
            cur = &map[new.0][new.1]
        }
        (steps, path)
    }).collect::<Vec<(i32, Vec<(usize, usize)>)>>();


    let mut max_steps = 0;
    let mut path : Vec<(usize, usize)> = Vec::new();
    for i in steps {
        if max_steps <= i.0 {
            max_steps = i.0;
            path = i.1;
        }
    }

    let mut enclosed = 0;
    
    for i in map{
        let mut is_inside : bool = false;
        for mut j in i {
            if j.sign == 'S' { j.sign = '7' }

            if path.contains(&j.location) {
                if "7|F".contains(j.sign) { 
                    is_inside = !is_inside;
                }
            } else {
                if is_inside == true {
                    enclosed += 1
                }
            }
        }
    }

    if max_steps % 2 != 0 {
        return ((max_steps + 1) / 2, enclosed)
    } else {
        return (max_steps / 2, enclosed)
    }


}


pub struct Tile {
    location : (usize, usize),
    sign : char,
    links : Vec<(usize, usize)>,
}

fn get_links(c : &char, location : (usize, usize)) -> Vec<(usize, usize)>{

    let mut local_c = c.clone();

    if location.0 == 0 {
        if "|JL".contains(local_c) {
            local_c = '.'
        }  
    } 
    if location.1 == 0 {
        if "J-7".contains(local_c) {
            local_c = '.'
        }  
    } 

    if local_c == 'S' {
        let mut temp_vec: Vec<(usize, usize)> = Vec::new();
        match location.0.checked_sub(1) {
            Some(_) => { temp_vec.push((location.0-1, location.1)) }
            None => {}
        }
        match location.1.checked_sub(1) {
            Some(_) => { temp_vec.push((location.0, location.1-1)) }
            None => {}
        }

        temp_vec.push((location.0 +1, location.1));
        temp_vec.push((location.0, location.1 + 1));

        // for i in &tempVec { println!("Link to check: {}{}", i.0, i.1)}

        return temp_vec;
    }

    match local_c {

        c if c == '|' => { vec![(location.0+1, location.1), (location.0-1, location.1)] },
        c if c == '7' => { vec![(location.0+1, location.1), (location.0, location.1-1)] },
        c if c == 'J' => { vec![(location.0, location.1-1), (location.0-1, location.1)] },
        c if c == 'F' => { vec![(location.0, location.1+1), (location.0+1, location.1)] },
        c if c == '-' => { vec![(location.0, location.1-1), (location.0, location.1+1)] },
        c if c == 'L' => { vec![(location.0-1, location.1), (location.0, location.1+1)] },
        _ => { vec![] }
    }
}
