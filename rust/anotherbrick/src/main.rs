use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let lines: Vec<String> = stdin.lock().lines().map(|x| x.unwrap()).collect();

    let first_line_str: Vec<&str> = lines[0].split(" ").collect();
    let first_line: Vec<i32> = first_line_str.iter().map(|x| x.parse::<i32>().unwrap()).collect();

    let second_line_str: Vec<&str> = lines[1].split(" ").collect();
    let mut second_line: Vec<i32> = second_line_str.iter().map(|x| x.parse::<i32>().unwrap()).collect();

    let height = first_line[0];
    let width = first_line[1];
    let bricks = first_line[2];

    let mut completable = false;

    'outer: for iteration in 0..height {	
        let mut current = 0;

        'inner: for y in 0..bricks {
            if second_line.len() != 0 {
                match current {
                    c if c < width => {
                        current = current + second_line[0];
                        second_line.remove(0);
                    }

                    c if c == width => {
                        match iteration {
                            x if x < height - 1 => continue 'outer,

                            x if x == height - 1 => {
                                completable = true;
                                break 'outer;
                            }

                            _ => println!("error!"),
                        }
                    }

                    c if c > width => {
                        break 'outer;
                    }

                    _ => println!("err!"),
                }
            }
        }
    }

    match completable {
        true => println!("YES"),
        false => println!("NO"),
    }
}