use std::collections::HashSet;

type Holes = HashSet<(u32, u32)>;

pub fn part1(input: &str) -> u32 {
    let (cmd, mut holes) = global(input);

    execut_cmd(cmd[0], &mut holes);

    holes.len() as u32
}

pub fn part2(input: &str) -> u32 {
    let (cmd, mut holes) = global(input);

    for cmd in cmd {
        execut_cmd(cmd, &mut holes);
    }
    
    println!("{}", holes_to_string(&holes));
    0
}

fn global(input: &str) -> (Vec<(&str, u32)>, Holes) {
    let mut input = input.trim().split("\r\n\r\n");
    let holes = input
        .next()
        .unwrap()
        .lines()
        .map(|line| {
            let mut coords = line.trim().split(',').map(|n| n.parse::<u32>().unwrap());
            (coords.next().unwrap(), coords.next().unwrap())
        })
        .collect::<Holes>();

    let cmd = input
        .next()
        .unwrap()
        .lines()
        .map(|line| {
            let cmd = line
                .trim()
                .rsplit_once(' ')
                .unwrap()
                .1
                .split_once('=')
                .unwrap();

            (cmd.0, cmd.1.parse::<u32>().unwrap())
        })
        .collect::<Vec<(&str, u32)>>();

    (cmd, holes)
}

fn execut_cmd(cmd: (&str, u32), holes: &mut Holes) {
    match cmd.0 {
        "x" => {
            let fliped = holes
                .drain_filter(|(x, _)| *x > cmd.1)
                .map(|(x, y)| (cmd.1 - (x - cmd.1), y))
                .collect::<Holes>();
            holes.extend(fliped)
        }
        "y" => {
            let fliped = holes
                .drain_filter(|(_, y)| *y > cmd.1)
                .map(|(x, y)| (x, cmd.1 - (y - cmd.1)))
                .collect::<Holes>();
            holes.extend(fliped)
        }
        _ => (),
    }
}

fn holes_to_string(holes: &Holes) -> String {
    let (min_x, max_x, min_y, max_y) = holes.iter().fold(
        (u32::MAX, 0, u32::MAX, 0),
        |(min_x, max_x, min_y, max_y), (x, y)| {
            (min_x.min(*x), max_x.max(*x), min_y.min(*y), max_y.max(*y))
        },
    );

    (min_y..=max_y)
        .map(|y| {
            (min_x..=max_x)
                .map(|x| if holes.contains(&(x, y)) { '#' } else { ' ' })
                .collect::<String>()
        })
        .collect::<Vec<String>>()
        .join("\n")
}
