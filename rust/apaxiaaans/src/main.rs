use std::io::{self, BufRead};

fn main() {
    let mut line = String::new();
    let stdin = io::stdin();
    stdin.lock().read_line(&mut line).unwrap();

    let characters: Vec<char> = line.chars().collect();

    let mut new_string = String::new();

    for (index, character) in characters.iter().enumerate() {
        if index == 0 {
            new_string.push(*character);
            continue;
        }
        if *character != characters[index - 1] {
            new_string.push(*character);
        }
    }   
    println!("{}", new_string);
}