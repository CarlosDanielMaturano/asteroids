use crate::screen::{SCREEN_HEIGHT, SCREEN_WIDTH};
use std::fmt::Debug;
use std::ops::{Add, AddAssign, Div, Mul, Sub, SubAssign};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vector2(pub f64, pub f64);

impl Vector2 {
    pub fn as_i32(&self) -> (i32, i32) {
        (self.0 as i32, self.1 as i32)
    }
    pub fn wrap(&mut self) {
        let w = SCREEN_WIDTH as f64;
        let h = SCREEN_HEIGHT as f64;
        if self.0 >= w  {
            self.0 = self.0 - w;
        }
        if self.0 < 0.0 {
            self.0 = w + self.0
        }

        if self.1 > h  {
            self.1 = self.1 - h;
        }
        if self.1 < 0.0 {
            self.1 = h - self.1;
        }
    }
}

impl Add for Vector2 {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Vector2(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl AddAssign for Vector2{
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
        self.1 += rhs.1;
    }
}

impl Sub for Vector2 {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Vector2(self.0 - rhs.0, self.1 - rhs.1)
    }
}

impl SubAssign for Vector2{
    fn sub_assign(&mut self, rhs: Self) {
        self.0 -= rhs.0;
        self.1 -= rhs.1;
    }
}

impl Mul<Vector2> for Vector2 {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        Vector2(self.0 * rhs.0, self.1 * rhs.1)
    }
}

impl Mul<f64> for Vector2 {
    type Output = Self;
    fn mul(self, rhs: f64) -> Self::Output {
        Vector2(self.0 * rhs, self.1 * rhs)
    }
}

impl Div for Vector2 {
    type Output = Self;
    fn div(self, rhs: Self) -> Self::Output {
        Vector2(self.0 / rhs.0, self.1 / rhs.1)
    }
}
