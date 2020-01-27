use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let lines: Vec<String> = stdin.lock().lines().map(|x| x.unwrap()).collect();

    let mut found: Vec<i32> = Vec::new();

    for x in 0..lines.len() {
        let current = &lines[x as usize];

        match current.find("FBI") {
            Some(_y) => found.push((x + 1) as i32),
            None => (),
        }
    }

    match found.len() == 0 {

    false => {
            for find in found {
            print!("{} ", find);
        }
    },

    true => print!("HE GOT AWAY!"),

    }
}
