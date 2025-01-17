use agb::fixnum::Num;
use core::cmp::Ordering;
use core::ops::Neg;
use core::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};
use serde::Deserialize;
use serde::Deserializer;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct Fixed(Num<i32, 8>);

pub fn default_bool() -> bool {
    false
}

pub fn default_u16() -> u16 {
    0
}

#[allow(dead_code)]
pub fn default_i16() -> i16 {
    0
}

pub fn default_i32() -> i32 {
    0
}

pub fn positive_i16() -> i16 {
    1
}


pub fn default_fixed() -> Fixed {
    Fixed::new(0)
}

pub fn default_fixed_3_8() -> [[Fixed; 3]; 8] {
    [[Fixed::const_new(0); 3]; 8]
}

pub fn default_fixed_3_14() -> [[Fixed; 3]; 14] {
    [[Fixed::const_new(0); 3]; 14]
}


pub fn default_fixed_3_3() -> [[Fixed; 3]; 3] {
    [[Fixed::const_new(0); 3]; 3]
}

pub const fn i32_to_fixed(m: i32) -> Fixed {
    return Fixed(Num::from_raw(m << 8));
}

impl Fixed {
    /// Creates a new `Fixed` with a default precision of 8.
    #[allow(dead_code)]
    pub fn new(arg: i32) -> Self {
        Fixed(Num::new(arg)) // Precision is always 8
    }
    pub const fn const_new(arg: i32) -> Self {
        i32_to_fixed(arg)
    }
    pub const fn from_raw(arg: i32) -> Self {
        Fixed(Num::from_raw(arg))
    }
    #[allow(dead_code)]
    pub fn to_raw(self) -> i32 {
        Num::to_raw(self.0)
    }
    #[allow(dead_code)]
    pub fn from_f32(arg: f32) -> Self {
        Fixed(Num::from_f32(arg))
    }
    pub fn trunc(&self) -> i32 {
        self.0.trunc()
    }
    pub fn abs(self) -> Self {
        if self.0 >= Num::from_raw(0) {
            self
        } else {
            -self
        }
    }
    #[allow(dead_code)]
    pub fn sqrt(self) -> Self {
        Fixed(self.0.sqrt())
    }

    pub fn modulo(self, other: Self) -> Self {
        let remainder: Num<i32, 8> = self.0 % other.0;
        Fixed(remainder)
    }
    
    pub fn cos(self) -> Self {
        Fixed(self.0.cos())
    }
    pub fn sin(self) -> Self {
        Fixed(self.0.sin())
    }
}

impl<'de> Deserialize<'de> for Fixed {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        // Deserialize the value as a f64
        let value = f32::deserialize(deserializer)?;
        //let valuei = (value * 100.0) as i32;
        let valuen: Num<i32, 8> = Num::from_f32(value);
        //Num::new(valuei) / 100;
        // Convert the floating-point value into your Fixed point representation
        Ok(Fixed(valuen))
    }
}

///
/// BASIC OPERATIONS
///

impl Neg for Fixed {
    type Output = Self;
    fn neg(self) -> Self {
        Fixed(-self.0)
    }
}

impl Add for Fixed {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Fixed(self.0 + other.0)
    }
}

impl AddAssign for Fixed {
    fn add_assign(&mut self, other: Self) {
        self.0 = self.0 + other.0;
    }
}

impl Sub for Fixed {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Fixed(self.0 - other.0)
    }
}

// Implement SubAssign
impl SubAssign for Fixed {
    fn sub_assign(&mut self, other: Self) {
        self.0 = self.0 - other.0;
    }
}

// Implement Mul
impl Mul for Fixed {
    type Output = Self;
    fn mul(self, other: Self) -> Self {
        Fixed(self.0 * other.0)
    }
}

// Implement MulAssign
impl MulAssign for Fixed {
    fn mul_assign(&mut self, other: Self) {
        self.0 = self.0 * other.0;
    }
}

// Implement Div
impl Div for Fixed {
    type Output = Self;
    fn div(self, other: Self) -> Self {
        Fixed(self.0 / other.0)
    }
}

// Implement DivAssign
impl DivAssign for Fixed {
    fn div_assign(&mut self, other: Self) {
        self.0 = self.0 / other.0;
    }
}

// Implement `Mul` for Fixed * i32
impl Mul<i32> for Fixed {
    type Output = Self;
    fn mul(self, other: i32) -> Self {
        Fixed(self.0 * other)
    }
}

// Implement `Mul` for Fixed * usize
impl Mul<usize> for Fixed {
    type Output = Self;
    fn mul(self, other: usize) -> Self {
        Fixed(self.0 * other as i32)
    }
}

// Implement MulAssign
impl MulAssign<i32> for Fixed {
    fn mul_assign(&mut self, other: i32) {
        self.0 = self.0 * other;
    }
}

impl Div<i32> for Fixed {
    type Output = Self;
    fn div(self, other: i32) -> Self {
        Fixed(self.0 / other)
    }
}

impl DivAssign<i32> for Fixed {
    fn div_assign(&mut self, other: i32) {
        self.0 = self.0 / other;
    }
}




impl PartialEq<i32> for Fixed {
    fn eq(&self, other: &i32) -> bool {
        self.0 == Num::new(*other) // Convert `i32` to `Fixed`-compatible `Num` and compare
    }
}

impl PartialOrd<i32> for Fixed {
    fn partial_cmp(&self, other: &i32) -> Option<Ordering> {
        self.0.partial_cmp(&Num::new(*other)) // Convert `i32` to `Num` and compare
    }
}