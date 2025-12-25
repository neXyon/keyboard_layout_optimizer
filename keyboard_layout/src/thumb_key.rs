use crate::key::MatrixPosition;
use crate::layout::{LayerKey};

use std::ops::{Add, Sub, Mul, Div, Rem};

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Vec2(pub i8, pub i8);

impl Add for Vec2 {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Vec2(self.0 + other.0, self.1 + other.1)
    }
}

impl Add<i8> for Vec2 {
    type Output = Self;

    fn add(self, other: i8) -> Self {
        Vec2(self.0 + other, self.1 + other)
    }
}

impl Sub for Vec2 {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Vec2(self.0 - other.0, self.1 - other.1)
    }
}

impl Sub<i8> for Vec2 {
    type Output = Self;

    fn sub(self, other: i8) -> Self {
        Vec2(self.0 - other, self.1 - other)
    }
}

impl Mul<i8> for Vec2 {
    type Output = Self;

    fn mul(self, other: i8) -> Self {
        Vec2(self.0 * other, self.1 * other)
    }
}

impl Div<i8> for Vec2 {
    type Output = Self;

    fn div(self, other: i8) -> Self {
        Vec2(self.0 / other, self.1 / other)
    }
}

impl Rem<i8> for Vec2 {
    type Output = Self;

    fn rem(self, other: i8) -> Self {
        Vec2(self.0 % other, self.1 % other)
    }
}

impl Vec2 {
    pub fn length(&self) -> f32 {
        let x = self.0 as f32;
        let y = self.1 as f32;
        (x * x + y * y).sqrt()
    }
}

impl From<MatrixPosition> for Vec2 {
    fn from(pos: MatrixPosition) -> Self {
        Vec2(pos.0 as i8, pos.1 as i8)
    }
}

pub fn key_to_position(k: &LayerKey) -> Vec2
{
    Vec2::from(k.key.matrix_position) / 3
}

pub fn key_to_movement(k: &LayerKey) -> Vec2
{
    /*let position = Vec2::from(k.key.matrix_position) / 3;
    let center = position * 3 + 1;*/
    Vec2::from(k.key.matrix_position) % 3 - 1//- center
}
