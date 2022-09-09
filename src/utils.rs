use std::io::{self, Read};
use std::ops::Sub;
use std::{fs::File, path::Path};

pub fn load_input<P: AsRef<Path>>(path: P) -> io::Result<String> {
    let mut file = File::open(path)?;
    let mut buffer = String::new();

    file.read_to_string(&mut buffer)?;
    Ok(buffer)
}

// The int_abs_diff feature is not stabilized at this time
pub fn abs_diff<T: PartialOrd + Sub<Output = T>>(x1: T, x2: T) -> T {
    if x1 < x2 {
        x2 - x1
    } else {
        x1 - x2
    }
}


pub fn vec_of_bit_to_u32(vec: &[u8]) -> u32 {
    let mut number = 0;
    let mut power_of_two = 1;
    for d in vec.iter().rev() {
        number += power_of_two * *d as u32;
        power_of_two <<= 1;
    }
    number
}
