use std::fmt;

use crate::utils::abs_diff;

pub fn part1(input: &str) -> u32 {
    global(input, Map::trace)
}

pub fn part2(input: &str) -> u32 {
    global(input, Map::trace_with_diagonal)
}

fn global<F: Fn(&mut Map, (usize, usize), (usize, usize)) -> Option<()>>(
    input: &str,
    trace: F,
) -> u32 {
    let mut map = Map::default();

    let coords = input
        .lines()
        .map(|line| {
            let mut coords = line.split(" -> ").map(|coord| {
                let mut coord = coord.split(',');
                let x = coord.next().unwrap().parse::<usize>().unwrap();
                let y = coord.next().unwrap().parse::<usize>().unwrap();
                (x, y)
            });

            let start = coords.next().unwrap();
            let end = coords.next().unwrap();
            (start, end)
        })
        .collect::<Vec<_>>();

    for (start, end) in coords {
        trace(&mut map, start, end);
    }

    map.count_more_than(1)
}

struct Map {
    map: Vec<Vec<u8>>,
}

impl Map {
    fn trace(&mut self, (x1, y1): (usize, usize), (x2, y2): (usize, usize)) -> Option<()> {
        let (x1, x2) = if x1 < x2 { (x1, x2) } else { (x2, x1) };
        let (y1, y2) = if y1 < y2 { (y1, y2) } else { (y2, y1) };

        if y1 == y2 {
            let line = self.map.get_mut(y1)?;
            for x in x1..=x2 {
                *line.get_mut(x)? += 1;
            }
        } else if x1 == x2 {
            for y in y1..=y2 {
                *self.map.get_mut(y)?.get_mut(x1)? += 1
            }
        }

        Some(())
    }

    fn trace_with_diagonal(
        &mut self,
        (x1, y1): (usize, usize),
        (x2, y2): (usize, usize),
    ) -> Option<()> {
        if x1 == x2 || y1 == y2 {
            self.trace((x1, y1), (x2, y2))
        } else if abs_diff(x1, x2) == abs_diff(y1, y2) {
            let ((x1, y1), (x2, y2)) = if x1 < x2 {
                ((x1, y1), (x2, y2))
            } else {
                ((x2, y2), (x1, y1))
            };

            if y1 < y2 {
                for (x, y) in (x1..=x2).zip(y1..=y2) {
                    *self.map.get_mut(y)?.get_mut(x)? += 1
                }
            } else {
                for (x, y) in (x1..=x2).zip((y2..=y1).rev()) {
                    *self.map.get_mut(y)?.get_mut(x)? += 1
                }
            };

            Some(())
        } else {
            Some(())
        }
    }

    fn count_more_than(&self, n: u8) -> u32 {
        self.map
            .iter()
            .map(|line| line.iter().filter(|value| **value > n).count())
            .sum::<usize>() as u32
    }
}

impl Default for Map {
    fn default() -> Self {
        Self {
            map: (0..1000)
                .map(|_| (0..1000).map(|_| 0).collect::<Vec<_>>())
                .collect::<Vec<_>>(),
        }
    }
}

impl fmt::Debug for Map {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(
            &self
                .map
                .iter()
                .map(|line| {
                    line.iter()
                        .map(|n| {
                            if *n == 0 {
                                '.'.to_string()
                            } else {
                                n.to_string()
                            }
                        })
                        .collect::<String>()
                })
                .collect::<Vec<String>>()
                .join("\n"),
        )
    }
}
