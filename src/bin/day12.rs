use std::{fs, io::Write};

fn main() {
    let data: String = fs::read_to_string("data/day12").expect("Didn't find the file?");
    let p1_result = part1(&data);
    println!("Part 1 Total {}", p1_result);

    let p2_result = part2(&data);
    println!("Part 2 Total {}", p2_result);
}

fn part1(input: &str) -> usize {
    0
}

fn part2(input: &str) -> usize {
    0
}

fn get_counts(input: &str) -> Vec<usize> {
    input
        .split(' ')
        .last()
        .unwrap()
        .split(',')
        .map(|x| x.parse::<usize>().unwrap())
        .collect()
}

fn get_permutations(options: &Vec<char>, count: u32) -> Vec<String> {
    let mut result: Vec<String> = vec![];
    let perm_count: usize = options.len().pow(count);

    (0..perm_count).map(|x| (0..count).fold(vec![] | acc, y | acc.push(options[(x as u32 % count as u32) as usize])));

        vec![]
}

#[allow(dead_code)]
fn get_test_data() -> String {
    "???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1"
        .to_string()
}

#[test]
fn test_count_extraction() {
    let data = get_test_data();
    let first_line = data.lines().next().unwrap();
    let counts: Vec<usize> = get_counts(&first_line);
    let expected = vec![1, 1, 3];
    assert_eq!(counts, expected);
}

#[test]
fn test_permutations() {
    let options = vec!['1', '2', '3'];
    let count = 2;
    let perms: Vec<String> = get_permutations(&options, count);
    let expected: Vec<String> = vec![
        "11".to_string(),
        "12".to_string(),
        "13".to_string(),
        "21".to_string(),
        "22".to_string(),
        "23".to_string(),
        "31".to_string(),
        "32".to_string(),
        "33".to_string(),
    ];
}

#[test]
fn test_fold() {
    (0..10).fold(vec![] |acc, x| acc.push(x));
}
