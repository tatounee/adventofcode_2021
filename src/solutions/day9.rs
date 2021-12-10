use std::{collections::HashSet, str::FromStr};

pub fn part1(input: &str) -> u32 {
    let cave = Cave::from_str(input).unwrap();
    cave.sum_min()
}

pub fn part2(input: &str) -> u32 {
    let mut width = 0;
    let mut height = 0;

    let mut coords = HashSet::new();
    let mut marked = HashSet::new();

    let mut sizes = vec![];

    for (y, line) in input.lines().enumerate() {
        height = y;
        for (x, ground) in line.chars().enumerate() {
            width = x;
            let ground = ground.to_digit(10).unwrap();
            if ground == 9 {
                marked.insert((x as u8, y as u8));
            } else {
                coords.insert((x as u8, y as u8));
            }
        }
    }

    for y in 0..height {
        let y = y as u8;
        for x in 0..width {
            let x = x as u8;
            if coords.contains(&(x, y)) {
                let size = extract_basine((x, y), &mut coords, &mut marked);
                if let Some(size) = size {
                    sizes.push(size);
                }
            }
        }
    }

    get_maxs(&sizes, 3).into_iter().product()
}

struct Cave {
    cave: Vec<Vec<u8>>,
}

impl Cave {
    fn sum_min(&self) -> u32 {
        self.cave
            .iter()
            .enumerate()
            .map(|(y, line)| {
                line.iter()
                    .enumerate()
                    .filter_map(|(x, height)| {
                        if self
                            .get_neighbours((x, y))
                            .unwrap()
                            .iter()
                            .all(|h| height < h)
                        {
                            Some(*height as u32 + 1)
                        } else {
                            None
                        }
                    })
                    .sum::<u32>()
            })
            .sum()
    }

    fn get_neighbours(&self, (x, y): (usize, usize)) -> Option<Vec<u8>> {
        let mut vec = vec![];

        if let Some(line) = self.cave.get(y) {
            if let Some(h) = line.get(x + 1) {
                vec.push(*h)
            }
            if x != 0 {
                if let Some(h) = line.get(x - 1) {
                    vec.push(*h)
                }
            }
        }

        if let Some(line) = self.cave.get(y + 1) {
            if let Some(h) = line.get(x) {
                vec.push(*h)
            }
        }
        if y != 0 {
            if let Some(line) = self.cave.get(y - 1) {
                if let Some(h) = line.get(x) {
                    vec.push(*h)
                }
            }
        }

        Some(vec)
    }
}

impl FromStr for Cave {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            cave: s
                .lines()
                .map(|line| {
                    line.chars()
                        .map(|c| c.to_digit(10).unwrap() as u8)
                        .collect()
                })
                .collect(),
        })
    }
}

fn extract_basine(
    coord: (u8, u8),
    from: &mut HashSet<(u8, u8)>,
    to: &mut HashSet<(u8, u8)>,
) -> Option<u32> {
    if to.contains(&coord) {
        None
    } else {
        let mut size_basine = 1;
        from.remove(&coord);
        to.insert(coord);
        for coord in get_neighbours(coord, &*from) {
            if from.contains(&coord) {
                let size = extract_basine(coord, from, to);
                if let Some(size) = size {
                    size_basine += size
                }
                to.insert(coord);
                from.remove(&coord);
            }
        }
        Some(size_basine)
    }
}

fn get_neighbours((x, y): (u8, u8), set: &HashSet<(u8, u8)>) -> Vec<(u8, u8)> {
    let mut coords = vec![];

    let left = x.checked_sub(1).map(|x| (x, y));
    let right = (x + 1, y);
    let up = y.checked_sub(1).map(|y| (x, y));
    let down = (x, y + 1);

    if set.contains(&right) {
        coords.push(right)
    }
    if set.contains(&down) {
        coords.push(down)
    }
    if let Some(left) = left {
        if set.contains(&left) {
            coords.push(left)
        }
    }
    if let Some(up) = up {
        if set.contains(&up) {
            coords.push(up)
        }
    }

    coords
}

fn get_maxs(vec: &[u32], amount: usize) -> Vec<u32> {
    let mut maxs = vec[0..amount].to_vec();

    for x in vec.iter().skip(amount) {
        maxs.push(*x);
        maxs.sort_by(|a, b| b.cmp(a));
        maxs.pop();
    }

    maxs
}

// very dirty, maybe a better way is to iter over 0..height and 0..width
fn coords_to_string(coords: &HashSet<(u8, u8)>, width: usize, height: usize) -> String {
    let mut lines = vec![" ".repeat(width); height];

    for (x, y) in coords.iter() {
        if let Some(line) = lines.get_mut(*y as usize) {
            let x = *x as usize;
            *line = line
                .chars()
                .enumerate()
                .map(|(i, c)| if i == x { '#' } else { c })
                .collect();
        }
    }

    lines.join("\n")
}

#[test]
fn yes() {
    let vec = [
        78, 5, 13, 87, 0, 1, 32, 7, 3, 98, 12, 5, 1, 321, 4, 8, 87, 354, 654, 32, 187,
    ];

    let maxs = get_maxs(&vec, 4);

    assert_eq!(maxs, vec![654, 354, 321, 187])
}
