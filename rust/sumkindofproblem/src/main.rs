use::std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let lines: Vec<String> = stdin.lock().lines().map(|x| x.unwrap()).collect();
    let mut cases = lines.iter();
    cases.next(); // ignore the first line, irrelevant

    for case in cases {
        let split_data: Vec<&str> = case.split(" ").collect();
        
        let case_num = split_data[0];
        let integer = split_data[1].parse::<i32>().unwrap();

        let s1 = sum_calc(integer, 1, 1);
        let s2 = sum_calc(integer, 1, 2);
        let s3 = sum_calc(integer, 2, 2);

        println!("{} {} {} {}", case_num, s1, s2, s3);
    }
}

fn sum_calc(range: i32, initial_value: i32, increment: i32) -> i32 {
    let mut current = initial_value;
    let mut sum = current;

    for _number in 1..range {
        current = current + increment;
        sum = sum + current;
    }
    sum
}