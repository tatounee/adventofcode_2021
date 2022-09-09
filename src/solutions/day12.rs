use std::collections::{HashMap, HashSet};

pub fn part1(input: &str) -> u32 {
    let network = parse_network(input);

    0u8.checked_add(4);
    let mut path = HashSet::new();
    path.insert("start");
    shearch1(&network, "start", path)

}

pub fn part2(input: &str) -> u32 {
    let network = parse_network(input);

    let mut path = HashMap::new();
    path.insert("start", 2);
    shearch2(&network, "start", (false, path))
}

fn parse_network(input: &str) -> HashMap<&str, HashSet<&str>> {
    let mut network = HashMap::new();

    input.trim().lines().for_each(|line| {
        let caves = line.trim().split('-').collect::<Vec<_>>();
        network
            .entry(caves[0])
            .or_insert_with(HashSet::new)
            .insert(caves[1]);
        network
            .entry(caves[1])
            .or_insert_with(HashSet::new)
            .insert(caves[0]);
    });

    network
}

fn shearch1<'a>(
    network: &HashMap<&str, HashSet<&'a str>>,
    start: &str,
    path: HashSet<&'a str>,
) -> u32 {
    let mut paths = 0;

    for cave in network.get(start).unwrap() {
        if *cave == "end" {
            paths += 1;
        } else if is_big(cave) || !path.contains(cave) {
            let mut path = path.clone();
            path.insert(cave);
            paths += shearch1(network, cave, path);
        }
    }

    paths
}

fn shearch2<'a>(
    network: &HashMap<&str, HashSet<&'a str>>,
    start: &str,
    (has_two, path): (bool, HashMap<&'a str, u32>),
) -> u32 {
    let mut paths = 0;

    for cave in network.get(start).unwrap() {
        if *cave == "start" {
            continue;
        } else if *cave == "end" {
            paths += 1;
        } else if is_big(cave) {
            let mut path = path.clone();
            *path.entry(cave).or_insert(0) += 1;
            paths += shearch2(network, cave, (has_two, path));
        } else {
            let mut path = path.clone();
            let visit = path.entry(cave).or_insert(0);
            if !has_two || *visit == 0  {
                *visit += 1;
                paths += shearch2(network, cave, (*visit >= 2 || has_two, path));
            }
        }
    }

    paths
}

#[inline]
fn is_big(cave: &str) -> bool {
    cave.chars().all(|c| c.is_uppercase())
}
