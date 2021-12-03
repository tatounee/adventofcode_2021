pub fn part1(input: &str) -> u32 {
    let mut input = input.split('\n').map(|l| l.trim());

    let mut acc = vec![];

    for c in input.next().unwrap().chars() {
        let value = c.to_digit(2).unwrap() as i32 * 2 - 1;
        acc.push(value);
    }

    for line in input {
        for (idx, c) in line.chars().enumerate() {
            let value = c.to_digit(2).unwrap() as i32 * 2 - 1;
            acc[idx] += value;
        }
    }

    let acc = acc
        .into_iter()
        .map(|value| (value >= 0) as u8)
        .collect::<Vec<_>>();

    let gamma = vec_of_bit_to_u32(&acc);

    let espilon = 2u32.pow(acc.len() as u32) - 1 - gamma;

    gamma * espilon
}

#[allow(non_snake_case)]
pub fn part2(input: &str) -> u32 {
    let len = input
        .chars()
        .map_while(|c| if c != '\n' { Some(c) } else { None })
        .count();
    let first = input
        .split('\n')
        .map(|line| (line.trim().starts_with('1')) as i8 * 2 - 1)
        .sum::<i8>();

    let lines = input
        .split('\n')
        .map(|line| {
            line.trim()
                .chars()
                .map(|c| c.to_digit(2).unwrap() as u8)
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let mut oxygen = vec![];
    let mut CO2 = vec![];

    let maj = (first >= 0) as u8;

    for line in lines.iter() {
        if line[0] == maj {
            oxygen.push(line)
        } else {
            CO2.push(line)
        }
    }

    for i in 1..len {
        let maj = (oxygen
            .iter()
            .map(|line| (line[i] != 0) as i8 * 2 - 1)
            .sum::<i8>()
            >= 0) as u8;

        oxygen.retain(|line| line[i] == maj);

        if oxygen.len() <= 1 {
            break;
        }
    }

    for i in 1..len {
        let maj = (CO2
            .iter()
            .map(|line| (line[i] != 0) as i8 * 2 - 1)
            .sum::<i8>()
            < 0) as u8;

        CO2.retain(|line| line[i] == maj);

        if CO2.len() <= 1 {
            break;
        }
    }

    let oxygen = vec_of_bit_to_u32(oxygen[0]);
    let CO2 = vec_of_bit_to_u32(CO2[0]);

    oxygen * CO2
}

fn get_sum_bit_each_line(input: &str) -> Vec<i32> {
    let mut input = input.split('\n').map(|l| l.trim());

    let mut acc = vec![];

    for c in input.next().unwrap().chars() {
        let value = c.to_digit(2).unwrap() as i32 * 2 - 1;
        acc.push(value);
    }

    for line in input {
        for (idx, c) in line.chars().enumerate() {
            let value = c.to_digit(2).unwrap() as i32 * 2 - 1;
            acc[idx] += value;
        }
    }

    acc
}

fn vec_of_bit_to_u32(vec: &[u8]) -> u32 {
    let mut nomber = 0;
    let mut power_of_two = 1;
    for d in vec.iter().rev() {
        nomber += power_of_two * *d as u32;
        power_of_two <<= 1;
    }
    nomber
}
