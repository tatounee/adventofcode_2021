use crate::utils::abs_diff;

pub fn part1(input: &str) -> u32 {
    let mut ferris = input
        .trim()
        .split(',')
        .map(|n| n.parse::<u32>().unwrap())
        .collect::<Vec<_>>();
    ferris.sort_unstable();

    let go_to = if ferris.len() % 2 == 1 {
        ferris[ferris.len() / 2]
    } else {
        (ferris[ferris.len() / 2] + ferris[ferris.len() / 2 + 1]) / 2
    };

    ferris.iter().map(|c| abs_diff(*c, go_to)).sum()
}

pub fn part2(input: &str) -> u32 {
    let ferris = input
        .trim()
        .split(',')
        .map(|n| n.parse::<u32>().unwrap())
        .collect::<Vec<_>>();

    let mean = ferris.iter().sum::<u32>() / ferris.len() as u32;

    let mut fuel = ferris
        .iter()
        .map(|c| {
            let n = abs_diff(*c, mean);
            n * (n + 1) / 2
        })
        .sum();

    let neg = ferris
        .iter()
        .map(|c| {
            let n = abs_diff(*c, mean + 1);
            n * (n + 1) / 2
        })
        .sum::<u32>()
        > fuel;

    for i in 1..mean {
        let fuel_maybe = ferris
            .iter()
            .map(|c| {
                let n = abs_diff(*c, if neg { mean - i } else { mean + i });
                n * (n + 1) / 2
            })
            .sum();

        if fuel_maybe < fuel {
            fuel = fuel_maybe
        } else {
            return fuel;
        }
    }

    // This case is only used when mean < 2
    fuel
}
