pub fn part1(input: &str) -> u32 {
    let height = input
        .split('\n')
        .map(|line| line.trim().parse::<u32>().unwrap())
        .collect::<Vec<_>>();

    height
        .windows(2)
        .fold(0, |acc, h| acc + (h[1] > h[0]) as u32)
}

pub fn part2(input: &str) -> u32 {
    let height = input
        .split('\n')
        .map(|line| line.trim().parse::<u32>().unwrap())
        .collect::<Vec<_>>();

    let mut height = height.windows(3).map(|h| h.iter().sum::<u32>());

    let first = height.next().unwrap();
    height
        .fold((0, first), |(acc, previus), h| {
            (acc + (h > previus) as u32, h)
        })
        .0
}
