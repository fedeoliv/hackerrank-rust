// Problem: https://www.hackerrank.com/challenges/insertionsort2/problem
// Time: O(N^2)
// Space: O(1)

use std::io;

fn insertion_sort(capacity: usize, items: &mut Vec<i16>) {
    for i in 1..capacity {
        for j in (0..i).rev() {
            if items[j] > items[j + 1] {
                let temp: i16 = items[j];
                items[j] = items[j + 1];
                items[j + 1] = temp;
            }
        }

        print_vec(capacity, items);
    }
}

fn print_vec(capacity: usize, items: &Vec<i16>) {
    for i in 0..capacity {
        print!("{} ", items[i]);
    }

    println!();
}

fn main() {
    let mut input: (u16, Vec<i16>) = read_input();
    insertion_sort(input.0 as usize, &mut input.1);
}

fn read_input() -> (u16, Vec<i16>) {
    let reader: io::Stdin = io::stdin();

    let capacity: u16 = read_line(&reader).parse().unwrap();
    let items: Vec<i16> = read_line(&reader)
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    (capacity, items)
}

fn read_line(reader: &io::Stdin) -> String {
    let mut buffer = String::new();
    reader.read_line(&mut buffer).ok().unwrap();

    buffer.trim().to_string()
}
