use std::fs;

fn main() {
    let data: String = fs::read_to_string("data/day11").expect("Didn't find the file?");
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
