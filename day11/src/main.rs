use std::{fs::read_to_string, str::Lines};
use anyhow::{Result, Context};

#[derive(Debug)]
enum Operation {
    Add(u32),
    Mul(u32),
    Square,
}

#[derive(Debug)]
struct Monkey {
    items: Vec<u32>,
    op: Operation,
    test_div: u32,
    test_pass_dst: u32,
    test_fail_dst: u32,
}

impl Monkey {
    fn items_from_line(line: Option<&str>) -> Vec<u32> {
        line.expect("loading items")[18..].split(", ").map(|i| i.parse::<u32>().unwrap()).collect()
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
            s => panic!("unknown operator: {}", s)
        }
    }

    fn test_div_from_line(line: Option<&str>) -> u32 {
        let line = line.expect("parsing test");
        line[21..].parse().expect("parsing test divisor")
    }

    fn test_dst_from_line(line: Option<&str>) -> u32 {
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

        let test_success_str = lines.next().expect("loading test success");
        let test_fail_str = lines.next().expect("loading test success");

        Monkey { items, op, test_div, test_pass_dst, test_fail_dst }
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
    println!("Using input file: {}", input_file);

    let input: String = read_to_string(input_file).context("failed to read the data file")?;
    let mut lines: Lines = input.lines();

    let mut monkeys: Vec<Monkey> = Vec::new();

    while let Some(header) = lines.find(|l| l.starts_with("Monkey")) {
        let monkey = Monkey::from_lines(&mut lines);
        monkeys.push(monkey);
    }

    println!("Monkeys:");
    for monkey in monkeys.into_iter() {
        println!(" - {:?}", monkey);
    }

    Ok(())
}
