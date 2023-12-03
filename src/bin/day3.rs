use std::fs;

fn main() {
    let data: String = fs::read_to_string("data/day3").expect("Didn't find the file?");
}

fn part1(input: String) -> u32 {
    let lines: Vec<&str> = input.split('\n').collect();
    58
}

#[derive(Debug, PartialEq, PartialOrd, Ord, Eq)]
struct Location<T> {
    x: T,
    y: T,
}

impl<T> Location<T> {
    pub fn new(x: T, y: T) -> Location<T> {
        Location { x, y }
    }
}

struct Symbol {
    pub adjacent: Vec<Location<i32>>,
}

impl Symbol {
    pub fn new(x: u32, y: u32) -> Symbol {
        let u = x as i32;
        let v = y as i32;
        let adjacent = vec![
            Location::new(u - 1, v - 1),
            Location::new(u, v - 1),
            Location::new(u + 1, v - 1),
            Location::new(u - 1, v),
            Location::new(u + 1, v),
            Location::new(u - 1, v + 1),
            Location::new(u, v + 1),
            Location::new(u + 1, v + 1),
        ];
        Symbol { adjacent: adjacent }
    }
}

struct Number {
    pub value: u32,
    pub locations: Vec<Location<i32>>,
}

impl Number {
    pub fn new(digits: &str, last_location: Location<u32>) -> Number {
        let locations = (0..digits.len())
            .map(|x| Location::new(last_location.x as i32 - x as i32, last_location.y as i32))
            .collect();
        let value: u32 = digits.parse().expect("");
        Number{value, locations}
        
    }
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
fn test_input() {
    assert_eq!(part1(get_test_data()), 4361);
}

#[test]
fn new_symbol() {
    let symbol = Symbol::new(3, 3);
    let mut expected = vec![
        Location::new(2, 2),
        Location::new(2, 3),
        Location::new(2, 4),
        Location::new(3, 2),
        Location::new(3, 4),
        Location::new(4, 2),
        Location::new(4, 3),
        Location::new(4, 4),
    ];
    let mut input = symbol.adjacent;
    input.sort();
    expected.sort();
    assert_eq!(input, expected);
}

#[test]
fn number_creation() {
    let number = Number::new("654", Location::new(3, 3));
    assert_eq!(654, number.value);

    let mut expected_locations = vec![
        Location::new(3, 3),
        Location::new(2, 3),
        Location::new(1, 3),
    ];
    let mut input_locations = number.locations;
    input_locations.sort();
    expected_locations.sort();

    assert_eq!(expected_locations, input_locations);
}
