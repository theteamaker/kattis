use std::io::{self, BufRead};

fn main() {
    let mut line = String::new();
    let stdin = io::stdin();
    stdin.lock().read_line(&mut line).unwrap();

    let split_line: Vec<&str> = line.split(" ").collect();

    let fizz = split_line[0].parse::<i32>().unwrap();
    let buzz = split_line[1].parse::<i32>().unwrap();
    let number = split_line[2].trim().parse::<i32>().unwrap();

    for i in 1..number + 1 {
        let result = fizzbuzz(i, fizz, buzz);
        
        if result.len() == 0 {
            println!("{}", i);
            continue;
        }

        else {
            for i in result {
                print!("{}", i);
            }
            print!("\n");
        }
    }
}

fn is_divisible_by(number: i32, divisor: i32) -> bool {
    let result = number % divisor;
    let check: bool;

    if result == 0 {
        check = true
    }
    else {
        check = false
    }

    check
}

fn fizzbuzz(number: i32, fizz: i32, buzz: i32) -> Vec<String> {
    let mut return_vec: Vec<String> = Vec::new();

    if is_divisible_by(number, fizz) == true {
        return_vec.push("Fizz".to_string());
    }

    if is_divisible_by(number, buzz) == true {
        return_vec.push("Buzz".to_string());
    }

    return_vec
}