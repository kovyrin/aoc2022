use std::{fs::read_to_string, str::Lines, collections::VecDeque};
use anyhow::{Result, Context};

#[derive(Debug)]
enum Operation {
    Add(u64),
    Mul(u64),
    Square,
}

impl Operation {
    fn run(&self, operand: u64) -> u64 {
        match self {
            Operation::Add(x) => operand + x,
            Operation::Mul(x) => operand * x,
            Operation::Square => operand * operand
        }
    }
}

#[derive(Debug)]
struct Monkey {
    items: VecDeque<u64>,
    op: Operation,
    test_div: u64,
    test_pass_dst: usize,
    test_fail_dst: usize,
}

impl Monkey {
    fn items_from_line(line: Option<&str>) -> VecDeque<u64> {
        line.expect("loading items")[18..].split(", ").map(|i| i.parse::<u64>().unwrap()).collect()
    }

    fn operation_from_line(line: Option<&str>) -> Operation {
        let mut op_str = line.expect("loading operation")[23..].split_whitespace();
        let operator = op_str.next().expect("extracting operator");
        let operand = op_str.next().expect("extracting operand");

        match operator {
            "+" => {
                let operand_int = operand.parse().expect("parsing operand for addition");
                Operation::Add(operand_int)
            },
            "*" => {
                if operand.eq("old") {
                    Operation::Square
                } else {
                    let operand_int = operand.parse().expect("parsing operand for addition");
                    Operation::Mul(operand_int)
                }
            },
            s => panic!("unknown operator: {s}")
        }
    }

    fn test_div_from_line(line: Option<&str>) -> u64 {
        let line = line.expect("parsing test");
        line[21..].parse().expect("parsing test divisor")
    }

    fn test_dst_from_line(line: Option<&str>) -> usize {
        let line = line.expect("parsing test result");
        line.split_whitespace().last().expect("loading monkey number")
            .parse().expect("parsing test throw monkey")
    }

    // Monkey 1:
    // Starting items: 54, 65, 75, 74
    // Operation: new = old + 6
    // Test: divisible by 19
    //   If true: throw to monkey 2
    //   If false: throw to monkey 0
    fn from_lines(lines: &mut Lines) -> Self {
        let items = Self::items_from_line(lines.next());
        let op = Self::operation_from_line(lines.next());
        let test_div = Self::test_div_from_line(lines.next());
        let test_pass_dst = Self::test_dst_from_line(lines.next());
        let test_fail_dst = Self::test_dst_from_line(lines.next());

        Monkey { items, op, test_div, test_pass_dst, test_fail_dst }
    }

    fn test(&self, value: u64) -> usize {
        if value % self.test_div == 0 {
            self.test_pass_dst
        } else {
            self.test_fail_dst
        }
    }
}

struct Game {
    monkeys: Vec<Monkey>,
    activity: Vec<u64>,
    modulus: u64,
}

impl Game {
    fn new() -> Self {
        Game {
            monkeys: Vec::new(),
            activity: Vec::new(),
            modulus: 1
        }
    }

    fn push(&mut self, monkey: Monkey) {
        self.modulus *= monkey.test_div;
        self.monkeys.push(monkey);
        self.activity.resize(self.monkeys.len(), 0);
    }

    fn print_monkeys(&self) {
        println!("Monkeys:");
        for monkey in self.monkeys.iter() {
            println!(" - {:?}", monkey);
        }
    }

    fn process_item(&mut self, monkey_idx: usize, item: u64) {
        self.activity[monkey_idx] += 1;
        let monkey = &self.monkeys[monkey_idx];
        let result = monkey.op.run(item);
        let dst = monkey.test(result);
        self.monkeys[dst].items.push_back(result % self.modulus);
    }

    fn round(&mut self) {
        for m in 0..self.monkeys.len() {
            let items = std::mem::take(&mut self.monkeys[m].items);
            for item in items {
                self.process_item(m, item);
            }
        }
    }
}

fn main() -> Result<()>{
    // If first argument is "real", use the real input file
    // Otherwise, use the test input file
    let input_file = if std::env::args().nth(1).unwrap_or(String::default()).eq("real") {
        "real-input.txt"
    } else {
        "demo-input.txt"
    };
    println!("Using input file: {input_file}");

    let input: String = read_to_string(input_file).context("failed to read the data file")?;
    let mut lines: Lines = input.lines();

    let mut game = Game::new();

    while let Some(_) = lines.find(|l| l.starts_with("Monkey")) {
        let monkey = Monkey::from_lines(&mut lines);
        game.push(monkey);
    }

    game.print_monkeys();
    for _round in 0..10000 {
        game.round();
    }

    game.activity.sort();
    game.activity.reverse();

    println!("Sorted activity: {:?}", game.activity);
    let monkey_business = game.activity[0] * game.activity[1];
    println!("Monkey business: {monkey_business}");

    Ok(())
}
