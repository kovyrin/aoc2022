use std::{fs::read_to_string, str::Lines, collections::HashMap};
use anyhow::Context;

#[derive(Debug)]
enum Job {
    Value(i64),
    Add(String, String),
    Sub(String, String),
    Mul(String, String),
    Div(String, String),
}
impl Job {
    fn from_str(line: &str) -> Job {
        if line.chars().next().unwrap().is_numeric() {
            return Job::Value(line.parse().expect("Parsing a value"))
        }

        let mut parts = line.split_whitespace();

        let name1 = parts.next().expect("op1 capture").to_string();
        let op = parts.next().expect("op capture").to_string();
        let name2 = parts.next().expect("op2 capture").to_string();

        match op.chars().next() {
            Some('+') => Job::Add(name1, name2),
            Some('-') => Job::Sub(name1, name2),
            Some('*') => Job::Mul(name1, name2),
            Some('/') => Job::Div(name1, name2),
            Some(c) => panic!("Unexpected op: {}", c),
            None => panic!("Cannot get an op")
        }
    }
}

fn parse_line(line: &str) -> (String, Job) {
    let name = line[0..4].to_string();
    let job = Job::from_str(&line[6..]);
    (name, job)
}

fn main() {
    // If first argument is "real", use the real input file
    // Otherwise, use the test input file
    let input_type = std::env::args().nth(1).unwrap_or(String::default());
    let input_file = if input_type.eq("real") {
        "real-input.txt"
    } else {
        "demo-input.txt"
    };
    println!("Using input file: {}", input_file);

    let input: String = read_to_string(input_file).context("failed to read the data file").unwrap();
    let lines: Lines = input.lines();
    let mut monkeys: HashMap<String, Job> = HashMap::new();

    println!("Loading data...");
    for line in lines {
        let (name, job) = parse_line(&line);
        monkeys.insert(name, job);
    }

    println!("Calculating the result...");
    let root_name = String::from("root");
    let result = calculate_monkey(&root_name, &monkeys);
    println!("Root monkey yells {}", result);
}

fn calculate_monkey(name: &String, monkeys: &HashMap<String, Job>) -> i64 {
    let job = monkeys.get(name).expect("fetching a monkey");
    match job {
        Job::Value(v) => *v,
        Job::Add(m1, m2) => calculate_monkey(m1, monkeys) + calculate_monkey(m2, monkeys),
        Job::Sub(m1, m2) => calculate_monkey(m1, monkeys) - calculate_monkey(m2, monkeys),
        Job::Mul(m1, m2) => calculate_monkey(m1, monkeys) * calculate_monkey(m2, monkeys),
        Job::Div(m1, m2) => calculate_monkey(m1, monkeys) / calculate_monkey(m2, monkeys),
    }
}
