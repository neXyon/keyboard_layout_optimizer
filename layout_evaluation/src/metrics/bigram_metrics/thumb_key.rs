//! The bigram metric [`Thumb-Key`] assigns a cost to the finger movement within Thumb-Key for a
//! specific bigram computing the planar movement of the finger from the finger's end position
//! of the first letter to the end position of the second letter.

use super::BigramMetric;

use keyboard_layout::{
    key::MatrixPosition,
    layout::{LayerKey, Layout},
};

use serde::Deserialize;

use std::ops::{Add, Sub, Mul, Div, Rem};

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
struct Vec2(i8, i8);

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
    fn length(&self) -> f32 {
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

#[derive(Clone, Deserialize, Debug)]
pub struct Parameters {}

#[derive(Clone, Debug)]
pub struct ThumbKey {}

impl ThumbKey {
    pub fn new(_params: &Parameters) -> Self {
        Self {}
    }
}

fn key_to_position(k: &LayerKey) -> Vec2
{
    Vec2::from(k.key.matrix_position) / 3
}

fn key_to_movement(k: &LayerKey) -> Vec2
{
    let position = Vec2::from(k.key.matrix_position) / 3;
    let center = position * 3 + 1;
    Vec2::from(k.key.matrix_position) - center
}

impl BigramMetric for ThumbKey {
    fn name(&self) -> &str {
        "Thumb-Key"
    }

    #[inline(always)]
    fn individual_cost(
        &self,
        k1: &LayerKey,
        k2: &LayerKey,
        weight: f64,
        _total_weight: f64,
        _layout: &Layout,
    ) -> Option<f64> {
        let k1_swipe_direction = key_to_movement(k1);
        let k2_swipe_direction = key_to_movement(k2);

        let k1_start_pos = key_to_position(k1);
        let k2_start_pos = key_to_position(k2);

        let k1_end_pos = k1_start_pos + k1_swipe_direction;
        let k2_end_pos = k2_start_pos + k2_swipe_direction;
        
        let movement1 = k2_start_pos - k1_end_pos;
        let movement2 = k2_swipe_direction;

        Some(f64::from(movement1.length() + movement2.length()) * weight)
    }
}
