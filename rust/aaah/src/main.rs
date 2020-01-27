use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let lines: Vec<String> = stdin.lock().lines().map(|x| x.unwrap()).collect();

    match lines[0].len() < lines[1].len() {
        true => println!("no"),
        false => println!("go"),
    }
}
