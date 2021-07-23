// Problem: https://www.hackerrank.com/challenges/flipping-bits/problem
// Time: O(1)
// Space: O(1)

use std::io;

fn flip_bits(n: u32) -> u32 {
    !n
}

fn main() {
    let reader: io::Stdin = io::stdin();
    let q: u8 = read_line(&reader).parse().unwrap();

    for _ in 0..q {
        let n: u32 = read_line(&reader).parse().unwrap();
        let result: u32 = flip_bits(n);

        println!("{}", result);
    }
}

fn read_line(reader: &io::Stdin) -> String {
    let mut buffer = String::new();

    reader.read_line(&mut buffer).ok().unwrap();

    buffer.trim().to_string()
}
