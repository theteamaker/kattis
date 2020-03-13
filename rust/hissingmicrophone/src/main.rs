use std::io::{self, BufRead};

fn main() {
    let mut line = String::new();
    let stdin = io::stdin();
    stdin.lock().read_line(&mut line).unwrap();
    let characters: Vec<char> = line.chars().collect();

    let mut status = "no hiss";
    for (index, character) in characters.iter().enumerate() {
        if index == 0 {
            continue;
        }

        if *character == 's' {
            if characters[index - 1] == 's' {
                status = "hiss";
                break;
            }
        }
    }

    println!("{}", status);
}
