use std::fs;

fn main() {
    let data = fs::read_to_string("data/day2").expect("Didn't find the file?");
    let lines = data.split('\n').filter(|x| x.len() > 0);
    let games = lines.map(|x| Game::new(x)).map(|x| x.get_power());
    let total: u32 = games.sum();
    print!("Total {}", total);
}

#[derive(Debug, PartialEq)]
struct Game {
    pub number: u32,
    pub samples: Vec<(u8, u8, u8)>,
}

impl Game {
    pub fn new(input: &str) -> Game {
        let parts: Vec<&str> = input.split(':').collect();
        let number = parts
            .first()
            .expect("")
            .chars()
            .filter(|x| x.is_ascii_digit())
            .collect::<String>()
            .parse::<u32>()
            .expect("");
        let samples: Vec<(u8, u8, u8)> = parts
            .last()
            .expect("")
            .split(';')
            .map(|x| extract_rgb(x))
            .collect();
        Game { number, samples }
    }

    pub fn check_if_impossible(&self, max_vals: (u8, u8, u8)) -> bool {
        for vals in &self.samples {
            if vals.0 > max_vals.0 {
                return false;
            }
            if vals.1 > max_vals.1 {
                return false;
            }
            if vals.2 > max_vals.2 {
                return false;
            }
        }
        true
    }

    fn get_fewest_possible(&self) -> (u8, u8, u8) {
        let min_red = &self.samples.iter().map(|x| x.0).max().expect("");
        let min_green = &self.samples.iter().map(|x| x.1).max().expect("");
        let min_blue = &self.samples.iter().map(|x| x.2).max().expect("");
        (*min_red, *min_green, *min_blue)
    }

    pub fn get_power(&self) -> u32 {
        let (a, b, c) = self.get_fewest_possible();
        a as u32 * b as u32 * c as u32
    }
}

fn extract_rgb(input: &str) -> (u8, u8, u8) {
    let pieces: Vec<&str> = input.split(',').collect();
    let mut red: u8 = 0;
    let mut blue: u8 = 0;
    let mut green: u8 = 0;

    for piece in pieces {
        let portions: Vec<&str> = piece.split(" ").filter(|x| x.len() > 0).collect();
        let number = portions.first().expect("").parse::<u8>().expect("");
        let color = portions.last().expect("");
        match *color {
            "red" => red = number,
            "blue" => blue = number,
            "green" => green = number,
            _ => panic!(),
        }
    }
    (red, green, blue)
}

#[test]
fn test_new_game() {
    let game = Game::new("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green");
    assert_eq!(
        game,
        Game {
            number: 1,
            samples: vec![(4, 0, 3), (1, 2, 6), (0, 2, 0)]
        }
    );
}

#[test]
fn test_get_tuple() {
    assert_eq!((1, 2, 3), extract_rgb(" 2 green, 1 red, 3 blue"));
}

#[test]
fn test_possible() {
    let game = Game::new("Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue");
    assert!(game.check_if_impossible((12, 13, 14)));
}

#[test]
fn test_impossible() {
    let game =
        Game::new("Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red");
    assert!(!game.check_if_impossible((12, 13, 14)));
}

#[test]
fn test_min_possible() {
    let game = Game::new("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green");
    assert_eq!(game.get_fewest_possible(), (4, 2, 6));
}
