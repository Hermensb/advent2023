use std::collections::HashSet;
use std::fs;

fn main() {
    let data: String = fs::read_to_string("data/day10").expect("Didn't find the file?");
    let p1_result = part1(&data);
    println!("Part 1 Total {}", p1_result);

    let p2_result = part2(&data);
    println!("Part 2 Total {}", p2_result);
}

fn part1(input: &str) -> u64 {
    let processed_data = chars_to_pipes(&data_to_vec(input));
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

fn chars_to_pipes(input: &Vec<Vec<char>>) -> Vec<Vec<Option<Pipe>>> {
    let mut result: Vec<Vec<Option<Pipe>>> = vec![];
    for (row, line) in input.iter().enumerate() {
        let pipes: Vec<Option<Pipe>> = line
            .iter()
            .enumerate()
            .map(|x| Pipe::new(*x.1, (row, x.0), get_size(input)))
            .collect();
        result.push(pipes);
    }
    result
}

fn find_start(input: &Vec<Vec<Option<Pipe>>>) -> Option<(usize, usize)> {
    let (rows, columns) = get_size(input);
    for row in 0..rows {
        for column in 0..columns {
            if let Some(pipe) = &input[row][column] {
                if pipe.start {
                    return Some((row, column));
                }
            } else {
            }
        }
    }
    None
}

#[derive(Debug, PartialEq)]
struct Pipe {
    pub connections: HashSet<(usize, usize)>,
    pub start: bool,
}

impl Pipe {
    fn new(letter: char, location: (usize, usize), grid_size: (usize, usize)) -> Option<Pipe> {
        let (north, south, east, west) = find_valid_spaces(location, grid_size);

        match letter {
            '|' => Some(Pipe {
                connections: [north, south]
                    .iter()
                    .filter(|x| !x.is_none())
                    .map(|x| x.unwrap())
                    .collect(),
                start: false,
            }),
            '-' => Some(Pipe {
                connections: [east, west]
                    .iter()
                    .filter(|x| !x.is_none())
                    .map(|x| x.unwrap())
                    .collect(),
                start: false,
            }),
            'J' => Some(Pipe {
                connections: [north, west]
                    .iter()
                    .filter(|x| !x.is_none())
                    .map(|x| x.unwrap())
                    .collect(),
                start: false,
            }),
            '7' => Some(Pipe {
                connections: [west, south]
                    .iter()
                    .filter(|x| !x.is_none())
                    .map(|x| x.unwrap())
                    .collect(),
                start: false,
            }),
            'F' => Some(Pipe {
                connections: [east, south]
                    .iter()
                    .filter(|x| !x.is_none())
                    .map(|x| x.unwrap())
                    .collect(),
                start: false,
            }),
            'L' => Some(Pipe {
                connections: [north, east]
                    .iter()
                    .filter(|x| !x.is_none())
                    .map(|x| x.unwrap())
                    .collect(),
                start: false,
            }),
            'S' => Some(Pipe {
                connections: HashSet::new(),
                start: true,
            }),
            _ => None,
        }
    }
}

fn get_size<T>(space: &Vec<Vec<T>>) -> (usize, usize) {
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

#[cfg(test)]
mod tests {
    use super::*;

    fn get_test_data_1() -> String {
        ".....
.S-7.
.|.|.
.L-J.
....."
            .to_string()
    }

    fn get_test_data_2() -> String {
        "..F7.
.FJ|.
SJ.L7
|F--J
LJ..."
            .to_string()
    }

    #[test]
    fn test_find_start() {
        let Some(start): Option<(usize, usize)> =
            find_start(&chars_to_pipes(&data_to_vec(&get_test_data_1())))
        else {
            panic!()
        };
        assert_eq!(start, (1, 1));
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
    fn test_get_size() {
        let (row, column) = get_size(&data_to_vec(&get_test_data_1()));
        assert_eq!(row, 5);
        assert_eq!(column, 5);
    }

    #[test]
    fn test_create_pipe_with_J() {
        let Some(pipe) = Pipe::new('J', (1, 1), (3, 3)) else {
            panic!()
        };
        let expected = Pipe {
            connections: HashSet::from([(0, 1), (1, 0)]),
            start: false,
        };
        assert_eq!(pipe, expected);
    }

    #[test]
    fn test_create_pipe_with_F() {
        let Some(pipe) = Pipe::new('F', (1, 1), (3, 3)) else {
            panic!()
        };
        let expected = Pipe {
            connections: HashSet::from([(1, 2), (2, 1)]),
            start: false,
        };
        assert_eq!(pipe, expected);
    }

    #[test]
    fn test_create_pipe_with_7() {
        let Some(pipe) = Pipe::new('7', (1, 1), (3, 3)) else {
            panic!()
        };
        let expected = Pipe {
            connections: HashSet::from([(1, 0), (2, 1)]),
            start: false,
        };
        assert_eq!(pipe, expected);
    }

    #[test]
    fn test_create_pipe_with_L() {
        let Some(pipe) = Pipe::new('L', (1, 1), (3, 3)) else {
            panic!()
        };
        let expected = Pipe {
            connections: HashSet::from([(1, 2), (0, 1)]),
            start: false,
        };
        assert_eq!(pipe, expected);
    }

    #[test]
    fn test_create_pipe_with_dash() {
        let Some(pipe) = Pipe::new('-', (1, 1), (3, 3)) else {
            panic!()
        };
        let expected = Pipe {
            connections: HashSet::from([(1, 0), (1, 2)]),
            start: false,
        };
        assert_eq!(pipe, expected);
    }

    #[test]
    fn test_create_pipe_with_vert() {
        let Some(pipe) = Pipe::new('-', (1, 1), (3, 3)) else {
            panic!()
        };
        let expected = Pipe {
            connections: HashSet::from([(1, 0), (1, 2)]),
            start: false,
        };
        assert_eq!(pipe, expected);
    }

    #[test]
    fn create_J_pipe_at_corner() {
        let Some(pipe) = Pipe::new('J', (0, 0), (3, 3)) else {
            panic!()
        };
        let expected = Pipe {
            connections: HashSet::from([]),
            start: false,
        };
        assert_eq!(pipe, expected);
    }

    #[test]
    fn create_F_pipe_at_corner() {
        let Some(pipe) = Pipe::new('F', (2, 2), (3, 3)) else {
            panic!()
        };
        let expected = Pipe {
            connections: HashSet::from([]),
            start: false,
        };
        assert_eq!(pipe, expected);
    }

    #[test]
    fn create_7_pipe_at_corner() {
        let Some(pipe) = Pipe::new('7', (2, 0), (3, 3)) else {
            panic!()
        };
        let expected = Pipe {
            connections: HashSet::from([]),
            start: false,
        };
        assert_eq!(pipe, expected);
    }

    #[test]
    fn create_L_pipe_at_corner() {
        let Some(pipe) = Pipe::new('L', (0, 2), (3, 3)) else {
            panic!()
        };
        let expected = Pipe {
            connections: HashSet::from([]),
            start: false,
        };
        assert_eq!(pipe, expected);
    }

    #[test]
    fn test_other_char() {
        let pipe = Pipe::new('.', (0, 0), (3, 3));
        assert!(pipe.is_none());
    }
}
