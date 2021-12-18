use std::collections::HashMap;

pub fn part1(input: &str) -> u32 {
    let (mut polymer, rules) = global(input);

    for _ in 0..10 {
        let mut new_polymer = Vec::with_capacity(polymer.len() * 2 - 1);

        new_polymer.push(polymer[0]);
        for from in polymer.windows(2) {
            let to = rules[from];

            new_polymer.push(to);
            new_polymer.push(from[1]);
        }

        polymer = new_polymer;
    }

    let mut elements = HashMap::new();
    for el in polymer {
        *elements.entry(el).or_insert(0) += 1
    }

    let (min, max) = elements
        .values()
        .fold((u32::MAX, 0), |(acc_min, acc_max), c| {
            (acc_min.min(*c), acc_max.max(*c))
        });

    max - min
}

pub fn part2(input: &str) -> u64 {
    let (polymer, rules) = global(input);

    let mut elements = HashMap::new();
    for atom in polymer.iter() {
        *elements.entry(*atom).or_insert(0) += 1
    }

    let mut polymer_pat = HashMap::with_capacity(rules.len());
    for pat in polymer.windows(2) {
        *polymer_pat.entry(pat.to_vec()).or_insert(0) += 1
    }

    for _ in 0..40 {
        let mut new_polymer_pat = HashMap::with_capacity(polymer_pat.len());

        for (pat, redundancy) in polymer_pat.iter() {
            let atom = rules[pat];
            *elements.entry(atom).or_insert(0) += redundancy;

            let mut left = vec![pat[0]];
            left.push(atom);

            let mut right = vec![atom];
            right.push(pat[1]);

            *new_polymer_pat.entry(left).or_insert(0) += redundancy;
            *new_polymer_pat.entry(right).or_insert(0) += redundancy;
        }

        polymer_pat = new_polymer_pat
    }

    let (min, max) = elements
        .values()
        .fold((u64::MAX, 0), |(acc_min, acc_max), c| {
            (acc_min.min(*c), acc_max.max(*c))
        });

    max - min
}

fn global(input: &str) -> (Vec<char>, HashMap<Vec<char>, char>) {
    let mut input = input.trim().split("\r\n\r\n");

    let polymer = input.next().unwrap().trim().chars().collect::<Vec<char>>();

    let rules = input
        .next()
        .unwrap()
        .lines()
        .map(|line| {
            let (from, to) = line.split_once(" -> ").unwrap();
            (from.chars().collect::<Vec<_>>(), to.chars().next().unwrap())
        })
        .collect::<HashMap<Vec<char>, char>>();

    (polymer, rules)
}
