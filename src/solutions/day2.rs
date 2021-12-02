pub fn part1(input: &str) -> i32 {
    let (x, y) = input
        .split('\n')
        .map(|line| {
            let mut line = line.trim().split(' ');
            (
                line.next().unwrap(),
                line.next().unwrap().parse::<i32>().unwrap(),
            )
        })
        .fold((0, 0), |(x, y), (cmd, value)| match cmd {
            "forward" => (x + value, y),
            "down" => (x, y + value),
            "up" => (x, y - value),
            _ => unreachable!(),
        });

    x * y
}

pub fn part2(input: &str) -> i32 {
    let (x, y, _) = input
        .split('\n')
        .map(|line| {
            let mut line = line.trim().split(' ');
            (
                line.next().unwrap(),
                line.next().unwrap().parse::<i32>().unwrap(),
            )
        })
        .fold((0, 0, 0), |(x, y, aim), (cmd, value)| match cmd {
            "forward" => (x + value, y + value * aim, aim),
            "down" => (x, y, aim + value),
            "up" => (x, y, aim - value),
            _ => unreachable!(),
        });

    x * y
}
