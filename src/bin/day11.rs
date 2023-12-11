use std::fs;

fn main() {
    let data: String = fs::read_to_string("data/day11").expect("Didn't find the file?");
    let p1_result = part1(&data);
    println!("Part 1 Total {}", p1_result);

    let p2_result = part2(&data);
    println!("Part 2 Total {}", p2_result);
}

fn part1(input: &str) -> u64 {
    0
}

fn part2(input: &str) -> u64 {
    0
}
