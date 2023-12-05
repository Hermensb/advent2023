use std::{
    collections::{HashMap, HashSet},
    fs, vec,
};

fn main() {
    let data: String = fs::read_to_string("data/day5").expect("Didn't find the file?");
    let p1_result = part1(&data);
    println!("Part 1 Total {}", p1_result);

    let p2_result = part2(&data);
    println!("Part 2 Total {}", p2_result);
}

fn part1(input: &str) -> u64 {
    let lines: Vec<String> = input.split("\n\n").map(|x| x.to_string()).collect();

    let mut seeds: HashSet<u64> = extract_seeds(&lines[0]);
    let translators: Vec<Translator> = lines[1..]
        .iter()
        .map(|x| extract_translations(x))
        .map(|x| Translator::new(&x))
        .collect();
    for t in translators.iter() {
        seeds = seeds.iter().map(|x| t.translate(*x)).collect();
    }

    *seeds.iter().min().unwrap()
}

fn part2(input: &str) -> u32 {
    0
}

fn extract_seeds(input: &str) -> HashSet<u64> {
    let numbers: String = input.split(':').last().unwrap().to_string();
    let result: HashSet<u64> = numbers
        .split(" ")
        .filter(|x| x.len() > 0)
        .map(|x| x.trim())
        .map(|x| x.parse::<u64>().unwrap())
        .collect();
    result
}

fn extract_translations(input: &str) -> Vec<(u64, u64, u64)> {
    let mut result: Vec<(u64, u64, u64)> = vec![];
    let number_groups = input
        .split(':')
        .last()
        .unwrap()
        .split('\n')
        .filter(|x| x.len() > 0);
    for group in number_groups {
        let stuff: Vec<u64> = group
            .split(' ')
            .filter(|x| x.len() > 0)
            .map(|x| x.trim())
            .map(|x| x.parse().unwrap())
            .collect();

        result.push((stuff[0], stuff[1], stuff[2]));
    }
    result
}

struct Translator<'a> {
    translations: HashMap<u64, u64>,
    translator: Option<&'a Translator<'a>>,
}

impl<'a> Translator<'a> {
    fn new(input: &[(u64, u64, u64)]) -> Translator<'a> {
        let mut translations: HashMap<u64, u64> = HashMap::new();

        for (to, from, iterations) in input {
            for i in 0..*iterations {
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

#[allow(dead_code)]
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
    let input: [(u64, u64, u64); 1] = [(20, 10, 5)];
    let translator = Translator::new(&input);
    assert_eq!(translator.translate(12), 22);
    assert_eq!(translator.translate(40), 40);
}

#[test]
fn test_translate_chain() {
    let input1: [(u64, u64, u64); 1] = [(20, 10, 5)];
    let input2: [(u64, u64, u64); 1] = [(30, 20, 5)];
    let mut t1 = Translator::new(&input1);
    let t2 = Translator::new(&input2);
    t1.set_internal_translator(&t2);
    assert_eq!(t1.translate(11), 31);
    assert_eq!(t1.translate(9), 9);
}

#[test]
fn test_get_seeds() {
    let input: String = "seeds: 79 14 55 13".to_string();
    assert_eq!(extract_seeds(&input), HashSet::from([79, 14, 55, 13]));
}

#[test]
fn test_extract_translations() {
    let input: String = "blah-to-eh translations:
10 20 30
40 50 60"
        .to_string();
    assert_eq!(
        extract_translations(&input),
        vec![(10, 20, 30), (40, 50, 60)]
    );
}
