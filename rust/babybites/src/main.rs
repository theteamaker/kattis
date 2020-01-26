use::std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let lines: Vec<String> = stdin.lock().lines().map(|x| x.unwrap()).collect();

    let split_list: Vec<&str> = lines[1].split(" ").collect();

    let mut fishy = false;

    for i in 0..split_list.len() {
        let fruit = (i + 1).to_string();

        if split_list[i as usize] != fruit {
            if split_list[i as usize] != "mumble" {
                fishy = true;
            }
        }
    }

    match fishy {
        false => println!("makes sense"),
        true => println!("something is fishy"),
    }
}