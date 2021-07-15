// Problem: https://www.hackerrank.com/challenges/strong-password/problem
// Time complexity: O(N)
// Space complexity: O(N)

use std::collections::HashSet;
use std::io;
use std::io::prelude::*;

static MIN_REQUIREMENTS: u8 = 4;
static SPECIAL_CHARACTERS: &str = "!@#$%^&*()-+";

fn has_digit(password: &str) -> bool {
    for c in password.chars() {
        if c.is_digit(10) {
            return true;
        }
    }

    false
}

fn has_lowercase(password: &str) -> bool {
    for c in password.chars() {
        if c.is_lowercase() {
            return true;
        }
    }

    false
}

fn has_uppercase(password: &str) -> bool {
    for c in password.chars() {
        if c.is_uppercase() {
            return true;
        }
    }

    false
}

fn has_special_character(password: &str) -> bool {
    let special_chars: HashSet<char> = SPECIAL_CHARACTERS.chars().into_iter().collect();

    for c in password.chars() {
        if special_chars.contains(&c) {
            return true;
        }
    }

    false
}

fn main() {
    let input: (u8, String) = read_input();

    let digit_count: u8 = has_digit(&input.1) as u8;
    let lowercase_count: u8 = has_lowercase(&input.1) as u8;
    let uppercase_count: u8 = has_uppercase(&input.1) as u8;
    let special_character_count: u8 = has_special_character(&input.1) as u8;

    let requirements_found: u8 =
        digit_count + lowercase_count + uppercase_count + special_character_count;
    let requirements_missed: u8 = MIN_REQUIREMENTS - requirements_found;

    if requirements_missed <= 1 && input.0 < 6 {
        println!("{}", 6 - input.0);
    } else {
        println!("{}", requirements_missed);
    }
}

fn read_input() -> (u8, String) {
    let reader = io::stdin();

    let pwd_length: u8 = reader
        .lock()
        .lines()
        .next()
        .unwrap()
        .ok()
        .unwrap()
        .trim()
        .parse()
        .unwrap();

    let mut pwd = String::new();
    reader.read_line(&mut pwd).ok().unwrap();

    (pwd_length, pwd.trim().to_string())
}
