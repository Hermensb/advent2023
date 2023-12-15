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
    let _start = find_start(&processed_data);
    0
}

fn part2(_input: &str) -> u64 {
    0
}

fn data_to_vec(input: &str) -> Vec<Vec<char>> {
    input
        .lines()
        .filter(|x| x.len() > 0)
        .map(|x| x.chars().collect())
        .collect()
}

fn find_start(input: &Vec<Vec<char>>) -> Option<Coord<usize>> {
    for (row, chars) in input.iter().enumerate() {
        for (column, letter) in chars.iter().enumerate() {
            if letter == &'S' {
                return Some(Coord { row, column });
            }
        }
    }
    None
}

enum Direction<T> {
    North(T),
    South(T),
    East(T),
    West(T),
}

struct Pipe {
    pub connections: Vec<Coord<usize>>,
}

impl Pipe {
    fn new(letter: char, location: &Coord<usize>, path: &Vec<Vec<char>>) -> Pipe {
        let mut _connections: Vec<Coord<usize>>;

        match letter {
            '|' => todo!(),
            '-' => todo!(),
            'J' => todo!(),
            '7' => todo!(),
            'F' => todo!(),
            'L' => todo!(),
            _ => panic!(),
        }
    }
}

fn find_valid_spaces(
    input: &Coord<usize>,
    path: &Vec<Vec<char>>,
) -> Vec<Direction<Option<Coord<usize>>>> {
    let row: i64 = input.row as i64;
    let column: i64 = input.column as i64;
    let max_row: i64 = path.len() as i64;
    let max_column: i64 = path.iter().map(|x| x.len()).min().unwrap() as i64;

    if input.column as i64 >= max_column || input.row as i64 >= max_row{
        panic!("Invalid input! Out of bounds!");
    }

    let north = (row - 1, column);
    let south = (row + 1, column);
    let east = (row, column + 1);
    let west = (row, column - 1);

    let mut result: Vec<Direction<Option<Coord<usize>>>> = vec![];

    use Direction::*;
    if north.0 < 0 {
        result.push(North(None))
    } else {
        result.push(North(Some(Coord {
            row: north.0 as usize,
            column: north.1 as usize,
        })))
    }
    if south.0 >= max_row {
        result.push(South(None))
    } else {
        result.push(South(Some(Coord {
            row: south.0 as usize,
            column: south.1 as usize,
        })))
    }
    if east.1 >= max_column{
        result.push(East(None))
    } else {
        result.push(East(Some(Coord {
            row: east.0 as usize,
            column: east.1 as usize,
        })))
    }
    if west.1 < 0 {
        result.push(West(None))
    } else {
        result.push(West(Some(Coord {
            row: west.0 as usize,
            column: west.1 as usize,
        })))
    }

    result

}

#[derive(Debug, PartialEq)]
struct Coord<T> {
    pub row: T,
    pub column: T,
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
    assert_eq!(find_start(&path1), Some(Coord { row: 1, column: 1 }));
    assert_eq!(find_start(&path2), Some(Coord { row: 2, column: 0 }));
}

#[test]
fn test_numbers() {
    let _a: usize = 5;
    let _b: usize = 7;
    let _c: usize = 5 % 7;
}

#[test]
fn test_create_pipe_with_J() {
    todo!();
}

#[test]
fn test_create_pipe_with_F() {
    todo!();
}

#[test]
fn test_create_pipe_with_7() {
    todo!();
}

#[test]
fn test_create_pipe_with_L() {
    todo!();
}

#[test]
fn test_create_pipe_with_dash() {
    todo!();
}

#[test]
fn test_create_pipe_with_vert() {
    todo!();
}

#[test]
fn create_J_pipe_at_corner() {
    todo!();
}

#[test]
fn create_F_pipe_at_corner() {
    todo!();
}
