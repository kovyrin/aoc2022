use std::{fs::read_to_string, str::Lines, collections::HashMap};
use anyhow::Context;

const HUMAN: &str = "humn";

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

    fn operands(&self) -> (String, String) {
        let (l, r) = match self {
            Job::Add(l, r) => (l, r),
            Job::Sub(l, r) => (l, r),
            Job::Mul(l, r) => (l, r),
            Job::Div(l, r) => (l, r),
            Job::Value(_) => panic!("No operands in a value"),
        };

        (l.clone(), r.clone())
    }

    // Receives a Job f=self.run(op1, op2), returns a new Job, where op1 = new_job(f, op2)
    fn equation_for(&self, f: &String, op1: &String) -> Job {
        match self {
            Job::Add(l, r) if l == op1 => Job::Sub(f.clone(), r.clone()), // f=op1+r => op1=f-r
            Job::Add(l, r) if r == op1 => Job::Sub(f.clone(), l.clone()), // f=l+op1 => op1=f-l

            Job::Sub(l, r) if l == op1 => Job::Add(f.clone(), r.clone()), // f=op1-r => op1=f+r
            Job::Sub(l, r) if r == op1 => Job::Sub(l.clone(), f.clone()), // f=r-op1 => op1=r-f

            Job::Mul(l, r) if l == op1 => Job::Div(f.clone(), r.clone()), // f=op1*r => op1=f/r
            Job::Mul(l, r) if r == op1 => Job::Div(f.clone(), l.clone()), // f=l*op1 => op1=f/l

            Job::Div(l, r) if l == op1 => Job::Mul(f.clone(), r.clone()), // f=op1/r => op1=f*r
            Job::Div(l, r) if r == op1 => Job::Div(l.clone(), f.clone()), // f=r/op1 => op1=r/f

            _ => panic!("Cannot calculate {} from {}={:?}", op1, f, self),
        }
    }
}

fn human_dependent_jobs(name: &String, monkeys: &HashMap<String, Job>, result: &mut Vec<String>) -> bool {
    if name == HUMAN { return true }
    let job = monkeys.get(name).expect("fetching a monkey");

    let (l, r) = match job {
        Job::Value(_) => return false,
        Job::Add(l, r) => (l, r),
        Job::Sub(l, r) => (l, r),
        Job::Mul(l, r) => (l, r),
        Job::Div(l, r) => (l, r),
    };

    if human_dependent_jobs(l, monkeys, result){
        result.push(name.clone());
        return true;
    }

    if human_dependent_jobs(r, monkeys, result) {
        result.push(name.clone());
        return true;
    }

    false
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

    for line in lines {
        let (name, job) = parse_line(&line);
        monkeys.insert(name, job);
    }

    let root_name = String::from("root");
    let result = calculate_monkey(&root_name, &monkeys);
    println!("Root monkey yells {}", result);

    // Get both sides of the root
    let (left, right) = monkeys[&root_name].operands();

    // One of them will be a solvable side, the solution, and another one will have an unknown (human)
    let mut human_dep_jobs: Vec<String> = Vec::new();
    human_dependent_jobs(&right, &monkeys, &mut human_dep_jobs);
    let (mut solution, mut equation) = (&left, &right);
    if human_dep_jobs.is_empty() {
        human_dependent_jobs(&left, &monkeys, &mut human_dep_jobs);
        (solution, equation) = (&right, &left);
    }

    // Calculate the solution to get a value
    let solution = calculate_monkey(&solution, &monkeys);

    // Walk all nodes dependent on humn and invert them
    let mut eq_for = HUMAN.to_string();
    for name in human_dep_jobs.iter() {
        let job = monkeys.get(name).expect("fetching a monkey");
        let eq = job.equation_for(name, &eq_for); // returns a new job to calculate the value of eq_for
        monkeys.insert(eq_for.to_owned(), eq);
        eq_for = name.clone();
    }

    // Inject the solution into the dataset
    monkeys.insert(equation.clone(), Job::Value(solution));

    // Solve for the human value
    let human_val = calculate_monkey(&HUMAN.to_string(), &monkeys);
    println!("Human value: {}", human_val);
}
