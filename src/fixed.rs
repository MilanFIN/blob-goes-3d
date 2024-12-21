

use agb::fixnum::Num;
use serde::Deserialize;
use serde::Deserializer;
use core::ops::Neg;
use core::ops::{Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, DivAssign};


#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct Fixed(Num<i32, 8>);


pub const fn i32_to_fixed(m:i32) -> Fixed {
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
    pub fn from_f32(arg: f32) -> Self {
        Fixed(Num::from_f32(arg))
    }
    pub fn trunc(&self) -> i32 {
        self.0.trunc()
    }
    pub fn abs(self) -> Self {
        if self.0 >= Num::from_raw(0) {
            self
        } 
        else {
            -self
        }
    }
}

// Implement `Deserialize` for the wrapper
// Create a fixed, and divide by 100
impl<'de> Deserialize<'de> for Fixed {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = i32::deserialize(deserializer)?;
        Ok(Fixed(Num::new(value) / 100))
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

///
/// Num traits 
/// 

impl Fixed {
    pub fn cos(self) -> Self {
        Fixed(self.0.cos())
    }
    pub fn sin(self) -> Self {
        Fixed(self.0.sin())
    }
}