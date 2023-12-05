use std::{collections::HashMap, fs};

fn main() {
    let data: String = fs::read_to_string("data/day5").expect("Didn't find the file?");
    let p1_result = part1(&data);
    println!("Part 1 Total {}", p1_result);

    let p2_result = part2(&data);
    println!("Part 2 Total {}", p2_result);
}

fn part1(input: &str) -> u32 {
    let lines = input.split("\n\n");
    for line in lines {
        println!("{line:?}");
    }
    0
}

fn part2(input: &str) -> u32 {
    0
}

struct Translator<'a> {
    translations: HashMap<u64, u64>,
    translator: Option<&'a Translator<'a>>,
}

impl<'a> Translator<'a> {
    fn new(input: Vec<(u64, u64, u64)>) -> Translator<'a> {
        let mut translations: HashMap<u64, u64> = HashMap::new();

        for (to, from, iterations) in input {
            for i in 0..iterations {
                translations.insert(from + i, to + i);
            }
        }
        println!("{translations:?}");

        Translator {
            translations,
            translator: None,
        }
    }

    fn set_internal_translator(&mut self, translator: &'a Translator) {
        self.translator = Some(translator);
    }

    fn translate(&self, input: u64) -> u64 {
        let result: Option<u64> = self.translations.get(&input).copied();
        let mut output: u64;

        match result {
            Some(number) => output = number,
            None => output = input,
        }
        match self.translator {
            Some(t) => return t.translate(output),
            None => return output,
        }
    }
}

fn get_test_data() -> String {
    "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4"
        .to_string()
}

#[test]
fn test_part1() {
    assert_eq!(part1(&get_test_data()), 35);
}

#[test]
fn create_translator() {
    let input: Vec<(u64, u64, u64)> = vec![(20, 10, 5)];
    let translator = Translator::new(input);
    assert_eq!(translator.translate(12), 22);
    assert_eq!(translator.translate(40), 40);
}

#[test]
fn test_translate_chain() {
    let input1: Vec<(u64, u64, u64)> = vec![(20, 10, 5)];
    let input2: Vec<(u64, u64, u64)> = vec![(30, 20, 5)];
    let mut t1 = Translator::new(input1);
    let t2 = Translator::new(input2);
    t1.set_internal_translator(&t2);
    assert_eq!(t1.translate(11), 31);
    assert_eq!(t1.translate(9), 9);
}
