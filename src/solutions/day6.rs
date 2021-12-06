use std::{cmp::Ordering, collections::HashMap};

use lazy_static::lazy_static;

pub fn part1(input: &str) -> u32 {
    let mut fishs = input
        .trim()
        .split(',')
        .map(|n| n.parse::<u8>().unwrap())
        .collect::<Vec<_>>();

    let mut to_add = 0;
    for _ in 0..80 {
        for fish in fishs.iter_mut() {
            if *fish == 0 {
                to_add += 1;
                *fish = 6
            } else {
                *fish -= 1;
            }
        }
        fishs.extend((0..to_add).map(|_| 8));
        to_add = 0
    }

    fishs.len() as u32
}

pub fn part2(input: &str) -> u64 {
    let mut fishs = HashMap::new();

    input
        .trim()
        .split(',')
        .map(|n| {
            let fish = n.parse::<u8>().unwrap();
            *fishs.entry(fish).or_insert_with(|| get_fishs(fish, 256))
        })
        .sum()
}

lazy_static! {
    static ref CALCULATE: HashMap<u8, (Vec<u8>, u64)> = {
        let mut fishs = HashMap::new();
        fishs.insert(0, pre_calculate(0));
        fishs.insert(1, pre_calculate(1));
        fishs.insert(2, pre_calculate(2));
        fishs.insert(3, pre_calculate(3));
        fishs.insert(4, pre_calculate(4));
        fishs.insert(5, pre_calculate(5));
        fishs.insert(6, pre_calculate(6));
        fishs.insert(7, pre_calculate(7));
        fishs.insert(8, pre_calculate(8));

        fishs
    };
}

fn pre_calculate(days_starting: u8) -> (Vec<u8>, u64) {
    let mut fishs = vec![days_starting];

    let mut to_add = 0;
    for _ in 0..64 {
        for fish in fishs.iter_mut() {
            if *fish == 0 {
                to_add += 1;
                *fish = 6
            } else {
                *fish -= 1;
            }
        }
        fishs.extend((0..to_add).map(|_| 8));
        to_add = 0
    }

    let len = fishs.len() as u64;
    (fishs, len)
}

fn get_fishs(fish: u8, day_to_go: u32) -> u64 {
    match day_to_go.cmp(&64) {
        Ordering::Greater => CALCULATE
            .get(&fish)
            .unwrap()
            .0
            .iter()
            .map(|fish| get_fishs(*fish, day_to_go - 64))
            .sum::<u64>(),
        Ordering::Equal => CALCULATE.get(&fish).unwrap().1,
        Ordering::Less => {
            let mut fishs = vec![fish];

            let mut to_add = 0;
            for _ in 0..day_to_go {
                for fish in fishs.iter_mut() {
                    if *fish == 0 {
                        to_add += 1;
                        *fish = 6
                    } else {
                        *fish -= 1;
                    }
                }
                fishs.extend((0..to_add).map(|_| 8));
                to_add = 0
            }

            fishs.len() as u64
        }
    }
}
