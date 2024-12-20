use agb::fixnum::Num;

pub const fn new_num(m:i32) -> Num<i32, 8> {
    return Num::from_raw(m << 8);
}

#[allow(dead_code)]
pub fn clamp(n: &mut Num<i32, 8>, min: &Num<i32, 8>, max: &Num<i32, 8>) {
    if *n < *min {
        *n = *min;
    } else if *n > *max {
        *n = *max;
    }
}

pub fn abs(a: Num<i32, 8>) -> Num<i32, 8> {
    if a >= new_num(0) {
        return a;
    }
    else {
        return -a
    }
}