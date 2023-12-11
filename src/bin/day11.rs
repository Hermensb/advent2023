use std::{collections::HashSet, fs};

fn main() {
    let data: String = fs::read_to_string("data/day11").expect("Didn't find the file?");
    let p1_result = part1(&data);
    println!("Part 1 Total {}", p1_result);

    let p2_result = part2(&data);
    println!("Part 2 Total {}", p2_result);
}

fn part1(input: &str) -> u64 {
    0
}

fn part2(input: &str) -> u64 {
    0
}

#[derive(Debug, PartialEq, Eq, Hash)]
struct Location {
    pub x: usize,
    pub y: usize,
}

fn find_galaxies(sky_map: &str) -> HashSet<Location> {
    let mut locations: HashSet<Location> = HashSet::new();

    for (row, line) in sky_map.lines().enumerate() {
        for (column, object) in line.chars().enumerate() {
            match object {
                '#' => {
                    locations.insert(Location { x: column, y: row });
                }
                _ => {}
            }
        }
    }
    locations
}

fn get_empty_rows(galaxies: &HashSet<Location>, total_rows: usize) -> HashSet<usize> {
    let filled_rows: HashSet<usize> = galaxies.iter().map(|x| x.y).collect();
    (0..10).filter(|x| !filled_rows.contains(x)).collect()
}

#[allow(dead_code)]
fn get_test_data() -> String {
    "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#....."
        .to_string()
}

#[test]
fn test_find_galaxies() {
    let galaxies: HashSet<Location> = find_galaxies(&get_test_data());
    let expected: HashSet<Location> = HashSet::from([
        Location { x: 3, y: 0 },
        Location { x: 7, y: 1 },
        Location { x: 0, y: 2 },
        Location { x: 6, y: 4 },
        Location { x: 1, y: 5 },
        Location { x: 9, y: 6 },
        Location { x: 7, y: 8 },
        Location { x: 0, y: 9 },
        Location { x: 4, y: 9 },
    ]);
    let diff = galaxies.difference(&expected);
    println!("Unexpected locations: {diff:?}");
    assert_eq!(galaxies, expected);
}

#[test]
fn test_find_empty_rows() {
    let empty_rows: HashSet<usize> = get_empty_rows(&find_galaxies(&get_test_data()), 10);
    let expected: HashSet<usize> = HashSet::from([3, 7]);
    assert_eq!(empty_rows, expected);
}
