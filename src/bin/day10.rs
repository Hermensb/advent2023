use std::fs;

fn main() {
    let data: String = fs::read_to_string("data/day10").expect("Didn't find the file?");
    let p1_result = part1(&data);
    println!("Part 1 Total {}", p1_result);

    let p2_result = part2(&data);
    println!("Part 2 Total {}", p2_result);
}

fn part1(input: &str) -> u64 {
    let processed_data = data_to_vec(input);
    let start = find_start(&processed_data);
    println!("{start:?}");
    0
}

fn part2(input: &str) -> u64 {
    0
}

fn data_to_vec(input: &str) -> Vec<Vec<char>> {
    input
        .lines()
        .filter(|x| x.len() > 0)
        .map(|x| x.chars().collect())
        .collect()
}

fn find_start(input: &Vec<Vec<char>>) -> Option<(usize, usize)> {
    for (row, chars) in input.iter().enumerate() {
        for (column, letter) in chars.iter().enumerate() {
            if letter == &'S' {
                return Some((row, column));
            }
        }
    }
    None
}
