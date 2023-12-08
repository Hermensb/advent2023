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

fn part1(_input: &str) -> u64 {
    let mut hands: Vec<Hand> = _input.lines().map(|x| Hand::new(x)).collect();
    hands.sort();
    let score: u64 = hands
        .iter()
        .enumerate()
        .map(|x| (x.0 + 1) as u64 * x.1.bid)
        .sum();
    score
}

fn part2(_input: &str) -> u64 {
    0
}

#[derive(Debug)]
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

    fn is_five_of_a_kind(&self) -> bool {
        if self.items.values().len() == 1 {
            return true;
        }
        false
    }

    fn is_four_of_a_kind(&self) -> bool {
        if self.items.values().filter(|x| x == &&4).count() == 1 {
            return true;
        }
        false
    }

    fn is_full_house(&self) -> bool {
        if self.items.values().collect::<HashSet<&u8>>() == HashSet::from([&2, &3]) {
            return true;
        }
        false
    }

    fn is_three_of_a_kind(&self) -> bool {
        if self.items.values().filter(|x| x == &&3).count() == 1 {
            return true;
        }
        false
    }

    fn is_two_pair(&self) -> bool {
        if self.items.values().filter(|x| x == &&2).count() == 2 {
            return true;
        }
        false
    }

    fn is_pair(&self) -> bool {
        if self.items.values().filter(|x| x == &&2).count() == 1 {
            return true;
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
    u64::from_str_radix(
        &input.chars().map(|x| char_to_hex(x)).collect::<String>(),
        16,
    )
    .unwrap()
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

#[test]
fn test_five_of_kind() {
    let hand = Hand::new("AAAAA 111");
    assert!(hand.is_five_of_a_kind());
}

#[test]
fn test_four_of_a_kind() {
    let hand = Hand::new("QQJQQ 45456");
    assert!(hand.is_four_of_a_kind());
}

#[test]
fn test_full_house() {
    let hand = Hand::new("33232 4454");
    assert!(hand.is_full_house());
}

#[test]
fn test_two_pair() {
    let hand = Hand::new("23423 5");
    assert!(hand.is_two_pair());
}

#[test]
fn test_pair() {
    let hand = Hand::new("78967 567");
    assert!(hand.is_pair());
}

#[test]
fn test_three_of_a_kind() {
    let hand = Hand::new("65646 554");
    assert!(hand.is_three_of_a_kind());
}
