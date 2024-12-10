use agb::fixnum::{Num};

pub const fn NewNum(m:i32) -> Num<i32, 8> {
    return Num::from_raw(m << 8);
}

pub fn clamp(n: &mut Num<i32, 8>, min: &Num<i32, 8>, max: &Num<i32, 8>) {
    if *n < *min {
        *n = *min;
    } else if *n > *max {
        *n = *max;
    }
}
