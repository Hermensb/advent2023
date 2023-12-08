
use std::fs;
use std::collections::HashMap;

fn main() {
    let data: String = fs::read_to_string("data/day8").expect("Didn't find the file?");
    let p1_result = part1(&data);
    println!("Part 1 Total {}", p1_result);

    let p2_result = part2(&data);
    println!("Part 2 Total {}", p2_result);
}

fn part1(input: &str) -> u64 {
    let mut lines = input.lines().filter(|x| x.len() > 0);
    let directions: Vec<Direction> = get_directions(lines.next().unwrap());
    let map: HashMap<&str, Next> = lines.map(|x| extract(x)).collect();

    let mut current_key: &str = &"AAA";
    let mut steps: u64 = 0;
    while current_key != "ZZZ" {
        let d_index = (steps % directions.len() as u64) as usize;
        let next_instruction = &directions[d_index];
        current_key = map.get(current_key).unwrap().next(next_instruction);
        steps +=1
    }
    steps
    
}

fn part2(_input: &str) -> u64 {
    0
}

#[derive(Debug, PartialEq)]
enum Direction {
    Left,
    Right,
}

#[derive(Debug)]
struct Next {
    left: String,
    right: String,
}

impl Next {
    fn new(left: &str, right: &str) -> Next {
        Next {
            left: left.to_string(),
            right: right.to_string(),
        }
    }

    fn next(&self, direction: &Direction) -> &str {
        match direction {
            Direction::Left => return &self.left,
            Direction::Right => return &self.right,
        }
    }
}

fn get_directions(data: &str) -> Vec<Direction> {
    let first_line: &str = data.lines().next().unwrap();
    first_line.chars().map(|x| to_direction(x)).collect()
}

fn to_direction(character: char) -> Direction {
    match character {
        'L' => return Direction::Left,
        'R' => return Direction::Right,
        _ => panic!("Bad char: {character}"),
    }
}

fn extract(data: &str) -> (&str, Next) {
    let mut parts = data.split('=');
    let key: &str = parts.next().unwrap().trim();
    let next_parts: Vec<&str> = parts
        .next()
        .unwrap()
        .trim_matches(char::is_whitespace)
        .trim_matches(|c| c == '(' || c == ')')
        .split(',')
        .map(|x| x.trim())
        .collect();
    let next: Next = Next::new(next_parts[0], next_parts[1]);
    (key, next)
}

#[allow(dead_code)]
fn get_test_data1() -> String {
    "RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)"
        .to_string()
}

#[allow(dead_code)]
fn get_test_data2() -> String {
    "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)"
        .to_string()
}

#[test]
fn test_part1_data1() {
    assert_eq!(part1(&get_test_data1()), 2)
}

#[test]
fn test_part1_data2() {
    assert_eq!(part1(&get_test_data2()), 6)
}

#[test]
fn test_get_directions() {
    let input = "LR".to_string();
    assert_eq!(
        get_directions(&input),
        vec![Direction::Left, Direction::Right]
    );
}
