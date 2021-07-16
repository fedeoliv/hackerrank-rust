// Problem: https://www.hackerrank.com/challenges/insertionsort1/problem
// Time: O(N^2)
// Space: O(1)

use std::io;

fn insertion_sort(capacity: usize, items: &mut Vec<i16>) {
    let item: i16 = items[capacity - 1];
    let mut order_changed: bool = false;

    for i in (0..(capacity - 1)).rev() {
        if items[i] < item {
            items[i + 1] = item;
            order_changed = true;

            print_arr(capacity, items);
            break;
        }

        items[i + 1] = items[i];
        print_arr(capacity, items);
    }

    if !order_changed {
        items[0] = item;
        print_arr(capacity, items);
    }
}

fn print_arr(capacity: usize, items: &Vec<i16>) {
    for i in 0..(capacity) {
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
    let numbers: Vec<i16> = read_line(&reader)
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    (capacity, numbers)
}

fn read_line(reader: &io::Stdin) -> String {
    let mut buffer = String::new();
    reader.read_line(&mut buffer).ok().unwrap();

    buffer.trim().to_string()
}
