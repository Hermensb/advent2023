use std::{collections::HashMap, fs};

fn main() {
    let data: String = fs::read_to_string("data/day6").expect("Didn't find the file?");
    let p1_result = part1(&data);
    println!("Part 1 Total {}", p1_result);

    let p2_result = part2(&data);
    println!("Part 2 Total {}", p2_result);
}

fn part1(input: &str) -> u64 {
    let data = convert_data1(input);
    data.iter()
        .map(|x| count_win_possibilities(*x.0, *x.1))
        .product()
}

fn part2(input: &str) -> u64 {
    let data = convert_data2(input);
    count_win_possibilities(data.0, data.1)
}

fn convert_data1(input: &str) -> HashMap<u64, u64> {
    let lines: Vec<Vec<u64>> = input
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
    let pairs: HashMap<u64, u64> = (0..lines[0].len())
        .map(|x| (lines[0][x], lines[1][x]))
        .collect();
    pairs
}

fn convert_data2(input: &str) -> (u64, u64) {
    let lines: Vec<u64> = input
        .split("\n")
        .filter(|x| x.len() > 0)
        .map(|x| {
            x.split(":")
                .last()
                .unwrap()
                .replace(" ", "")
                .parse()
                .unwrap()
        })
        .collect();
    (lines[0], lines[1])
}

fn count_win_possibilities(time: u64, record: u64) -> u64 {
    let winning_runs = (0..=time)
        .map(|x| (time - x) * x)
        .filter(|x| x > &record)
        .count();
    winning_runs as u64
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
        convert_data1(&get_test_data()),
        HashMap::from([(7, 9), (15, 40), (30, 200)])
    );
}

#[test]
fn test_part1() {
    assert_eq!(part1(&get_test_data()), 288);
}

#[test]
fn test_part2() {
    assert_eq!(part2(&get_test_data()), 71503);
}

#[test]
fn test_win_count() {
    assert_eq!(count_win_possibilities(7, 9), 4);
}

#[test]
fn test_data_to_tuple() {
    assert_eq!(convert_data2(&get_test_data()), (71530, 940200));
}
