use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();

    let mut lines = stdin.lock().lines();
    let line1 = lines.next().unwrap().unwrap();

    let numbers: Vec<&str> = line1.split(" ").collect();

    let mut modified_numbers: Vec<i32> = Vec::new();

    for number in numbers {
        let mut reversed_number = String::new();

        for i in 0..number.len() {
            let current_num = number.len() - 1 - i;

            let current_char = number.chars().nth(current_num).unwrap();
            reversed_number.push_str(&current_char.to_string());
        }

        let reversed_int = reversed_number.parse::<i32>().unwrap();

        modified_numbers.push(reversed_int);
    }

    let mut max = 0;

    for number in modified_numbers {
        if number > max {
            max = number;
        }
    }

    println!("{}", max);
}