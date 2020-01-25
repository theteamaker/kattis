use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let lines: Vec<String> = stdin.lock().lines().map(|x| x.unwrap()).collect();
    
    let letters = "abcdefghijklmnopqrstuvwxyz";

    for y in 1..lines.len() {
        let line = lines[y as usize].to_lowercase();

        let mut not_found: Vec<char> = Vec::new();

        for i in letters.chars() {
            match line.find(i) {
                None => not_found.push(i),
                Some(_i) => (),
            }
        }

        if not_found.len() == 0 {
            println!("pangram")
        }

        else if not_found.len() > 0 {
            let to_print: String = not_found.iter().collect();
            println!("missing {}", to_print);
        }
    }
}