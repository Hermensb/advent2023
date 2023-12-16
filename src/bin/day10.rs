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

#[derive(Debug, PartialEq)]
struct Pipe {
    pub connections: Vec<(usize, usize)>,
}

impl Pipe {
    fn new(letter: char, location: (usize, usize), grid_size: (usize, usize)) -> Pipe {
        let (north, south, east, west) = find_valid_spaces(location, grid_size);

        match letter {
            '|' => Pipe {
                connections: [north, south]
                    .iter()
                    .filter(|x| !x.is_none())
                    .map(|x| x.unwrap())
                    .collect(),
            },
            '-' => Pipe {
                connections: [east, west]
                    .iter()
                    .filter(|x| !x.is_none())
                    .map(|x| x.unwrap())
                    .collect(),
            },
            'J' => Pipe {
                connections: [north, west]
                    .iter()
                    .filter(|x| !x.is_none())
                    .map(|x| x.unwrap())
                    .collect(),
            },
            '7' => Pipe {
                connections: [west, south]
                    .iter()
                    .filter(|x| !x.is_none())
                    .map(|x| x.unwrap())
                    .collect(),
            },
            'F' => Pipe {
                connections: [east, south]
                    .iter()
                    .filter(|x| !x.is_none())
                    .map(|x| x.unwrap())
                    .collect(),
            },
            'L' => Pipe {
                connections: [north, east]
                    .iter()
                    .filter(|x| !x.is_none())
                    .map(|x| x.unwrap())
                    .collect(),
            },
            _ => panic!(),
        }
    }
}

fn get_size(space: &Vec<Vec<char>>) -> (usize, usize) {
    let max_row: usize = space.len();
    let max_column: usize = space.iter().map(|x| x.len()).min().unwrap();
    (max_row, max_column)
}

fn find_valid_spaces(
    input: (usize, usize),
    space_size: (usize, usize),
) -> (
    Option<(usize, usize)>,
    Option<(usize, usize)>,
    Option<(usize, usize)>,
    Option<(usize, usize)>,
) {
    let row: i64 = input.0 as i64;
    let column: i64 = input.1 as i64;
    let max_row: i64 = space_size.0 as i64;
    let max_column: i64 = space_size.1 as i64;

    if input.1 as i64 >= max_column || input.0 as i64 >= max_row {
        panic!("Invalid input! Out of bounds!");
    }

    let north = (row - 1, column);
    let south = (row + 1, column);
    let east = (row, column + 1);
    let west = (row, column - 1);

    let n_ret: Option<(usize, usize)>;
    let s_ret: Option<(usize, usize)>;
    let e_ret: Option<(usize, usize)>;
    let w_ret: Option<(usize, usize)>;

    if north.0 < 0 {
        n_ret = None;
    } else {
        n_ret = Some((north.0 as usize, north.1 as usize));
    }
    if south.0 >= max_row {
        s_ret = None;
    } else {
        s_ret = Some((south.0 as usize, south.1 as usize));
    }
    if east.1 >= max_column {
        e_ret = None;
    } else {
        e_ret = Some((east.0 as usize, east.1 as usize));
    }
    if west.1 < 0 {
        w_ret = None;
    } else {
        w_ret = Some((west.0 as usize, west.1 as usize));
    }

    (n_ret, s_ret, e_ret, w_ret)
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
    assert_eq!(find_start(&path1), Some((1, 1)));
    assert_eq!(find_start(&path2), Some((2, 0)));
}

#[test]
fn test_create_pipe_with_J() {
    let pipe = Pipe::new('J', (1, 1), (3, 3));
    let expected = Pipe {
        connections: vec![(0, 1), (1, 0)],
    };
    assert_eq!(pipe, expected);
}

#[test]
fn test_create_pipe_with_F() {
    let pipe = Pipe::new('F', (1, 1), (3, 3));
    let expected = Pipe {
        connections: vec![(1, 2), (2, 1)],
    };
    assert_eq!(pipe, expected);
}

#[test]
fn test_create_pipe_with_7() {
    let pipe = Pipe::new('7', (1, 1), (3, 3));
    let expected = Pipe {
        connections: vec![(1, 0), (2, 1)],
    };
    assert_eq!(pipe, expected);
}

#[test]
fn test_create_pipe_with_L() {
    let pipe = Pipe::new('L', (1, 1), (3, 3));
    let expected = Pipe {
        connections: vec![(1, 2), (0, 1)],
    };
    assert_eq!(pipe, expected);
}

#[test]
fn test_create_pipe_with_dash() {
    let pipe = Pipe::new('-', (1, 1), (3, 3));
    let expected = Pipe {
        connections: vec![(1, 0), (1, 2)],
    };
    assert_eq!(pipe, expected);
}

#[test]
fn test_create_pipe_with_vert() {
    let pipe = Pipe::new('-', (1, 1), (3, 3));
    let expected = Pipe {
        connections: vec![(1, 0), (1, 2)],
    };
    assert_eq!(pipe, expected);
}

#[test]
fn create_J_pipe_at_corner() {
    let pipe = Pipe::new('J', (0, 0), (3, 3));
    let expected = Pipe {
        connections: vec![],
    };
    assert_eq!(pipe, expected);
}

#[test]
fn create_F_pipe_at_corner() {
    let pipe = Pipe::new('F', (2, 2), (3, 3));
    let expected = Pipe {
        connections: vec![],
    };
    assert_eq!(pipe, expected);
}

#[test]
fn create_7_pipe_at_corner() {
    let pipe = Pipe::new('7', (2, 0), (3, 3));
    let expected = Pipe {
        connections: vec![],
    };
    assert_eq!(pipe, expected);
}

#[test]
fn create_L_pipe_at_corner() {
    let pipe = Pipe::new('L', (0, 2), (3, 3));
    let expected = Pipe {
        connections: vec![],
    };
    assert_eq!(pipe, expected);
}
