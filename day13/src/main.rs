use std::{fs::read_to_string, str::{Lines, Chars}, iter::Peekable};
use anyhow::{Context, Result};

#[derive(Debug)]
enum List {
    Single(usize),
    List(Vec<List>)
}

impl List {
    fn list_from_str(line: &mut Peekable<Chars>) -> List {
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
        List::List(list_vals)
    }

    fn number_from_str(line: &mut Peekable<Chars>) -> List {
        let mut value = String::with_capacity(1);
        while let Some(c) = line.next() {
            if !c.is_numeric() { break }
            value.push(c);
        }
        let value = value.parse().expect("int parsing");
        List::Single(value)
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
        let line1 = lines.next().expect("reading line 1");
        let line2 = lines.next().expect("reading line 2");

        let list1 = List::list_from_str(&mut line1.chars().peekable());
        let list2 = List::list_from_str(&mut line2.chars().peekable());

        println!("Line 1: {:?}", line1);
        println!("List 1: {:?}", list1);

        println!("Line 2: {:?}", line2);
        println!("List 2: {:?}", list2);

        if let None = lines.next() { break }
    }

    Ok(())
}
