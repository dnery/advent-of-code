//use std::error::Error;
use std::io::Read;

fn main() {
    let mut buffer = Vec::new();
    std::io::stdin().read_to_end(&mut buffer).unwrap();
    let input = std::str::from_utf8(&buffer).unwrap_or("");

    let r1 = run_part1(input);
    let r2 = run_part2(input);
    println!("{}", r1);
    println!("{}", r2);
}

fn run_part1(input: &str) -> i64 {
    let mut tmp = 0i64;
    let mut val = 0i64;
    let lines = input.split("\n");

    for line in lines {
        if line.is_empty() {
            val = if tmp > val { tmp } else { val };
            tmp = 0;
        }
        tmp += line.parse::<i64>().unwrap_or(0);
    }

    val
}

fn run_part2(input: &str) -> i64 {
    let mut tmp = 0i64;
    let mut vals: Vec<i64> = vec![];
    let lines = input.split("\n");

    for line in lines {
        if line.is_empty() {
            vals.push(tmp);
            tmp = 0;
        }
        tmp += line.parse::<i64>().unwrap_or(0);
    }

    vals.sort_by(|a, b| b.cmp(a));
    vals.into_iter()
        .take(3)
        .reduce(|acc, v| acc + v)
        .unwrap_or(0)
}
