// Problem: https://www.hackerrank.com/challenges/sum-vs-xor/problem
// Time: O(n)
// Space: O(1)

use std::io;

fn count_zero_bits(mut n: u64) -> u64 {
    let mut result: u64 = 0;

    while n > 0 {
        result = result + (1 - (n & 1));
        n = n >> 1;
    }

    result
}

fn count_sum_vs_xor(n: u64) -> u64 {
    1 << count_zero_bits(n)
}

fn main() {
    let reader: io::Stdin = io::stdin();
    let n: u64 = read_line(&reader).parse().unwrap();
    let counter: u64 = count_sum_vs_xor(n);

    println!("{}", counter);
}

fn read_line(reader: &io::Stdin) -> String {
    let mut buffer = String::new();

    reader.read_line(&mut buffer).ok().unwrap();

    buffer.trim().to_string()
}
