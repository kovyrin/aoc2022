use std::{fs::read_to_string, str::{Lines, Chars}, iter::Peekable, cmp::Ordering, fmt::Write};
use anyhow::{Context, Result};

#[derive(Debug, Clone, PartialEq, Eq)]
enum Packet {
    Single(usize),
    List(Vec<Packet>)
}

impl std::fmt::Display for Packet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Packet::List(items) => {
                f.write_char('[')?;
                let str = items.iter().map(|i| format!("{i}")).collect::<Vec<String>>().join(",");
                write!(f, "{str}")?;
                f.write_char(']')
            },
            Packet::Single(item) => write!(f, "{}", item),
        }
    }
}

impl Packet {
    fn from_str(line: &str) -> Packet {
        Self::list_from_str(&mut line.chars().peekable())
    }

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

    fn cmp(&self, right: &Self) -> Ordering {
        let left = self;
        match (left, right) {
            (Packet::Single(left), Packet::Single(right)) => {
                left.cmp(right)
            },
            (Packet::List(left), Packet::List(right)) => {
                return Self::compare_lists(left, right)
            },
            (Packet::List(_), Packet::Single(right)) => {
                let right = Packet::List(vec![Packet::Single(*right)]);
                return left.cmp(&right);
            },
            (Packet::Single(left), Packet::List(_)) => {
                let left = Packet::List(vec![Packet::Single(*left)]);
                return left.cmp(&right);
            },
        }
    }

    fn compare_lists(left: &Vec<Packet>, right: &Vec<Packet>) -> Ordering {
        for pos in 0..left.len() {
            if pos >= right.len() {
                return Ordering::Greater
            }
            let left = &left[pos];
            let right = &right[pos];
            let res = left.cmp(right);
            if !res.is_eq() { return res }
        }
        if left.len() == right.len() {
            Ordering::Equal
        } else {
            Ordering::Less
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
    let lines: Lines = input.lines();

    let mut packets: Vec<Packet> = Vec::default();

    let divider2 = Packet::from_str("[[2]]");
    let divider6 = Packet::from_str("[[6]]");
    packets.push(divider2.clone());
    packets.push(divider6.clone());

    for line in lines {
        if line.is_empty() { continue }
        let packet = Packet::from_str(line);
        packets.push(packet);
    }

    packets.sort_by(|a,b| a.cmp(b));

    let pos2 = packets.iter().position(|p| p.eq(&divider2) ).unwrap() + 1;
    let pos6 = packets.iter().position(|p| p.eq(&divider6) ).unwrap() + 1;

    println!("Positions for dividers: {pos2} and {pos6}");
    println!("Result: {}", pos2*pos6);

    Ok(())
}
