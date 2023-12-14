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

#[derive(Debug, Hash, Eq, PartialEq)]
enum Direction<T> {
    North(T),
    South(T),
    East(T),
    West(T),
}


#[allow(dead_code)]
fn get_test_data_1() -> String {
    ".....
.S-7.
.|.|.
.L-J.
....."
        .to_string()
}

#[allow(dead_code)]
fn get_test_data_2() -> String {
    "..F7.
.FJ|.
SJ.L7
|F--J
LJ..."
        .to_string()
}
#[test]
fn test_data_to_vec() {
    let expected: Vec<Vec<char>> = vec![
        vec!['.', '.', '.', '.', '.'],
        vec!['.', 'S', '-', '7', '.'],
        vec!['.', '|', '.', '|', '.'],
        vec!['.', 'L', '-', 'J', '.'],
        vec!['.', '.', '.', '.', '.'],
    ];
    assert_eq!(expected, data_to_vec(&get_test_data_1()));
}

#[test]
fn test_find_start() {
    let path1 = data_to_vec(&get_test_data_1());
    let path2 = data_to_vec(&get_test_data_2());
    assert_eq!(find_start(&path1), Some((1, 1)));
    assert_eq!(find_start(&path2), Some((2, 0)));
}
