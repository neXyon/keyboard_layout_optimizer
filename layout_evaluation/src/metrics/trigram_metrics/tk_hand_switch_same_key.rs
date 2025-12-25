//! The bigram metric [`Thumb-Key`] assigns a cost to the finger movement within Thumb-Key for a
//! specific bigram computing the planar movement of the finger from the finger's end position
//! of the first letter to the end position of the second letter.

use super::TrigramMetric;

use keyboard_layout::{
    layout::{LayerKey, Layout},
    thumb_key::{key_to_position, key_to_movement}
};

use serde::Deserialize;

#[derive(Clone, Deserialize, Debug)]
pub struct Parameters {
}

#[derive(Clone, Debug)]
pub struct TKHandSwitchSameKey {
}

impl TKHandSwitchSameKey {
    pub fn new(_params: &Parameters) -> Self {
        Self {}
    }
}

impl TrigramMetric for TKHandSwitchSameKey {
    fn name(&self) -> &str {
        "Thumb-Key Hand Switch Same Key"
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

        let k1_end_pos = k1_start_pos + k1_swipe_direction;

        // could consider also further moves, but from experience that doesn't happen:
        // 1. observing myself: at the end of swiping, the finger movement is stopped,
        //    even if it would move in the correct direction
        // 2. swiping is harder due to the friction so it's stopped after a short distance,
        //    though it is possible to swipe all the way to the target

        Some(if k1_end_pos == k3_start_pos { -weight } else { 0.0 })
    }
}
