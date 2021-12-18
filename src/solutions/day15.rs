use std::{fmt, str::FromStr, hash::Hash};

use pathfinding::prelude::dijkstra;

type Coord = (usize, usize);

pub fn part1(input: &str) -> u32 {
    global(input, 1)
}

pub fn part2(input: &str) -> u32 {
    global(input, 5)
}

fn global(input: &str, repeating: usize) -> u32 {
    let mut grid = Grid::from_str(input).unwrap();
    grid.repeating = repeating;

    let first = grid.first().unwrap();
    let last = grid.last().unwrap();

    let result = dijkstra(
        first,
        |from_node| {
            grid.get_neighbours(from_node.coord)
                .into_iter()
                .map(|to_node| {
                    let risk = to_node.risk + 1;
                    (to_node, risk)
                })
        },
        |node| node == &last,
    )
    .unwrap();

    result.1
}

#[derive(Clone, Eq)]
struct Node {
    risk: u32,
    coord: Coord,
}

impl Node {
    fn to_grow(&self, by_x: usize, by_y: usize, width: usize, height: usize) -> Node {
        let mut node = self.clone();
        node.coord.0 += by_x * width;
        node.coord.1 += by_y * height;
        node.risk = (node.risk + (by_x + by_y) as u32) % 9;
        node
    }
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.coord == other.coord
    }
}

impl Hash for Node {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.coord.hash(state)
    }
}

impl fmt::Debug for Node {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_fmt(format_args!("{:?} <{}>", self.coord, self.risk + 1))
    }
}

struct Grid {
    grid: Vec<Vec<Node>>,
    repeating: usize,
}

impl Grid {
    #[inline]
    fn width(&self) -> usize {
        self.grid.len()
    }

    #[inline]
    fn height(&self) -> usize {
        self.grid[0].len()
    }

    #[inline]
    fn first(&self) -> Option<&Node> {
        self.grid.first()?.first()
    }

    #[inline]
    fn last(&self) -> Option<Node> {
        self.grid.last()?.last().map(|node| {
            node.to_grow(
                self.repeating - 1,
                self.repeating - 1,
                self.width(),
                self.height(),
            )
        })
    }

    fn get_neighbours(&self, (fixe_x, fixe_y): Coord) -> Vec<Node> {
        let mut coords = Vec::new();

        let max_x = self.width() * self.repeating - 1;
        let max_y = self.height() * self.repeating - 1;

        let tile_y = fixe_y / self.width();
        let tile_x = fixe_x / self.height();
        let y = fixe_y % self.height();

        if fixe_y <= max_y && fixe_x <= max_x {
            let x = fixe_x % self.width();

            let top_y = if fixe_y != 0 {
                match y.checked_sub(1) {
                    Some(y) => WCoord::InPlace(y),
                    None => WCoord::Wrapped(self.height() - 1),
                }
            } else {
                WCoord::NonExistent
            };

            let bottom_y = if fixe_y != max_y {
                let y = y + 1;
                if y < self.height() {
                    WCoord::InPlace(y)
                } else {
                    WCoord::Wrapped(0)
                }
            } else {
                WCoord::NonExistent
            };

            let left_x = if fixe_x != 0 {
                match x.checked_sub(1) {
                    Some(x) => WCoord::InPlace(x),
                    None => WCoord::Wrapped(self.width() - 1),
                }
            } else {
                WCoord::NonExistent
            };

            let right_x = if fixe_x != max_x {
                let x = x + 1;
                if x < self.height() {
                    WCoord::InPlace(x)
                } else {
                    WCoord::Wrapped(0)
                }
            } else {
                WCoord::NonExistent
            };

            // Top
            match top_y {
                WCoord::InPlace(y) => coords.push(self.grid[y][x].to_grow(
                    tile_x,
                    tile_y,
                    self.width(),
                    self.height(),
                )),
                WCoord::Wrapped(y) => coords.push(self.grid[y][x].to_grow(
                    tile_x,
                    tile_y - 1,
                    self.width(),
                    self.height(),
                )),
                WCoord::NonExistent => (),
            }

            // Bottom
            match bottom_y {
                WCoord::InPlace(y) => coords.push(self.grid[y][x].to_grow(
                    tile_x,
                    tile_y,
                    self.width(),
                    self.height(),
                )),
                WCoord::Wrapped(y) => coords.push(self.grid[y][x].to_grow(
                    tile_x,
                    tile_y + 1,
                    self.width(),
                    self.height(),
                )),
                WCoord::NonExistent => (),
            }

            // Left
            match left_x {
                WCoord::InPlace(x) => coords.push(self.grid[y][x].to_grow(
                    tile_x,
                    tile_y,
                    self.width(),
                    self.height(),
                )),
                WCoord::Wrapped(x) => coords.push(self.grid[y][x].to_grow(
                    tile_x - 1,
                    tile_y,
                    self.width(),
                    self.height(),
                )),
                WCoord::NonExistent => (),
            }

            // Right
            match right_x {
                WCoord::InPlace(x) => coords.push(self.grid[y][x].to_grow(
                    tile_x,
                    tile_y,
                    self.width(),
                    self.height(),
                )),
                WCoord::Wrapped(x) => coords.push(self.grid[y][x].to_grow(
                    tile_x + 1,
                    tile_y,
                    self.width(),
                    self.height(),
                )),
                WCoord::NonExistent => (),
            }
        }

        coords
    }
}

impl FromStr for Grid {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            repeating: 1,
            grid: s
                .trim()
                .lines()
                .enumerate()
                .map(|(y, line)| {
                    line.trim()
                        .chars()
                        .enumerate()
                        .map(move |(x, c)| Node {
                            risk: c.to_digit(10).unwrap() - 1,
                            coord: (x, y),
                        })
                        .collect()
                })
                .collect(),
        })
    }
}

enum WCoord {
    InPlace(usize),
    Wrapped(usize),
    NonExistent,
}
