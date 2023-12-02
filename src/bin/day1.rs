use std::fs;

fn main() {
    let data = fs::read_to_string("data/day1").expect("Didn't find the file?");
    let numbers = data.split('\n').map(|x| get_num(x));
    let total: u32 = numbers.sum();
    println!("Total: {}", total);
}

fn get_num(input: &str) -> u32 {
    if input.len() == 0 {
        return 0;
    }
    let numbers = input
        .chars()
        .filter(|x| x.is_ascii_digit())
        .collect::<Vec<char>>();
    let first = numbers
        .first()
        .expect("Didn't find first number")
        .to_digit(10)
        .expect("");
    let last = numbers
        .last()
        .expect("Didn't find last number")
        .to_digit(10)
        .expect("");
    (first * 10) + last
}

#[test]
fn test_get_num() {
    assert_eq!(75, get_num("kkyd7ki89sldicy5klk"));
}

#[test]
fn test_empty() {
    assert_eq!(0, get_num(""));
}
