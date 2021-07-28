// Problem: https://www.hackerrank.com/challenges/yet-another-minimax-problem/problem
// Time: O(N^2)
// Space: O(N)

use std::io;

fn to_binary_vec(items: &Vec<u32>, n: usize) -> Vec<String> {
    let mut bin_vec: Vec<String> = Vec::with_capacity(n);

    for item in items {
        let bin_item: String = format!("{:b}", item);
        bin_vec.push(bin_item);
    }

    bin_vec
}

fn max_bin_len(bin_vec: &Vec<String>) -> usize {
    bin_vec.iter().max_by_key(|x| x.len()).unwrap().len()
}

fn min_bin_len(bin_vec: &Vec<String>) -> usize {
    bin_vec.iter().min_by_key(|x| x.len()).unwrap().len()
}

fn trim(bin_vec: &mut Vec<String>, n: usize) {
    while max_bin_len(&bin_vec) == min_bin_len(&bin_vec) {
        for i in 0..n {
            bin_vec[i].remove(0);
            bin_vec[i] = bin_vec[i].trim_start_matches('0').to_string();
        }
    }
}

fn normalize_and_sort(bin_vec: &mut Vec<String>, n: usize) {
    let max_bin_len: usize = max_bin_len(&bin_vec);

    for i in 0..n {
        let leading_zeros: String = "0".repeat(max_bin_len - bin_vec[i].len());
        let mut normalized_bin = String::new();

        normalized_bin.push_str(&leading_zeros);
        normalized_bin.push_str(&bin_vec[i]);

        bin_vec[i] = normalized_bin;
    }

    bin_vec.sort();
}

fn to_decimal(bin_item: &String) -> u32 {
    u32::from_str_radix(&bin_item, 2).unwrap()
}

fn get_score(items: &Vec<u32>, n: usize) -> u32 {
    let max_item: u32 = *items.iter().max().unwrap();
    let min_item: u32 = *items.iter().min().unwrap();

    if max_item == 0 && (max_item == min_item) {
        return 0;
    }

    let mut score: u32 = std::u32::MAX;
    let mut bin_vec: Vec<String> = to_binary_vec(items, n);

    trim(&mut bin_vec, n);
    normalize_and_sort(&mut bin_vec, n);

    let mut vec_zeros: Vec<String> = Vec::new();
    let mut vec_ones: Vec<String> = Vec::new();

    for bin_item in bin_vec {
        match bin_item.chars().nth(0).unwrap() {
            '0' => vec_zeros.push(bin_item),
            _ => vec_ones.push(bin_item),
        }
    }

    for i in &vec_zeros {
        for j in &vec_ones {
            let result: u32 = to_decimal(&i) ^ to_decimal(&j);

            if result < score {
                score = result;
            }
        }
    }

    score
}

fn main() {
    let reader: io::Stdin = io::stdin();
    let n: usize = read_line(&reader).parse().unwrap();
    let mut items: Vec<u32> = Vec::with_capacity(n);

    for item in read_line(&reader).split_whitespace() {
        items.push(item.parse().unwrap());
    }

    let score = get_score(&items, n);
    println!("{}", score);
}

fn read_line(reader: &io::Stdin) -> String {
    let mut buffer = String::new();

    reader.read_line(&mut buffer).ok().unwrap();

    buffer.trim().to_string()
}
