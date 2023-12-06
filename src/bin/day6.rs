use std::{collections::HashMap, fs};

fn main() {
    let data = fs::read_to_string("data/day6").expect("Didn't find the file?");
}

fn part1(input: &str) -> u32 {
    0
}

fn part2(input: &str) -> u32 {
    0
}

fn convert_data(input: &str) -> HashMap<u32, u32> {
    let lines: Vec<Vec<u32>> = input
        .split("\n")
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
    let pairs: HashMap<u32, u32> = (0..lines[0].len())
        .map(|x| (lines[0][x], lines[1][x]))
        .collect();
    pairs
}

fn count_win_possibilities(time: u32, record: u32) -> u32 {
    let winning_runs: Vec<u32> = (0..=time)
        .map(|x| (7 - x) * x)
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
