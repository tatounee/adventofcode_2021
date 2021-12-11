use std::{fmt, str::FromStr};

type Coord = (usize, usize);

pub fn part1(input: &str) -> u32 {
    let mut octopus = Octopus::from_str(input).unwrap();
    let mut flashs = 0;

    for _ in 0..100 {
        let to_flashs = octopus.grow();
        octopus.flash(to_flashs).unwrap();
        flashs += octopus.count_and_reset_flash()
    }

    flashs
}

pub fn part2(input: &str) -> u32 {
    let mut octopus = Octopus::from_str(input).unwrap();

    for i in 1.. {
        let to_flashs = octopus.grow();
        octopus.flash(to_flashs).unwrap();
        
        if octopus.count_and_reset_flash() == 100 {
            return i
        }
    }

    unreachable!()
}

struct Octopus {
    octos: [[(bool, u8); 10]; 10],
}

impl Octopus {
    #[inline]
    fn grow(&mut self) -> Vec<Coord> {
        self.octos
            .iter_mut()
            .enumerate()
            .map(|(y, line)| {
                line.iter_mut()
                    .enumerate()
                    .flat_map(move |(x, (_, energy))| {
                        *energy += 1;
                        if *energy == 10 {
                            Some((x, y))
                        } else {
                            None
                        }
                    })
            })
            .flatten()
            .collect()
    }

    fn flash(&mut self, to_flashs: Vec<Coord>) -> Option<()> {
        for (x, y) in to_flashs {
            self.octos.get_mut(y)?.get_mut(x)?.0 = true;
            let to_flashs = self.get_neigtbours_and_grow((x, y));
            self.flash(to_flashs).unwrap()
        }

        Some(())
    }

    fn get_neigtbours_and_grow(&mut self, (x, y): Coord) -> Vec<Coord> {
        (0..=2)
            .filter_map(|shift_y| {
                (y + shift_y).checked_sub(1).map(|ty| {
                    (0..=2).filter_map(move |shift_x| {
                        (x + shift_x).checked_sub(1).and_then(|tx| {
                            if !(tx == x && ty == y) && tx < 10 && ty < 10 {
                                Some((tx, ty))
                            } else {
                                None
                            }
                        })
                    })
                })
            })
            .flatten()
            .flat_map(|(x, y)| {
                let (flashed, energy) = self.octos.get_mut(y).unwrap().get_mut(x).unwrap();
                if !*flashed && *energy < 10 {
                    *energy += 1;
                    if *energy > 9 {
                        *flashed = true;
                        return Some((x, y));
                    }
                }
                None
            })
            .collect()
    }

    fn count_and_reset_flash(&mut self) -> u32 {
        self.octos
            .iter_mut()
            .map(|line| {
                line.iter_mut()
                    .flat_map(|(flashed, energy)| {
                        if *flashed {
                            *flashed = false;
                            *energy = 0;
                            Some(())
                        } else {
                            None
                        }
                    })
                    .count() as u32
            })
            .sum()
    }
}

impl FromStr for Octopus {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            octos: s
                .trim()
                .lines()
                .map(|line| {
                    line.trim()
                        .chars()
                        .map(|c| (false, c.to_digit(10).unwrap() as u8))
                        .collect::<Vec<_>>()
                        .try_into()
                        .unwrap()
                })
                .collect::<Vec<_>>()
                .try_into()
                .unwrap(),
        })
    }
}

impl fmt::Debug for Octopus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(
            &self
                .octos
                .iter()
                .map(|line| {
                    line.iter()
                        .map(
                            |(flashed, energy)| {
                                if *flashed {
                                    ' '
                                } else {
                                    (energy + 48) as char
                                }
                            },
                        )
                        .collect::<String>()
                })
                .collect::<Vec<String>>()
                .join("\n"),
        )
    }
}
