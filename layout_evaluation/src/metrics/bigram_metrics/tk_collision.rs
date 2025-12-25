//! The bigram metric [`Thumb-Key`] assigns a cost to the finger movement within Thumb-Key for a
//! specific bigram computing the planar movement of the finger from the finger's end position
//! of the first letter to the end position of the second letter.

use super::BigramMetric;

use keyboard_layout::{
    layout::{LayerKey, Layout},
    thumb_key::{key_to_position, key_to_movement}
};

use serde::Deserialize;

#[derive(Clone, Deserialize, Debug)]
pub struct Parameters {
    weak_collision_weight: f64,
}

#[derive(Clone, Debug)]
pub struct TKCollision {
    weak_collision_weight: f64,
}

impl TKCollision {
    pub fn new(params: &Parameters) -> Self {
        Self {
            weak_collision_weight: params.weak_collision_weight
        }
    }
}

impl BigramMetric for TKCollision {
    fn name(&self) -> &str {
        "Thumb-Key Collision"
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
        if k1.key.hand == k2.key.hand {
            return Some(0.0);
        }

        let k1_swipe_direction = key_to_movement(k1);
        let k2_swipe_direction = key_to_movement(k2);

        let k1_start_pos = key_to_position(k1);
        let k2_start_pos = key_to_position(k2);

        let k1_end_pos = k1_start_pos + k1_swipe_direction;
        let k2_end_pos = k2_start_pos + k2_swipe_direction;

        let strong_collision = k1_end_pos == k2_start_pos;
        let weak_collision = (k1_start_pos == k2_start_pos) || (k1_end_pos == k2_end_pos);

        Some(if strong_collision { weight } else if weak_collision { self.weak_collision_weight * weight } else { 0.0 })
    }
}
