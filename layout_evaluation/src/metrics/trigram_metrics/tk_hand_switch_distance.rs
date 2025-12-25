//! The bigram metric [`Thumb-Key`] assigns a cost to the finger movement within Thumb-Key for a
//! specific bigram computing the planar movement of the finger from the finger's end position
//! of the first letter to the end position of the second letter.

use super::TrigramMetric;

use keyboard_layout::{
    layout::{LayerKey, Layout},
    thumb_key::{key_to_position, key_to_movement, Vec2}
};

use serde::Deserialize;

#[derive(Clone, Deserialize, Debug)]
pub struct Parameters {
    swipe_distance: f64,
}

#[derive(Clone, Debug)]
pub struct TKHandSwitchDistance {
    swipe_distance: f64,
}

impl TKHandSwitchDistance {
    pub fn new(params: &Parameters) -> Self {
        Self {
            swipe_distance: params.swipe_distance,
        }
    }
}

impl TrigramMetric for TKHandSwitchDistance {
    fn name(&self) -> &str {
        "Thumb-Key Hand Switch Distance"
    }

    #[inline(always)]
    fn individual_cost(
        &self,
        k1: &LayerKey,
        k2: &LayerKey,
        k3: &LayerKey,
        weight: f64,
        _total_weight: f64,
        _layout: &Layout,
    ) -> Option<f64> {
        if (k1.key.hand == k2.key.hand) || (k1.key.hand != k3.key.hand) {
            return Some(0.0);
        }

        let k1_swipe_direction = key_to_movement(k1);

        let k1_start_pos = key_to_position(k1);
        let k3_start_pos = key_to_position(k3);

        let k1_end_pos = Vec2::<f64>::from(k1_start_pos) + Vec2::<f64>::from(k1_swipe_direction) * self.swipe_distance;
        
        let movement = Vec2::<f64>::from(k3_start_pos) - k1_end_pos;

        Some(movement.length() * weight)
    }
}
