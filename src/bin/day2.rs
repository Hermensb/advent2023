use std::fs;

fn main() {
    let data = fs::read_to_string("data/day2").expect("Didn't find the file?");
    let lines = data.split('\n').filter(|x| x.len() > 0);
    for line in lines {
        println!("{}", line);
    }
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
        let samples = parts.last().expect("").split(';').collect::<Vec<&str>>();
        Game {
            number,
            samples: vec![(1, 2, 3), (4, 5, 6), (7, 8, 9)],
        }
    }
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
