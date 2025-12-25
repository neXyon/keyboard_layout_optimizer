use crate::key::MatrixPosition;
use crate::layout::{LayerKey};

use std::ops::{Add, Sub, Mul, Div, Rem};

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Vec2<T>(pub T, pub T);

impl<T: std::ops::Add<Output = T>> Add for Vec2<T> {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Vec2(self.0 + other.0, self.1 + other.1)
    }
}

impl<T: std::ops::Add<Output = T> + Clone> Add<T> for Vec2<T> {
    type Output = Self;

    fn add(self, other: T) -> Self {
        Vec2(self.0 + other.clone(), self.1 + other)
    }
}

impl<T: std::ops::Sub<Output = T>> Sub for Vec2<T> {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Vec2(self.0 - other.0, self.1 - other.1)
    }
}

impl<T: std::ops::Sub<Output = T> + Clone> Sub<T> for Vec2<T> {
    type Output = Self;

    fn sub(self, other: T) -> Self {
        Vec2(self.0 - other.clone(), self.1 - other)
    }
}

impl<T: std::ops::Mul<Output = T> + Clone> Mul<T> for Vec2<T> {
    type Output = Self;

    fn mul(self, other: T) -> Self {
        Vec2(self.0 * other.clone(), self.1 * other)
    }
}

impl<T: std::ops::Div<Output = T> + Clone> Div<T> for Vec2<T> {
    type Output = Self;

    fn div(self, other: T) -> Self {
        Vec2(self.0 / other.clone(), self.1 / other)
    }
}

impl<T: std::ops::Rem<Output = T> + Clone> Rem<T> for Vec2<T> {
    type Output = Self;

    fn rem(self, other: T) -> Self {
        Vec2(self.0 % other.clone(), self.1 % other)
    }
}

impl<T: Copy> Vec2<T> {
    pub fn length(&self) -> f64 where f64: From<T> {
        let x = f64::from(self.0);
        let y = f64::from(self.1);
        (x * x + y * y).sqrt()
    }
}

impl From<Vec2<i8>> for Vec2<f64> {
    fn from(v: Vec2<i8>) -> Self {
        Vec2 {
            0: v.0.into(),
            1: v.1.into(),
        }
    }
}

impl From<MatrixPosition> for Vec2<i8> {
    fn from(pos: MatrixPosition) -> Self {
        Vec2(pos.0 as i8, pos.1 as i8)
    }
}

pub fn key_to_position(k: &LayerKey) -> Vec2<i8>
{
    Vec2::from(k.key.matrix_position) / 3
}

pub fn key_to_movement(k: &LayerKey) -> Vec2<i8>
{
    Vec2::from(k.key.matrix_position) % 3 - 1
}
