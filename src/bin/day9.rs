use std::collections::HashSet;
use std::fs;

fn main() {
    let data: String = fs::read_to_string("data/day9").expect("Didn't find the file?");
    let p1_result = part1(&data);
    println!("Part 1 Total {}", p1_result);

    let p2_result = part2(&data);
    println!("Part 2 Total {}", p2_result);
}

fn part1(input: &str) -> i64 {
    let sequence_list: Vec<Vec<i64>> = input.lines().map(|x| extract_sequence(x)).collect();
    let next_vals: Vec<i64> = sequence_list.iter().map(|x| get_next_item(x)).collect();
    next_vals.iter().sum()
}

fn part2(input: &str) -> i64 {
    let sequence_list: Vec<Vec<i64>> = input.lines().map(|x| extract_sequence(x)).collect();
    let next_vals: Vec<i64> = sequence_list.iter().map(|x| get_first_item(x)).collect();
    next_vals.iter().sum()
}

fn get_next_item(sequence: &Vec<i64>) -> i64 {
    if sequence.iter().collect::<HashSet<&i64>>() == HashSet::from([&0]) {
        return 0;
    }
    sequence.last().unwrap() + get_next_item(&get_differences(&sequence))
}

fn get_first_item(sequence: &Vec<i64>) -> i64 {
    if sequence.iter().collect::<HashSet<&i64>>() == HashSet::from([&0]) {
        return 0;
    }
    sequence.first().unwrap() - get_first_item(&get_differences(&sequence))
}

fn get_differences(sequence: &Vec<i64>) -> Vec<i64> {
    (0..sequence.len() - 1)
        .map(|x| sequence[x + 1] - sequence[x])
        .collect::<Vec<i64>>()
}

fn extract_sequence(input: &str) -> Vec<i64> {
    input
        .trim()
        .split(' ')
        .filter(|x| x.len() > 0)
        .map(|x| x.parse().unwrap())
        .collect()
}

#[allow(dead_code)]
fn get_test_data() -> String {
    "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45"
        .to_string()
}

#[test]
fn test_part1() {
    assert_eq!(part1(&get_test_data()), 114);
}

#[test]
fn test_part2() {
    assert_eq!(part2(&get_test_data()), 2);
}

#[test]
fn test_get_differences() {
    let input = vec![1, 4, 9, 13];
    let expected = vec![3, 5, 4];
    assert_eq!(get_differences(&input), expected);
}

#[test]
fn test_get_next_in_sequence() {
    let input = vec![1, 3, 5, 7, 9];
    let expected = 11;
    assert_eq!(get_next_item(&input), expected);
}

#[test]
fn test_line_to_vec_of_nums() {
    let input = "455 822 357 454";
    let expected: Vec<i64> = vec![455, 822, 357, 454];
    assert_eq!(extract_sequence(&input), expected);
}

#[test]
fn test_get_first_item() {
    let input = vec![3, 5, 7, 9, 11];
    let expected = 1;
    assert_eq!(get_first_item(&input), 1);
}
