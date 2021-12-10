use std::collections::HashMap;

use lazy_static::lazy_static;

lazy_static! {
    static ref SCORES_1: HashMap<char, u32> = {
        let mut hm = HashMap::with_capacity(4);
        hm.insert(')', 3);
        hm.insert(']', 57);
        hm.insert('}', 1197);
        hm.insert('>', 25137);

        hm
    };
    static ref SCORES_2: HashMap<char, u64> = {
        let mut hm = HashMap::with_capacity(4);
        hm.insert(')', 1);
        hm.insert(']', 2);
        hm.insert('}', 3);
        hm.insert('>', 4);

        hm
    };
}

pub fn part1(input: &str) -> u32 {
    input
        .lines()
        .filter_map(|line| check(line.trim()).map(|c| SCORES_1[&c]))
        .sum()
}

pub fn part2(input: &str) -> u64 {
    let mut scores = input
        .lines()
        .filter(|line| check(line.trim()).is_none())
        .map(|line| {
            complet(line)
                .into_iter()
                .fold(0, |acc, c| acc * 5 + SCORES_2[&c])
        })
        .collect::<Vec<_>>();

    scores.sort_unstable();

    scores[scores.len() / 2]
}

fn check(line: &str) -> Option<char> {
    let mut tokens = vec![];

    for c in line.chars() {
        match c {
            '(' | '[' | '{' | '<' => tokens.push(c),
            c => {
                if let Some(last) = tokens.pop() {
                    if last != copain(c) {
                        return Some(c);
                    }
                } else {
                    return None;
                }
            }
        }
    }

    None
}

fn complet(line: &str) -> Vec<char> {
    let mut tokens = vec![];

    for c in line.chars() {
        match c {
            '(' | '[' | '{' | '<' => tokens.push(c),
            c => {
                if let Some(last) = tokens.last() {
                    if *last == copain(c) {
                        tokens.pop();
                    }
                }
            }
        }
    }

    tokens.into_iter().rev().map(copain).collect()
}

const fn copain(c: char) -> char {
    match c {
        ')' => '(',
        ']' => '[',
        '}' => '{',
        '>' => '<',
        '(' => ')',
        '[' => ']',
        '{' => '}',
        '<' => '>',
        _ => panic!(),
    }
}
