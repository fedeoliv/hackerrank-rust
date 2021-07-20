// Problem: https://www.hackerrank.com/challenges/lonely-integer/problem
// Time: O(N)
// Space: O(1)

use std::io;

fn find_lonely_integer(values: &Vec<u8>) -> u8 {
    if values.capacity() == 1 {
        return values[0];
    }

    let mut result: u8 = 0;

    for value in values {
        result = result ^ value;
    }

    result
}

fn main() {
    let values: Vec<u8> = read_input();
    let result: u8 = find_lonely_integer(&values);

    println!("{}", result);
}

fn read_input() -> Vec<u8> {
    let reader: io::Stdin = io::stdin();

    let capacity: usize = read_line(&reader).parse().unwrap();
    let mut values: Vec<u8> = Vec::with_capacity(capacity);

    for item in read_line(&reader).split_whitespace() {
        values.push(item.parse().unwrap());
    }

    values
}

fn read_line(reader: &io::Stdin) -> String {
    let mut buffer = String::new();
    reader.read_line(&mut buffer).ok().unwrap();

    return buffer.trim().to_string();
}
