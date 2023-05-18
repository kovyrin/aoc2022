use std::{fs::read_to_string, str::Lines};
use anyhow::{Context, Result};

#[derive(Debug)]
enum List {
    Single(usize),
    List(Vec<List>)
}

impl List {
    fn list_from_str(line: &str) -> (usize, List) {
        let mut list_vals = Vec::default();
        let chars: Vec<char> = line.chars().collect();
        let mut len = 1;
        loop {
            match chars[len] {
                c if c.is_numeric() => {
                    let parse_res = Self::number_from_str(&line[len..]);
                    len += parse_res.0;
                    list_vals.push(parse_res.1);
                },
                '[' => {
                    let parse_res = Self::list_from_str(&line[len..]);
                    len += parse_res.0;
                    list_vals.push(parse_res.1);
                },
                ',' => len += 1,
                ']' => {
                    len += 1;
                    break;
                }

                c => panic!("Unexpected character at pos {}: {}", len, c)
            }
        }
        (len, List::List(list_vals))
    }

    fn number_from_str(line: &str) -> (usize, List) {
        let mut len = line.len();
        for (i, c) in line.chars().enumerate() {
            if !c.is_numeric() {
                len = i;
                break
            }
        }
        let value = line[0..len].parse().expect("int parsing");
        (len, List::Single(value))
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

    loop {
        let line1 = lines.next().expect("line 1 read");
        let line2 = lines.next().expect("line 1 read");

        let list1 = List::list_from_str(line1);
        let list2 = List::list_from_str(line2);

        println!("Line 1: {:?}", line1);
        println!("List 1: {:?}", list1);

        println!("Line 2: {:?}", line2);
        println!("List 2: {:?}", list2);

        if let None = lines.next() { break }
    }

    Ok(())
}
