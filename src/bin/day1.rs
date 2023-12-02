use std::fs;

fn main() {
    let data = fs::read_to_string("data/day1").expect("Didn't find the file?");
    let numbers = data
        .split('\n')
        .filter(|x| x.len() > 0)
        .map(|x| replace_word_digits(x))
        .map(|x| get_num(&x));
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

fn replace_word_digits(input: &str) -> String {
    input
        .replace("oneight", "18")
        .replace("threeight", "38")
        .replace("fiveight", "58")
        .replace("nineight", "98")
        .replace("eightwo", "82")
        .replace("twone", "21")
        .replace("one", "1")
        .replace("two", "2")
        .replace("three", "3")
        .replace("four", "4")
        .replace("five", "5")
        .replace("six", "6")
        .replace("seven", "7")
        .replace("eight", "8")
        .replace("nine", "9")
}

#[test]
fn test_get_num() {
    assert_eq!(75, get_num("kkyd7ki89sldicy5klk"));
}

#[test]
fn test_empty() {
    assert_eq!(0, get_num(""));
}

#[test]
fn test_convert() {
    assert_eq!("4xv56yy", &replace_word_digits("fourxvfivesixyy"));
}
