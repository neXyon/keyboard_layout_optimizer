//! The bigram metric [`Thumb-Key`] assigns a cost to the finger movement within Thumb-Key for a
//! specific bigram computing the planar movement of the finger from the finger's end position
//! of the first letter to the end position of the second letter.

use super::BigramMetric;

use keyboard_layout::{
    key::MatrixPosition,
    layout::{LayerKey, Layout},
};

use serde::Deserialize;

use std::ops::{Add, Sub};

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
struct Vec2(i8, i8);

impl Add for Vec2 {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Vec2(self.0 + other.0, self.1 + other.1)
    }
}

impl Sub for Vec2 {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Vec2(self.0 - other.0, self.1 - other.1)
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

fn layer_to_movement(layer: u8) -> Vec2
{
    // swipe-directions: no-swipe, up-left, up, up-right, left, right, down-left, down, down-right
    match layer
    {
        1 => Vec2(-1, -1),
        2 => Vec2(0, -1),
        3 => Vec2(1, -1),
        4 => Vec2(-1, 0),
        5 => Vec2(1, 0),
        6 => Vec2(-1, 1),
        7 => Vec2(0, 1),
        8 => Vec2(1, 1),
        _ => Vec2(0, 0),
    }
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
        let k1_swipe_direction = layer_to_movement(k1.layer);
        let k2_swipe_direction = layer_to_movement(k2.layer);

        let k1_start_pos = Vec2::from(k1.key.matrix_position);
        let k2_start_pos = Vec2::from(k2.key.matrix_position);

        let k1_end_pos = k1_start_pos + k1_swipe_direction;
        let k2_end_pos = k2_start_pos + k2_swipe_direction;
        
        let movement1 = k2_start_pos - k1_end_pos;
        let movement2 = k2_swipe_direction;

        Some(f64::from(movement1.length() + movement2.length()) * weight)
    }
}
