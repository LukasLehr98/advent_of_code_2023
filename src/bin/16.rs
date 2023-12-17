use std::fs::read_to_string;

fn main(){
    let input = read_to_string("input/input16.txt").unwrap();
}

/*
grid datatype = char, bool

split lines by newline
for each line {
    for each char {
        insert in grid
    }
}

make step function ->

takes current coordinate in grid, recursively starts new iterations of step if its split
marks each coordinate as energized

make directions enum

for each sign, define out-directions based on in-direciton
*/

