To run a single day: cargo run package --release --bin {day}
To run the whole package: cargo run --package advent_of_code_2023 --bin advent_of_code_2023

# Run files with cargo
cargo run --release --bin $DAY                                                # run a specific day, e.g 01
cargo run --package advent_of_code_2023 --release --bin advent_of_code_2023   # run all days
