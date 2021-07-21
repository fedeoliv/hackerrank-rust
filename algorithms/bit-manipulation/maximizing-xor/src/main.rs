// Problem: https://www.hackerrank.com/challenges/maximizing-xor/problem
// Time: O(N*M); N = l and M = r
// Space: O(1)

use std::io;

fn find_max_xor(l: u16, r: u16) -> u16 {
    let mut max_xor: u16 = 0;

    if l == r {
        return max_xor;
    }

    for i in l..(r + 1) {
        for j in (i + 1)..(r + 1) {
            let result: u16 = i ^ j;

            if result > max_xor {
                max_xor = result;
            }
        }
    }

    max_xor
}

fn main() {
    let reader: io::Stdin = io::stdin();

    let l: u16 = read_input(&reader).parse().unwrap();
    let r: u16 = read_input(&reader).parse().unwrap();

    let max_xor: u16 = find_max_xor(l, r);
    println!("{}", max_xor);
}

fn read_input(reader: &io::Stdin) -> String {
    let mut buffer = String::new();

    reader.read_line(&mut buffer).ok().unwrap();

    buffer.trim().to_string()
}
