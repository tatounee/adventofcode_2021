use std::{convert::identity, fmt};

pub fn part1(input: &str) -> u32 {
    let (numbers, mut boards) = global(input);

    for n in numbers {
        for board in boards.iter_mut() {
            let win = board.play(n).unwrap_or(false);
            if win {
                return n as u32 * board.sum_unmarcked();
            }
        }
    }

    panic!("There is no winner here :(")
}

pub fn part2(input: &str) -> u32 {
    let (numbers, mut boards) = global(input);

    let mut to_remove = vec![];
    for n in numbers {
        let len = boards.len();
        for (i, board) in boards.iter_mut().enumerate() {
            let win = board.play(n).unwrap_or(false);
            if win {
                if len == 1 {
                    return n as u32 * boards[0].sum_unmarcked();
                }
                to_remove.push(i);
            }
        }

        for i in to_remove.iter().rev() {
            boards.remove(*i);
        }
        to_remove.clear();
    }

    panic!("There is no last winner here :(")
}

fn global(input: &str) -> (Vec<u8>, Vec<Board>) {
    let mut input = input
        .lines()
        .map(|line| line.trim())
        .filter(|line| !line.is_empty());

    let numbers = input
        .next()
        .unwrap()
        .split(',')
        .map(|n| n.parse::<u8>().unwrap())
        .collect::<Vec<_>>();

    let mut boards = vec![];
    let mut buffer = vec![];
    for line in input {
        let line = line
            .split(' ')
            .filter(|n| !n.is_empty())
            .map(|n| (false, n.parse::<u8>().unwrap()))
            .collect::<Vec<_>>()
            .as_slice()
            .try_into()
            .unwrap();

        buffer.push(line);
        if buffer.len() == 5 {
            let board = buffer.as_slice().try_into().unwrap();
            boards.push(Board::new(board));
            buffer.clear();
        }
    }

    (numbers, boards)
}

struct Board {
    board: [[(bool, u8); 5]; 5],
}

impl Board {
    fn new(board: [[(bool, u8); 5]; 5]) -> Self {
        Self { board }
    }

    #[inline]
    fn check(&self, x: usize, y: usize) -> Option<bool> {
        let line = self
            .board
            .get(y)?
            .iter()
            .map(|(checked, _)| *checked)
            .all(identity);

        let column = (0..5)
            .map(|y| self.board[y].get(x).map(|(b, _)| *b))
            .all(|b| b.unwrap_or(false));

        Some(line || column)
    }

    #[inline]
    fn play(&mut self, n: u8) -> Option<bool> {
        let mut x = None;
        let mut y = None;

        'outer: for i in 0..5 {
            for j in 0..5 {
                if self.board[i][j].1 == n {
                    x = Some(j);
                    y = Some(i);
                    break 'outer;
                }
            }
        }

        let x = x?;
        let y = y?;

        self.board.get_mut(y)?.get_mut(x)?.0 = true;
        self.check(x, y)
    }

    #[inline]
    fn sum_unmarcked(&self) -> u32 {
        self.board
            .iter()
            .map(|line| {
                line.iter()
                    .filter_map(|(b, n)| if !b { Some(*n as u32) } else { None })
                    .sum::<u32>()
            })
            .sum()
    }
}

impl fmt::Debug for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(
            &self
                .board
                .iter()
                .map(|line| {
                    line.iter()
                        .map(|(b, n)| {
                            if *b {
                                format!("[{:2}]", n)
                            } else {
                                format!(" {:2} ", n)
                            }
                        })
                        .collect::<Vec<String>>()
                        .join(" ")
                })
                .collect::<Vec<String>>()
                .join("\n"),
        )
    }
}
