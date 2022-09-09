pub fn part1(input: &str) -> u32 {
    let bytes = hex_to_bytes(input);
    let mut total = 0;

    let mut ptr = 0;
    while (ptr as usize) < bytes.len() * 8 {
        total += read_number(&bytes, ptr, 3).unwrap() as u32;
        ptr += 3;

        let type_id = read_number(&bytes, ptr, 3);
    }

    0
}

fn parse_packet(bytes: &[u8], ptr: &mut u32) -> u32 {
    let mut total = 0;

    while (*ptr as usize) < bytes.len() * 8 {
        total += read_number(&bytes, ptr, 3).unwrap() as u32;

        let type_id = read_number(&bytes, ptr, 3).unwrap();

        if type_id == 4 {
            parse_literal(bytes, ptr);
        }
    }

    total
}

fn parse_literal(bytes: &[u8], ptr: &mut u32) -> Vec<u8> {
    let mut total = vec![];
    let start = *ptr;

    let mut seconde = false;
    while read_number(bytes, ptr, 1).unwrap() != 0 {
        if !seconde {
            total.push((read_number(bytes, ptr, 4).unwrap() as u8) << 4)
        } else {
            let last = total.last_mut().unwrap();
            *last |= read_number(bytes, ptr, 4).unwrap() as u8
        }

        seconde = !seconde
    }

    if !seconde {
        total.push((read_number(bytes, ptr, 4).unwrap() as u8) << 4)
    } else {
        let last = total.last_mut().unwrap();
        *last |= read_number(bytes, ptr, 4).unwrap() as u8
    }

    *ptr += (*ptr - start) % 4;

    total
}

fn parse_op(bytes: &[u8], mut ptr: u32) -> Vec<Vec<u8>> {
    let packets = vec![];
    let lenght_type_id = read_number(bytes, &mut ptr, 1).unwrap() != 0;
    ptr += 1;

    if lenght_type_id {
        let packet_amount = read_number(bytes, &mut ptr, 11);
    }

    packets
}

fn hex_to_bytes(string: &str) -> Vec<u8> {
    string
        .trim()
        .chars()
        .fold((vec![], false), |(mut acc, seconde), c| {
            (
                if seconde {
                    let last = acc.last_mut().unwrap();
                    *last |= c.to_digit(16).unwrap() as u8;
                    acc
                } else {
                    acc.push((c.to_digit(16).unwrap() as u8) << 4);
                    acc
                },
                !seconde,
            )
        })
        .0
}

fn read_number(bytes: &[u8], start: &mut u32, len: u8) -> Option<u32> {
    if len > 32 || len == 0 {
        return None;
    }

    let len = len as u32;

    let start_index = (*start / 8) as usize;
    if start_index >= bytes.len() {
        return None;
    }

    let start_pos = *start % 8;
    let superposition = ((len - 1 + start_pos) / 8) as usize;
    let mask_start = 0xFFu8 >> start_pos;

    *start += len;

    if superposition == 0 {
        Some(((bytes[start_index] & mask_start) >> (8 - (start_pos + len))) as u32)
    } else if start_index + (superposition) < bytes.len() {
        let mask_end = 0xFFu8.overflowing_shl(8 - (len + start_pos) % 8).0;

        let mut number = (bytes[start_index] & mask_start) as u32;

        for i in 1..superposition {
            number = (number << 8) | bytes[start_index + i] as u32;
        }

        let place = len + start_pos;
        let place = if place == 0 { 0 } else { (place - 1) % 8 + 1 };

        number = (number << place)
            | ((bytes[start_index + superposition] & mask_end) as u32 >> (8 - place) % 8);
        Some(number)
    } else {
        None
    }
}

#[test]
fn h2b() {
    let s = "45A6F0";

    let b = hex_to_bytes(s);

    assert_eq!(b, vec![69, 166, 240])
}

#[test]
fn rn() {
    let bytes = vec![0b10110011, 0b01110010, 0b10011100, 0b11100110];

    let n1 = read_number(&bytes, &mut 0, 4).unwrap();
    let n2 = read_number(&bytes, &mut 3, 5).unwrap();
    let n3 = read_number(&bytes, &mut 9, 3).unwrap();
    let n4 = read_number(&bytes, &mut 4, 8).unwrap();
    let n5 = read_number(&bytes, &mut 14, 6).unwrap();
    let n6 = read_number(&bytes, &mut 7, 11).unwrap();
    let n7 = read_number(&bytes, &mut 0, 32).unwrap();

    assert_eq!(n1, 0b1011);
    assert_eq!(n2, 0b10011);
    assert_eq!(n3, 0b111);
    assert_eq!(n4, 0b110111);
    assert_eq!(n5, 0b101001);
    assert_eq!(n6, 0b10111001010);
    assert_eq!(n7, 0b10110011011100101001110011100110);
}
