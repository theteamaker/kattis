use std::io::{self, BufRead};
use std::collections::HashMap;

fn main() {
    let stdin = io::stdin();
    let lines: Vec<String> = stdin.lock().lines().map(|x| x.unwrap()).collect();

    let replacements: HashMap<&str, &str> = [
        ("Ab minor", "G# minor"), 
        ("A# minor", "Bb minor"), 
        ("Db minor", "C# minor"),
        ("D# minor", "Eb minor"),
        ("Gb minor", "F# minor"),
        ("A# major", "Bb major"),
        ("C# major", "Db major"),
        ("D# major", "Eb major"),
        ("Gb major", "F# major"),
        ("G# major", "Ab major")
        ].iter().cloned().collect();
    
    let mut counter = 1;

    for line in lines {
        let str_line: &str = &line[..];

        match replacements.get(str_line) {
            Option::None => {
                match replacements.iter().find_map(|(key, &val)| if val == str_line { Some(key) } else { None }) {
                Option::None => println!("Case {}: UNIQUE", counter),
                Option::Some(i) => println!("Case {}: {}", counter, i),
                }
            },

            Option::Some(i) => println!("Case {}: {}", counter, i),
        }

        counter = counter + 1;
    }
}