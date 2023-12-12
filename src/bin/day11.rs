use std::{collections::HashSet, fs};

fn main() {
    let data: String = fs::read_to_string("data/day11").expect("Didn't find the file?");
    let p1_result = part1(&data);
    println!("Part 1 Total {}", p1_result);

    let p2_result = part2(&data);
    println!("Part 2 Total {}", p2_result);
}

fn part1(input: &str) -> usize {
    let new_locations = expand_universe(&find_galaxies(input));
    let permutations: Vec<(&Location, &Location)> = get_permutations(&new_locations);
    permutations.iter().map(|x| x.0.distance_to(x.1)).sum()
}

fn part2(input: &str) -> u64 {
    0
}

#[derive(Debug, PartialEq, Eq, Hash)]
struct Location {
    pub x: usize,
    pub y: usize,
}

impl Location {
    pub fn distance_to(&self, other: &Self) -> usize {
        self.x.abs_diff(other.x) + self.y.abs_diff(other.y)
    }
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

fn get_empty_rows(galaxies: &HashSet<Location>) -> HashSet<usize> {
    let filled_rows: HashSet<usize> = galaxies.iter().map(|x| x.y).collect();
    (0..10).filter(|x| !filled_rows.contains(x)).collect()
}

fn get_empty_colums(galaxies: &HashSet<Location>) -> HashSet<usize> {
    let filled_columns: HashSet<usize> = galaxies.iter().map(|x| x.x).collect();
    (0..10).filter(|x| !filled_columns.contains(x)).collect()
}

fn expand_universe(galaxies: &HashSet<Location>) -> HashSet<Location> {
    let empty_columns = get_empty_colums(galaxies);
    let empty_rows = get_empty_rows(galaxies);

    let mut result = HashSet::new();
    for location in galaxies {
        let y = location.y + empty_rows.iter().filter(|&&i| i < location.y).count();
        let x = location.x + empty_columns.iter().filter(|&&i| i < location.x).count();
        result.insert(Location { x, y });
    }

    result
}

fn get_permutations(galaxies: &HashSet<Location>) -> Vec<(&Location, &Location)> {
    let galaxy_vec: Vec<&Location> = galaxies.iter().collect();
    let mut result: Vec<(&Location, &Location)> = vec![];

    for i in 0..galaxy_vec.len() {
        for j in i + 1..galaxy_vec.len() {
            result.push((galaxy_vec[i], galaxy_vec[j]));
        }
    }

    result
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
    let empty_rows: HashSet<usize> = get_empty_rows(&find_galaxies(&get_test_data()));
    let expected: HashSet<usize> = HashSet::from([3, 7]);
    assert_eq!(empty_rows, expected);
}

#[test]
fn test_find_empty_columns() {
    let empty_rows: HashSet<usize> = get_empty_colums(&find_galaxies(&get_test_data()));
    let expected: HashSet<usize> = HashSet::from([2, 5, 8]);
    assert_eq!(empty_rows, expected);
}

#[test]
fn test_expand_universe() {
    let new_locations = expand_universe(&find_galaxies(&get_test_data()));
    let expanded_data = "....#........
.........#...
#............
.............
.............
........#....
.#...........
............#
.............
.............
.........#...
#....#......."
        .to_string();
    let expected = find_galaxies(&expanded_data);
    assert_eq!(new_locations, expected);
}

#[test]
fn test_distance_between_locations() {
    let location1 = Location { x: 5, y: 11 };
    let location2 = Location { x: 1, y: 6 };
    assert_eq!(location2.distance_to(&location1), 9);
}

#[test]
fn test_gather_galaxy_pairs() {
    let new_locations = expand_universe(&find_galaxies(&get_test_data()));
    let permutations: Vec<(&Location, &Location)> = get_permutations(&new_locations);
    assert_eq!(permutations.len(), 36);
}

#[test]
fn test_part1() {
    assert_eq!(part1(&get_test_data()), 374);
}
