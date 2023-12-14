use std::fs;

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

