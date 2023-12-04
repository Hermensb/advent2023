use std::{collections::HashSet, fs};

fn main() {
    let data: String = fs::read_to_string("data/day3").expect("Didn't find the file?");
    let p1_result = part1(&data);
    println!("Part 1 Total {}", p1_result);

    let p2_result = part2(&data);
    println!("Part 2 Total {}", p2_result);
}

fn part1(input: &str) -> u32 {
    let numbers: Vec<Number> = input
        .split('\n')
        .enumerate()
        .map(|x| get_numbers(x.1, x.0 as u32))
        .flatten()
        .collect();
    let overlap_locations: Vec<Location<i32>> = input
        .split('\n')
        .enumerate()
        .map(|x| get_symbols(x.1, x.0 as u32))
        .flatten()
        .map(|x| x.adjacent)
        .flatten()
        .filter(|x| x.x >= 0 && x.y >= 0)
        .collect();

    let overlaps: HashSet<Location<i32>> = HashSet::from_iter(overlap_locations.iter().cloned());

    let mut total: u32 = 0;

    for number in numbers {
        let intersection: Vec<&Location<i32>> = number
            .locations
            .iter()
            .filter(|x| overlaps.contains(*x))
            .collect();
        if intersection.len() > 0 {
            total += number.value;
        }
    }

    total
}

fn part2(input: &str) -> u32 {
    let mut total: u32 = 0;
    let numbers: Vec<Number> = input
        .split('\n')
        .enumerate()
        .map(|x| get_numbers(x.1, x.0 as u32))
        .flatten()
        .collect();
    let symbols: Vec<Symbol> = input
        .split('\n')
        .enumerate()
        .map(|x| get_symbols(x.1, x.0 as u32))
        .flatten()
        .filter(|x| x.symbol == '*')
        .collect();

    for symbol in symbols {
        let matched_numbers: Vec<&Number> = numbers
            .iter()
            .filter(|x| {
                &x.locations
                    .intersection(&symbol.adjacent)
                    .collect::<Vec<&Location<i32>>>()
                    .len()
                    > &0
            })
            .collect();

        if matched_numbers.len() > 1 {
            total += matched_numbers.iter().map(|x| x.value).product::<u32>();
        }
    }
    total
}

#[derive(Debug, PartialEq, PartialOrd, Ord, Eq, Hash, Clone)]
struct Location<T> {
    x: T,
    y: T,
}

impl<T> Location<T> {
    pub fn new(x: T, y: T) -> Location<T> {
        Location { x, y }
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Symbol {
    pub symbol: char,
    pub adjacent: HashSet<Location<i32>>,
}

impl Symbol {
    pub fn new(symbol: char, x: u32, y: u32) -> Symbol {
        let u = x as i32;
        let v = y as i32;
        let mut adjacent: HashSet<Location<i32>> = HashSet::new();
        adjacent.insert(Location::new(u - 1, v - 1));
        adjacent.insert(Location::new(u, v - 1));
        adjacent.insert(Location::new(u + 1, v - 1));
        adjacent.insert(Location::new(u - 1, v));
        adjacent.insert(Location::new(u + 1, v));
        adjacent.insert(Location::new(u - 1, v + 1));
        adjacent.insert(Location::new(u, v + 1));
        adjacent.insert(Location::new(u + 1, v + 1));
        Symbol { symbol, adjacent }
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Number {
    pub value: u32,
    pub locations: HashSet<Location<i32>>,
}

impl Number {
    pub fn new(digits: &str, last_location: Location<u32>) -> Number {
        let locations = (0..digits.len())
            .map(|x| Location::new(last_location.x as i32 - x as i32, last_location.y as i32))
            .collect();
        let value: u32 = digits.parse().expect("");
        Number { value, locations }
    }
}

fn get_numbers(input: &str, line: u32) -> Vec<Number> {
    let mut result: Vec<Number> = Vec::new();
    let mut accumulator: Vec<char> = Vec::new();
    for (index, character) in input.chars().enumerate() {
        if character.is_ascii_digit() {
            accumulator.push(character);
        } else if !accumulator.is_empty() {
            let num: String = accumulator.iter().collect();
            result.push(Number::new(&num, Location::new((index - 1) as u32, line)));
            accumulator = Vec::new();
        }
    }
    if !accumulator.is_empty() {
        let num: String = accumulator.iter().collect();
        result.push(Number::new(
            &num,
            Location::new((input.len() - 1) as u32, line),
        ));
    }
    result
}

fn get_symbols(input: &str, line: u32) -> Vec<Symbol> {
    let mut result: Vec<Symbol> = Vec::new();
    for (index, character) in input.chars().enumerate() {
        if !character.is_ascii_alphabetic()
            && !character.is_ascii_digit()
            && !character.is_ascii_whitespace()
            && character != '.'
        {
            result.push(Symbol::new(character, index as u32, line));
        }
    }
    result
}

#[allow(dead_code)]
fn get_test_data() -> String {
    "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598.."
        .to_string()
}

#[test]
fn test_data() {
    let data = get_test_data();
    let lines: Vec<&str> = data.split('\n').collect();
    assert_eq!(lines.first().expect(""), &"467..114..");
    assert_eq!(lines.last().expect(""), &".664.598..");
}

#[test]
fn test_input_part_1() {
    assert_eq!(part1(&get_test_data()), 4361);
}

#[test]
fn test_input_part_2() {
    assert_eq!(part2(&get_test_data()), 467835);
}

#[test]
fn new_symbol() {
    let symbol = Symbol::new('*', 3, 3);
    let expected: HashSet<Location<i32>> = HashSet::from([
        Location::new(2, 2),
        Location::new(2, 3),
        Location::new(2, 4),
        Location::new(3, 2),
        Location::new(3, 4),
        Location::new(4, 2),
        Location::new(4, 3),
        Location::new(4, 4),
    ]);
    let input = symbol.adjacent;
    assert_eq!(input, expected);
}

#[test]
fn number_creation() {
    let number = Number::new("654", Location::new(3, 3));
    assert_eq!(654, number.value);

    let expected_locations: HashSet<Location<i32>> = HashSet::from([
        Location::new(3, 3),
        Location::new(2, 3),
        Location::new(1, 3),
    ]);
    let input_locations = number.locations;

    assert_eq!(expected_locations, input_locations);
}

#[test]
fn number_fetch() {
    let result = get_numbers(&"..45.3.233.15", 5);
    let expected = vec![
        Number::new("45", Location::new(3, 5)),
        Number::new("3", Location::new(5, 5)),
        Number::new("233", Location::new(9, 5)),
        Number::new("15", Location::new(12, 5)),
    ];
    assert_eq!(result, expected);
}

#[test]
fn symbol_fetch() {
    let result = get_symbols(&"..@.*+..$.#", 5);
    let expected = vec![
        Symbol::new('@', 2, 5),
        Symbol::new('*', 4, 5),
        Symbol::new('+', 5, 5),
        Symbol::new('$', 8, 5),
        Symbol::new('#', 10, 5),
    ];
    assert_eq!(result.len(), 5);
    assert_eq!(result, expected);
}
