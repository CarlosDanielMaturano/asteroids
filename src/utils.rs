use std::fmt::Debug;
use std::ops::{Add, Div, Mul, Sub};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vector2(pub f64, pub f64);

impl Vector2 {
    pub fn empty() -> Self {
        Self (0.0, 0.0)
    }
    pub fn as_i32(&self) -> (i32, i32) {
        (self.0 as i32, self.1 as i32)
    }
}

impl Add for Vector2 {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Vector2(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl Sub for Vector2 {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Vector2(self.0 - rhs.0, self.1 - rhs.1)
    }
}

impl Mul for Vector2 {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        Vector2(self.0 * rhs.0, self.1 * rhs.1)
    }
}

impl Div for Vector2 {
    type Output = Self;
    fn div(self, rhs: Self) -> Self::Output {
        Vector2(self.0 / rhs.0, self.1 / rhs.1)
    }
}
