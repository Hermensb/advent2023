use std::{collections::HashMap, fs};

fn main() {
    let data: String = fs::read_to_string("data/day7").expect("Didn't find the file?");
    let p1_result = part1(&data);
    println!("Part 1 Total {}", p1_result);

    let p2_result = part2(&data);
    println!("Part 2 Total {}", p2_result);
}

fn part1(_input: &str) -> u64 {
    0
}

fn part2(_input: &str) -> u64 {
    0
}

#[derive(Debug, PartialEq)]
struct Hand {
    pub items: HashMap<char, u8>,
    pub order_rep: u64,
    pub bid: u64,
}

impl Hand {
    fn new(input: &str) -> Hand {
        let parts: Vec<&str> = input.trim().split(' ').collect();
        let cards: String = parts.first().unwrap().trim().to_string();
        let bid: u64 = parts.last().unwrap().trim().parse().unwrap();
        let order_rep: u64 = hand_to_hex(&cards);
        let items = get_card_counts(&cards);

        Hand {
            items,
            order_rep,
            bid,
        }
    }
}

fn get_card_counts(input: &str) -> HashMap<char, u8> {
    let mut result = HashMap::new();
    for c in input.chars(){
        *result.entry(c).or_default() += 1;
    }
    result
}

fn char_to_hex(input: char) -> char {
    match input {
        '2' => return '0',
        '3' => return '1',
        '4' => return '2',
        '5' => return '3',
        '6' => return '4',
        '7' => return '5',
        '8' => return '6',
        '9' => return '7',
        'T' => return '8',
        'J' => return '9',
        'Q' => return 'a',
        'K' => return 'b',
        'A' => return 'c',
        _ => panic!(),
    }
}

fn hand_to_hex(input: &str) -> u64 {
    u64::from_str_radix(&input.chars().map(|x| char_to_hex(x)).collect::<String>(), 16).unwrap()
}

fn get_test_data() -> String {
    "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483"
        .to_string()
}

#[test]
fn test_part1() {
    assert_eq!(part1(&get_test_data()), 6440)
}

#[test]
fn test_part2() {
    todo!("fill in correct expectation");
    assert_eq!(part2(&get_test_data()), 1)
}

#[test]
fn test_hand_creation() {
    let hand = Hand::new("QQQJA 514");
    let expected = Hand {
        items: HashMap::from([('Q', 3), ('J', 1), ('A', 1)]),
        order_rep: 0xAAA9C,
        bid: 514,
    };
    assert_eq!(hand, expected);
}

#[test]
fn test_hand_to_hex() {
    let hand: String = "AKQJT98765432".to_string();
    let expected: u64 = 0xCBA9876543210;
    assert_eq!(hand_to_hex(&hand), expected);
}
