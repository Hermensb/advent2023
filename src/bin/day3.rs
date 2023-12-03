use std::fs;

fn main() {
    let data: String = fs::read_to_string("data/day3").expect("Didn't find the file?");
}

fn part1(input: String) -> u32 {
    let lines: Vec<&str> = input.split('\n').collect();
    58
}

fn get_test_data() -> String {
    "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598.."
        .to_string()
}

#[test]
fn test_data() {
    let data = get_test_data();
    let lines: Vec<&str> = data.split('\n').collect();
    assert_eq!(lines.first().expect(""), &"467..114..");
    assert_eq!(lines.last().expect(""), &".664.598..");
}

#[test]
fn test_input(){
    assert_eq!(part1(get_test_data()), 4361);
}
