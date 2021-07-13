// Problem: https://www.hackerrank.com/challenges/camelcase/problem

use std::io;

fn count_camelcase_words(input: String) -> usize {
    input.chars().filter(|x| x.is_uppercase()).count() + 1
}

fn main() {
    let input: String = read_line();
    let words_count: usize = count_camelcase_words(input);
    println!("{}", words_count);
}

fn read_line() -> String {
    let mut buffer = String::new();
    let reader: io::Stdin = io::stdin();

    reader.read_line(&mut buffer).expect("Could not read stdin");

    buffer.trim().to_string()
}
