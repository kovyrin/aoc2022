use std::{fs::read_to_string, str::{Lines, Chars}, iter::Peekable, cmp::Ordering, fmt::Write};
use anyhow::{Context, Result};

#[derive(Debug)]
enum Packet {
    Single(usize),
    List(Vec<Packet>)
}

impl std::fmt::Display for Packet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Packet::List(items) => {
                f.write_char('[')?;
                let str = items.iter().map(|i| format!("{i}")).collect::<Vec<String>>().join("");
                write!(f, "{str}")?;
                f.write_char(']')
            },
            Packet::Single(item) => write!(f, "{}", item),
        }
    }
}

impl Packet {
    fn list_from_str(line: &mut Peekable<Chars>) -> Packet {
        let mut list_vals = Vec::default();
        while let Some(c) = line.peek() {
            match c {
                c if c.is_numeric() => {
                    let value = Self::number_from_str(line);
                    list_vals.push(value);
                },
                '[' => {
                    line.next();
                    let value = Self::list_from_str(line);
                    list_vals.push(value);
                },
                ']' => {
                    line.next();
                    break;
                }
                _ => {
                    line.next();
                }
            }
        }
        Packet::List(list_vals)
    }

    fn number_from_str(line: &mut Peekable<Chars>) -> Packet {
        let value: String = line.by_ref().take_while(|c| c.is_digit(10)).collect();
        Packet::Single(value.parse().expect("int parsing"))
    }

    fn smaller_than(&self, other: &Packet) -> bool {
        match (self, other) {
            (Packet::List(right), Packet::List(left)) => {
                self::compare_lists(right, left).is_lt()
            },
            _ => false
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
    let mut pair_index = 1;
    let mut index_sum = 0;

    loop {
        println!("====== Pair {pair_index} ======");

        let line1 = lines.next().expect("reading line 1");
        let line2 = lines.next().expect("reading line 2");
        println!("- {}", line1);
        println!("- {}", line2);

        let line1 = Packet::list_from_str(&mut line1.chars().peekable());
        let line2 = Packet::list_from_str(&mut line2.chars().peekable());


        if line1.smaller_than(&line2) {
            println!("+ Ordering is correct");
            index_sum += pair_index;
        } else {
            println!("- Ordering is incorrect");
        }

        if let None = lines.next() { break }
        pair_index += 1;
        println!();
    }

    println!("Sum of list indexes that are in correct order: {index_sum}");
    Ok(())
}

fn compare_one_item(left: &Packet, right: &Packet) -> Ordering {
    println!("Compare {} vs {}", left, right);
    match (left, right) {
        (Packet::Single(left), Packet::Single(right)) => {
            left.cmp(right)
        },
        (Packet::List(left), Packet::List(right)) => {
            return compare_lists(left, right)
        },
        (Packet::List(_), Packet::Single(right)) => {
            let right = Packet::List(vec![Packet::Single(*right)]);
            return compare_one_item(left, &right)
        },
        (Packet::Single(left), Packet::List(_)) => {
            let left = Packet::List(vec![Packet::Single(*left)]);
            return compare_one_item(&left, right)
        },
    }
}

fn compare_lists(left: &Vec<Packet>, right: &Vec<Packet>) -> Ordering {
    for pos in 0..left.len() {
        if pos >= right.len() {
            println!("* Right ran out of items");
            return Ordering::Greater
        }
        let left = &left[pos];
        let right = &right[pos];
        let res = compare_one_item(left, right);
        if !res.is_eq() { return res }
    }
    if left.len() == right.len() {
        return Ordering::Equal
    } else {
        println!("* Left ran out of items");
        Ordering::Less
    }
}
