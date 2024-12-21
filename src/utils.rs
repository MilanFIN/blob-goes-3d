
use crate::Fixed;

pub const fn new_num(m:i32) -> Fixed {
    return Fixed::from_raw(m << 8);
}

#[allow(dead_code)]
pub fn clamp(n: &mut Fixed, min: &Fixed, max: &Fixed) {
    if *n < *min {
        *n = *min;
    } else if *n > *max {
        *n = *max;
    }
}
