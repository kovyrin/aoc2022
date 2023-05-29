use std::fs::read_to_string;
use anyhow::Context;


fn snafu_to_int(s: &str) -> i64 {
    let mut res: i64 = 0;
    for (pos, c) in s.chars().rev().enumerate() {
        res += snafu_digit_to_int(c) * 5_i64.pow(pos as u32);
    }
    res
}

fn snafu_digit_to_int(c: char) -> i64 {
    match c {
        '2' => 2,
        '1' => 1,
        '0' => 0,
        '-' => -1,
        '=' => -2,
        _ => panic!("Invalid snafu digit: {}", c),
    }
}

fn int_to_snafu(mut n: i64) -> String {
    let mut res = String::new();
    let mut overflow = 0;
    let mut snafu_digit: char;

    while n != 0 {
        (overflow, snafu_digit) = int_to_snafu_digit(n % 5);
        res.push(snafu_digit);
        n = n / 5 + overflow;
    }
    if overflow != 0 { res.push('1')  }
    res.chars().rev().collect()
}

fn int_to_snafu_digit(digit: i64) -> (i64, char) {
    match digit {
        0 => (0, '0'),
        1 => (0, '1'),
        2 => (0, '2'),
        3 => (1, '='),
        4 => (1, '-'),
        _ => panic!("Invalid integer for converting to snafu: {}", digit),
    }
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
    let mut sum = 0_i64;
    for line in input.lines() {
        let n = snafu_to_int(line);
        println!("{} -> {}", line, n);
        sum += n;
    }
    println!("Sum: {}", sum);
    println!("Sum in snafu: {}", int_to_snafu(sum));
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn snafu_to_int_test() {
        assert_eq!(snafu_to_int("1"), 1);
        assert_eq!(snafu_to_int("2"), 2);
        assert_eq!(snafu_to_int("1="), 3);
        assert_eq!(snafu_to_int("1-"), 4);
        assert_eq!(snafu_to_int("10"), 5);
        assert_eq!(snafu_to_int("11"), 6);
        assert_eq!(snafu_to_int("12"), 7);
        assert_eq!(snafu_to_int("2="), 8);
        assert_eq!(snafu_to_int("2-"), 9);
        assert_eq!(snafu_to_int("20"), 10);
        assert_eq!(snafu_to_int("1=0"), 15);
        assert_eq!(snafu_to_int("1-0"), 20);
        assert_eq!(snafu_to_int("1=11-2"), 2022);
        assert_eq!(snafu_to_int("1-0---0"), 12345);
        assert_eq!(snafu_to_int("1121-1110-1=0"), 314159265);
    }

    #[test]
    fn int_to_snafu_test() {
        assert_eq!(int_to_snafu(1), "1");
        assert_eq!(int_to_snafu(2), "2");
        assert_eq!(int_to_snafu(3), "1=");
        assert_eq!(int_to_snafu(4), "1-");
        assert_eq!(int_to_snafu(5), "10");
        assert_eq!(int_to_snafu(6), "11");
        assert_eq!(int_to_snafu(7), "12");
        assert_eq!(int_to_snafu(8), "2=");
        assert_eq!(int_to_snafu(9), "2-");
        assert_eq!(int_to_snafu(10), "20");
        assert_eq!(int_to_snafu(15), "1=0");
        assert_eq!(int_to_snafu(20), "1-0");
        assert_eq!(int_to_snafu(2022), "1=11-2");
        assert_eq!(int_to_snafu(12345), "1-0---0");
        assert_eq!(int_to_snafu(314159265), "1121-1110-1=0");
    }

}
