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
