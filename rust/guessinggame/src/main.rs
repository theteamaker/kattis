use std::io::{self, BufRead};

// lol this is awful i haven't coded rust in months

fn main() {
    let stdin = io::stdin();
    let lines: Vec<String> = stdin.lock().lines().map(|x| x.unwrap()).collect();
    let mut groups: Vec<Vec<String>> = Vec::new();
    let mut to_push: Vec<String> = Vec::new();

    for line in lines {
        if line == "0" {
            break;
        }

        else if line == "right on" {
            to_push.push(line);
            groups.push(to_push);
            to_push = Vec::new();
            continue;
        }

        else {
            to_push.push(line);
            continue;
        }
    }

    for i in groups {
        let result = check(i);
        match result {
            false => println!("Stan is dishonest"),
            true => println!("Stan may be honest")
        }
    }
}

fn check(group: Vec<String>) -> bool {
    let mut answer: i32 = 0;
    let mut guesses: Vec::<(i32, String)> = Vec::new();

    for (index, response) in group.iter().enumerate() {
        let result: f64 = (index as f64) % 2.0; // if the line index is even, it's a numerical guess, if it's odd, it's a response to that guess.
        if result as i32 == 1 {
            let previous_index = index as i32 - 1;
            if response == "right on" {
                answer = group[previous_index as usize].parse::<i32>().unwrap();
            }

            else {
                guesses.push((group[previous_index as usize].parse::<i32>().unwrap(), response.to_string()));
            }
        }
    }
    let mut check = true;
    
    for tuple in guesses {
        if tuple.1 == "too high" {
            if tuple.0 > answer {
                continue;
            }
            else {
                check = false;
                break;
            }
        }
        
        else if tuple.1 == "too low" {
            if tuple.0 < answer {
                continue;
            }
            else {
                check = false;
                break;
            }
        }
    }
    check
}