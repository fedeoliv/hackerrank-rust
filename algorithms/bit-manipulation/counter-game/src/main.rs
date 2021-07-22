// Problem: https://www.hackerrank.com/challenges/counter-game/problem
// Time: O(n)
// Space: O(1)

use std::io;

const LOUISE: &'static str = "Louise";
const RICHARD: &'static str = "Richard";

fn reset_bit(number: u64, left_position: u8) -> u64 {
    number & !(1 << left_position)
}

fn find_lower_power_of_two(mut number: u64) -> u64 {
    let mut left_position: u8 = 0;

    while !is_power_of_two(number) {
        number = reset_bit(number, left_position);
        left_position = left_position + 1;
    }

    number
}

fn is_power_of_two(number: u64) -> bool {
    number & (number - 1) == 0
}

fn find_winner(mut number: u64) -> String {
    if number == 1 {
        return RICHARD.to_string();
    }

    let mut current_player: bool = true;
    let mut winner: bool = false;

    while number > 1 {
        number = match is_power_of_two(number) {
            true => number / 2,
            false => number - find_lower_power_of_two(number),
        };

        match number {
            1 => winner = current_player,
            _ => current_player = !current_player,
        }
    }

    let winner_name: String = match winner {
        true => LOUISE.to_string(),
        false => RICHARD.to_string(),
    };

    winner_name
}

fn main() {
    let reader: io::Stdin = io::stdin();
    let test_cases: u8 = read_line(&reader).parse().unwrap();

    for _ in 0..test_cases {
        let n: u64 = read_line(&reader).parse().unwrap();
        println!("{}", find_winner(n));
    }
}

fn read_line(reader: &io::Stdin) -> String {
    let mut buffer = String::new();

    reader.read_line(&mut buffer).ok().unwrap();

    buffer.trim().to_string()
}
