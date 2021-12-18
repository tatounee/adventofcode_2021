use std::cmp::Ordering;

pub fn part1(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            line[(line.find('|').unwrap() + 1)..]
                .trim()
                .split(' ')
                .filter(|x| matches!(x.len(), 2 | 3 | 4 | 7))
                .count() as u32
        })
        .sum()
}

pub fn part2(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            let mut parts = line
                .split('|')
                .map(|numbres| {
                    numbres
                        .trim()
                        .split(' ')
                        .map(|digits| {
                            let mut digits = digits.chars().collect::<Vec<_>>();
                            digits.sort_unstable();
                            digits
                        })
                        .collect::<Vec<_>>()
                })
                .collect::<Vec<_>>();

            let to_find = parts.pop().unwrap();
            let mut digits = parts.pop().unwrap();

            let mut numbers = vec![vec![]; 10];

            // ONE
            numbers[1] = digits.remove(digits.iter().position(|digit| digit.len() == 2).unwrap());
            // FOUR
            numbers[4] = digits.remove(digits.iter().position(|digit| digit.len() == 4).unwrap());
            // SEVEN
            numbers[7] = digits.remove(digits.iter().position(|digit| digit.len() == 3).unwrap());
            // HEIGHT
            numbers[8] = digits.remove(digits.iter().position(|digit| digit.len() == 7).unwrap());

            let (mut five_segments, mut six_segments): (Vec<Vec<char>>, Vec<Vec<char>>) =
                digits.into_iter().partition(|digit| digit.len() == 5);

            // SIX
            numbers[6] = six_segments.remove(
                six_segments
                    .iter()
                    .position(|digit| unsafe { intersect(&numbers[1], digit) }.len() == 1)
                    .unwrap(),
            );

            // NINE
            numbers[9] = six_segments.remove(
                six_segments
                    .iter()
                    .position(|digit| unsafe { intersect(&numbers[4], digit) }.len() == 4)
                    .unwrap(),
            );

            // ZERO
            numbers[0] = six_segments.pop().unwrap();

            // THREE
            numbers[3] = five_segments.remove(
                five_segments
                    .iter()
                    .position(|digit| unsafe { intersect(&numbers[1], digit) }.len() == 2)
                    .unwrap(),
            );

            // FIVE
            numbers[5] = five_segments.remove(
                five_segments
                    .iter()
                    .position(|digit| unsafe { intersect(&numbers[9], digit) }.len() == 5)
                    .unwrap(),
            );

            // TWO
            numbers[2] = five_segments.pop().unwrap();

            to_find
                .into_iter()
                .rev()
                .enumerate()
                .map(|(i, to_find)| {
                    numbers.iter().position(|digit| digit == &to_find).unwrap() as u32
                        * 10u32.pow(i as u32)
                })
                .sum::<u32>()
        })
        .sum()
}

// The caller should assert that v1 and v2 are sorted
unsafe fn intersect(v1: &[char], v2: &[char]) -> Vec<char> {
    let mut inter = vec![];

    let mut i = 0;
    let mut j = 0;
    for _ in 0..(v1.len() + v2.len()) {
        if i == v1.len() || j == v2.len() {
            break;
        }
        match v1[i].cmp(&v2[j]) {
            Ordering::Greater => j += 1,
            Ordering::Equal => {
                inter.push(v1[i]);
                i += 1;
                j += 1;
            }
            Ordering::Less => i += 1,
        }
    }

    inter
}

#[test]
fn yes() {
    let a = vec!['a', 'b', 'c', 'd', 'h', 'i'];
    let b = vec!['a', 'b', 'd', 'e', 'h'];

    let i = unsafe { intersect(&a, &b) };

    assert_eq!(vec!['a', 'b', 'd', 'h'], i);
}
