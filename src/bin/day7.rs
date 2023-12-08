use std::{
    collections::{HashMap, HashSet},
    fs,
};

fn main() {
    let data: String = fs::read_to_string("data/day7").expect("Didn't find the file?");
    let p1_result = part1(&data);
    println!("Part 1 Total {}", p1_result);

    let p2_result = part2(&data);
    println!("Part 2 Total {}", p2_result);
}

fn part1(input: &str) -> u64 {
    let mut hands: Vec<Hand> = input.lines().map(|x| Hand::new(x, false)).collect();
    hands.sort();
    let score: u64 = hands
        .iter()
        .enumerate()
        .map(|x| (x.0 + 1) as u64 * x.1.bid)
        .sum();
    score
}

fn part2(input: &str) -> u64 {
    let mut hands: Vec<Hand> = input.lines().map(|x| Hand::new(x, true)).collect();
    hands.sort();
    let score: u64 = hands
        .iter()
        .enumerate()
        .map(|x| (x.0 + 1) as u64 * x.1.bid)
        .sum();
    score
}

#[derive(Debug)]
struct Hand {
    pub items: HashMap<char, u8>,
    pub order_rep: u64,
    pub bid: u64,
    pub part2: bool,
}

impl Hand {
    fn new(input: &str, part2: bool) -> Hand {
        let parts: Vec<&str> = input.trim().split(' ').collect();
        let cards: String = parts.first().unwrap().trim().to_string();
        let bid: u64 = parts.last().unwrap().trim().parse().unwrap();
        let order_rep: u64 = hand_to_hex(&cards, part2);
        let items = get_card_counts(&cards);

        Hand {
            items,
            order_rep,
            bid,
            part2,
        }
    }

    fn is_five_of_a_kind(&self) -> bool {
        if self.items.values().len() == 1 {
            return true;
        }
        if self.part2 {
            if self.items.keys().filter(|x| x != &&'J').count() == 1 {
                return true;
            }
        }
        false
    }

    fn is_four_of_a_kind(&self) -> bool {
        if self.items.values().filter(|x| x == &&4).count() == 1 {
            return true;
        }
        if self.part2 {
            if let Some(count) = self.items.get(&'J') {
                let needed = 4 - count;
                let found = self
                    .items
                    .iter()
                    .filter(|x| x.0 != &'J')
                    .filter(|x| x.1 >= &needed)
                    .count();
                if found == 1 {
                    return true;
                }
            }
        }
        false
    }

    fn is_full_house(&self) -> bool {
        if self.items.values().collect::<HashSet<&u8>>() == HashSet::from([&2, &3]) {
            return true;
        }
        if self.part2 {
            if let Some(count) = self.items.get(&'J') {
                match count {
                    3 => return true,
                    2 => {
                        if self.items.values().len() < 4 {
                            return true;
                        }
                    }
                    1 => {
                        if self.items.values().len() == 3 {
                            return true;
                        }
                    }
                    _ => return true,
                }
            }
        }

        false
    }

    fn is_three_of_a_kind(&self) -> bool {
        if self.items.values().filter(|x| x == &&3).count() == 1 {
            return true;
        }
        if self.part2 {
            if let Some(count) = self.items.get(&'J') {
                let needed = 3 - count;
                let found = self
                    .items
                    .iter()
                    .filter(|x| x.0 != &'J')
                    .filter(|x| x.1 >= &needed)
                    .count();
                if found >= 1 {
                    return true;
                }
            }
        }
        false
    }

    fn is_two_pair(&self) -> bool {
        if self.items.values().filter(|x| x == &&2).count() == 2 {
            return true;
        }
        if self.part2 {
            if let Some(count) = self.items.get(&'J') {
                match count {
                    2 => return true,
                    1 => {
                        if self.items.values().len() < 5 {
                            return true;
                        }
                    }
                    _ => return true,
                }
            }
        }
        false
    }

    fn is_pair(&self) -> bool {
        if self.items.values().filter(|x| x == &&2).count() == 1 {
            return true;
        }
        if self.part2{
            if let Some(_count) = self.items.get(&'J') {
                return true;
            }
        }
        false
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let self_val = get_hand_val(self);
        let other_val = get_hand_val(other);

        if self_val > other_val {
            return Some(std::cmp::Ordering::Greater);
        } else if self_val < other_val {
            return Some(std::cmp::Ordering::Less);
        } else {
            if self.order_rep > other.order_rep {
                return Some(std::cmp::Ordering::Greater);
            } else if self.order_rep < other.order_rep {
                return Some(std::cmp::Ordering::Less);
            } else if self.order_rep == other.order_rep {
                return Some(std::cmp::Ordering::Equal);
            } else {
                return None;
            }
        }
    }
}

impl Eq for Hand {}
impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let result = self.partial_cmp(other);
        match result {
            Some(out) => return out,
            _ => panic!(),
        }
    }
}
impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        if self.order_rep == other.order_rep {
            return true;
        }
        false
    }
}

