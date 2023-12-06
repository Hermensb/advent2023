use std::{collections::HashMap, fs};

fn main() {
    let data: String = fs::read_to_string("data/day6").expect("Didn't find the file?");
    let p1_result = part1(&data);
    println!("Part 1 Total {}", p1_result);

    let p2_result = part2(&data);
    println!("Part 2 Total {}", p2_result);
}

fn part1(input: &str) -> u32 {
    let data = convert_data(input);
    data.iter().map(|x| count_win_possibilities(*x.0, *x.1)).product()
}

fn part2(input: &str) -> u32 {
    0
}

fn convert_data(input: &str) -> HashMap<u32, u32> {
    let lines: Vec<Vec<u32>> = input
        .split("\n")
        .filter(|x| x.len() > 0)
        .map(|x| {
            x.split(":")
                .last()
                .unwrap()
                .trim()
                .split(" ")
                .filter(|x| x.len() > 0)
                .map(|x| x.parse().unwrap())
                .collect()
        })
        .collect();
        println!("{lines:?}");
    let pairs: HashMap<u32, u32> = (0..lines[0].len())
        .map(|x| (lines[0][x], lines[1][x]))
        .collect();
    pairs
}

fn count_win_possibilities(time: u32, record: u32) -> u32 {
    let winning_runs: Vec<u32> = (0..=time)
        .map(|x| (time - x) * x)
        .filter(|x| x > &record)
        .collect();
    winning_runs.len() as u32
}

#[allow(dead_code)]
fn get_test_data() -> String {
    "Time:      7  15   30
Distance:  9  40  200"
        .to_string()
}

#[test]
fn test_data_to_hashmap() {
    assert_eq!(
        convert_data(&get_test_data()),
        HashMap::from([(7, 9), (15, 40), (30, 200)])
    );
}

#[test]
fn test_part1() {
    assert_eq!(part1(&get_test_data()), 288);
}

#[test]
fn test_win_count() {
    assert_eq!(count_win_possibilities(7, 9), 4);
}
