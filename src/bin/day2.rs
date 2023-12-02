use std::fs;

fn main() {
    let data = fs::read_to_string("data/day2").expect("Didn't find the file?");
    let lines = data.split('\n').filter(|x| x.len() > 0);
    for line in lines {
        println!("{}", line);
    }
}