fn get_hand_val(hand: &Hand) -> u8 {
    if hand.is_five_of_a_kind() {
        return 6;
    } else if hand.is_four_of_a_kind() {
        return 5;
    } else if hand.is_full_house() {
        return 4;
    } else if hand.is_three_of_a_kind() {
        return 3;
    } else if hand.is_two_pair() {
        return 2;
    } else if hand.is_pair() {
        return 1;
    } else {
        return 0;
    }
}

fn get_card_counts(input: &str) -> HashMap<char, u8> {
    let mut result = HashMap::new();
    for c in input.chars() {
        *result.entry(c).or_default() += 1;
    }
    result
}

fn char_to_hex(input: char, part2: bool) -> char {
    match input {
        '2' => {
            if !part2 {
                return '0';
            } else {
                return '1';
            }
        }
        '3' => {
            if !part2 {
                return '1';
            } else {
                return '2';
            }
        }
        '4' => {
            if !part2 {
                return '2';
            } else {
                return '3';
            }
        }
        '5' => {
            if !part2 {
                return '3';
            } else {
                return '4';
            }
        }
        '6' => {
            if !part2 {
                return '4';
            } else {
                return '5';
            }
        }
        '7' => {
            if !part2 {
                return '5';
            } else {
                return '6';
            }
        }
        '8' => {
            if !part2 {
                return '6';
            } else {
                return '7';
            }
        }
        '9' => {
            if !part2 {
                return '7';
            } else {
                return '8';
            }
        }
        'T' => {
            if !part2 {
                return '8';
            } else {
                return '9';
            }
        }
        'J' => {
            if !part2 {
                return '9';
            } else {
                return '0';
            }
        }
        'Q' => return 'a',
        'K' => return 'b',
        'A' => return 'c',
        _ => panic!(),
    }
}

fn hand_to_hex(input: &str, part2: bool) -> u64 {
    u64::from_str_radix(
        &input
            .chars()
            .map(|x| char_to_hex(x, part2))
            .collect::<String>(),
        16,
    )
    .unwrap()
}

#[allow(dead_code)]
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
    assert_eq!(part2(&get_test_data()), 5905)
}

#[test]
fn test_hand_creation() {
    let hand = Hand::new("QQQJA 514", false);
    let expected = Hand {
        items: HashMap::from([('Q', 3), ('J', 1), ('A', 1)]),
        order_rep: 0xAAA9C,
        bid: 514,
        part2: false,
    };
    assert_eq!(hand, expected);
}

#[test]
fn test_hand_creation_part2() {
    let hand = Hand::new("QQQJA 514", false);
    let expected = Hand {
        items: HashMap::from([('Q', 3), ('J', 1), ('A', 1)]),
        order_rep: 0xAAA0C,
        bid: 514,
        part2: true,
    };
    //println!("{hand.order_rep:x}");
    assert_eq!(hand, expected);
}

#[test]
fn test_hand_to_hex() {
    let hand: String = "AKQJT98765432".to_string();
    let expected: u64 = 0xCBA9876543210;
    assert_eq!(hand_to_hex(&hand, false), expected);
}

#[test]
fn test_hand_to_hex_part2() {
    let hand: String = "AKQJT98765432".to_string();
    let expected: u64 = 0xCBA0987654321;
    assert_eq!(hand_to_hex(&hand, true), expected);
}

#[test]
fn test_five_of_kind() {
    let hand = Hand::new("AAAAA 111", false);
    assert!(hand.is_five_of_a_kind());
}

#[test]
fn test_five_of_kind_part2() {
    let hand = Hand::new("22JJ2 111", true);
    assert!(hand.is_five_of_a_kind());
}

#[test]
fn test_four_of_a_kind_part2() {
    let hand = Hand::new("23J3J 321", true);
    assert!(hand.is_four_of_a_kind());
}

#[test]
fn test_four_of_a_kind() {
    let hand = Hand::new("QQJQQ 45456", false);
    assert!(hand.is_four_of_a_kind());
}

#[test]
fn test_full_house() {
    let hand = Hand::new("33232 4454", false);
    assert!(hand.is_full_house());
}

#[test]
fn test_full_house_part2() {
    let hand = Hand::new("3323J 4454", true);
    assert!(hand.is_full_house());
}

#[test]
fn test_full_house_part2_2() {
    let hand = Hand::new("3J23J 4454", true);
    assert!(hand.is_full_house());
}

#[test]
fn test_two_pair() {
    let hand = Hand::new("23423 5", false);
    assert!(hand.is_two_pair());
}

#[test]
fn test_two_pair_part2() {
    let hand = Hand::new("2342J 5", true);
    assert!(hand.is_two_pair());
}

#[test]
fn test_two_pair_part2_2() {
    let hand = Hand::new("234JJ 5", true);
    assert!(hand.is_two_pair());
}

#[test]
fn test_pair() {
    let hand = Hand::new("78967 567", false);
    assert!(hand.is_pair());
}

#[test]
fn test_pair_part2() {
    let hand = Hand::new("78967 567", true);
    assert!(hand.is_pair());
}

#[test]
fn test_three_of_a_kind() {
    let hand = Hand::new("65646 554", false);
    assert!(hand.is_three_of_a_kind());
}

#[test]
fn test_three_of_a_kind_part2() {
    let hand = Hand::new("J5J46 554", true);
    assert!(hand.is_three_of_a_kind());
}
