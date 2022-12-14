//use std::error::Error;
use std::{io::Read, collections::HashMap};

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
    let tokens = input.split_ascii_whitespace();
    let mut respond_to_a = HashMap::new();
    let mut respond_to_b = HashMap::new();
    let mut respond_to_c = HashMap::new();

    respond_to_a.insert("X", 4); // 3 + 1
    respond_to_a.insert("Y", 8); // 6 + 2
    respond_to_a.insert("Z", 3); // 0 + 3

    respond_to_b.insert("X", 1); // 0 + 1
    respond_to_b.insert("Y", 5); // 3 + 2
    respond_to_b.insert("Z", 9); // 6 + 3

    respond_to_c.insert("X", 7); // 6 + 1
    respond_to_c.insert("Y", 2); // 0 + 2
    respond_to_c.insert("Z", 6); // 3 + 3

    let mut match_fn = &respond_to_a; // initialize it to something valid
    let mut val = 0i64;
    for tok in tokens {
        match tok {
            "A" => match_fn = &respond_to_a,
            "B" => match_fn = &respond_to_b,
            "C" => match_fn = &respond_to_c,
            _ => val += match_fn.get(tok).unwrap_or(&0),
        }
    }

    val
}

fn run_part2(input: &str) -> i64 {
    let tokens = input.split_ascii_whitespace();
    let mut respond_to_a = HashMap::new();
    let mut respond_to_b = HashMap::new();
    let mut respond_to_c = HashMap::new();

    respond_to_a.insert("Y", 4); // 3 + 1
    respond_to_a.insert("Z", 8); // 6 + 2
    respond_to_a.insert("X", 3); // 0 + 3

    respond_to_b.insert("X", 1); // 0 + 1
    respond_to_b.insert("Y", 5); // 3 + 2
    respond_to_b.insert("Z", 9); // 6 + 3

    respond_to_c.insert("Z", 7); // 6 + 1
    respond_to_c.insert("X", 2); // 0 + 2
    respond_to_c.insert("Y", 6); // 3 + 3

    let mut match_fn = &respond_to_a; // initialize it to something valid
    let mut val = 0i64;
    for tok in tokens {
        match tok {
            "A" => match_fn = &respond_to_a,
            "B" => match_fn = &respond_to_b,
            "C" => match_fn = &respond_to_c,
            _ => val += match_fn.get(tok).unwrap_or(&0),
        }
    }

    val
}
