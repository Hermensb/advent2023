use std::{
    collections::{HashMap, HashSet},
    fs,
};

fn main() {
    let data: String = fs::read_to_string("data/day4").expect("Didn't find the file?");
    let p1_result = part1(&data);
    println!("Part 1 Total {}", p1_result);

    let p2_result = part2(&data);
    println!("Part 2 Total {}", p2_result);
}

fn part1(input: &str) -> u32 {
    let lines = input.split('\n').filter(|x| x.len() > 0);
    let total: u32 = lines
        .map(|x| extract_sets(x))
        .map(|x| get_score(&x.0, &x.1))
        .sum();
    total
}

fn part2(input: &str) -> u32 {
    let mut card_counts: HashMap<u32, u32> = HashMap::new();
    let lines = input.split('\n').filter(|x| x.len() > 0);
    let overlap_counts: Vec<u32> = lines
        .map(|x| extract_sets(x))
        .map(|x| get_overlap_count(&x.0, &x.1))
        .collect();

    for (index, count) in overlap_counts.iter().enumerate() {
        let game_number: u32 = (index + 1) as u32;
        *card_counts.entry(game_number).or_default() += 1;

        let current_card_count: u32 = card_counts.get(&game_number).unwrap().clone();

        for i in game_number + 1..game_number + count + 1 {
            *card_counts.entry(i).or_default() += current_card_count;
        }
    }
    card_counts.values().sum()
}

fn extract_sets(input: &str) -> (HashSet<u32>, HashSet<u32>) {
    let numbers: Vec<&str> = input.split(':').last().expect("").split('|').collect();
    let entries: HashSet<u32> = HashSet::from_iter(
        numbers
            .first()
            .expect("")
            .split(" ")
            .filter(|x| x.len() > 0)
            .map(|x| x.parse::<u32>().unwrap()),
    );
    let winning_numbers: HashSet<u32> = HashSet::from_iter(
        numbers
            .last()
            .expect("")
            .split(" ")
            .filter(|x| x.len() > 0)
            .map(|x| x.parse::<u32>().unwrap()),
    );
    (entries, winning_numbers)
}

fn get_overlap_count(entries: &HashSet<u32>, winning_numbers: &HashSet<u32>) -> u32 {
    let overlap: Vec<&u32> = entries.intersection(winning_numbers).collect();
    overlap.len() as u32
}

fn get_score(entries: &HashSet<u32>, winning_numbers: &HashSet<u32>) -> u32 {
    let overlap: Vec<&u32> = entries.intersection(winning_numbers).collect();
    double(overlap.len().try_into().unwrap())
}

fn double(count: u32) -> u32 {
    if count == 0 {
        return 0;
    }
    let base: u32 = 2;
    base.pow(count - 1)
}

#[allow(dead_code)]
fn test_data() -> String {
    "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11"
        .to_string()
}

#[test]
fn test_part1() {
    assert_eq!(part1(&test_data()), 13);
}

#[test]
fn test_part2() {
    assert_eq!(part2(&test_data()), 30);
}

#[test]
fn line_into_hashsets() {
    let line = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53";
    let expected = (
        HashSet::from([41, 48, 83, 86, 17]),
        HashSet::from([83, 86, 6, 31, 17, 9, 48, 53]),
    );
    assert_eq!(extract_sets(line), expected);
}

#[test]
fn test_get_score() {
    let entries: HashSet<u32> = HashSet::from([13, 25, 17, 47, 64, 18]);
    let winning_numbers: HashSet<u32> = HashSet::from([12, 13, 44, 47, 65, 18]);
    assert_eq!(get_score(&entries, &winning_numbers), 4);
}

#[test]
fn test_get_score_no_overlap() {
    let entries: HashSet<u32> = HashSet::from([13, 25, 17, 47, 64, 18]);
    let winning_numbers: HashSet<u32> = HashSet::from([12, 44, 65]);
    assert_eq!(get_score(&entries, &winning_numbers), 0);
}
