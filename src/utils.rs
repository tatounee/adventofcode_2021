// The int_abs_diff feature is not stabilized at this time
pub fn abs_diff<T: PartialOrd + Sub<Output = T>>(x1: T, x2: T) -> T {
    if x1 < x2 {
        x2 - x1
    } else {
        x1 - x2
    }
}
