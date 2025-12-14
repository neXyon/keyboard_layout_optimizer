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
pub struct Parameters {}

#[derive(Clone, Debug)]
pub struct ThumbKey {}

impl ThumbKey {
    pub fn new(_params: &Parameters) -> Self {
        Self {}
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
        let k1_swipe_direction = key_to_movement(k1);
        let k2_swipe_direction = key_to_movement(k2);

        let k1_start_pos = key_to_position(k1);
        let k2_start_pos = key_to_position(k2);

        let k1_end_pos = k1_start_pos + k1_swipe_direction;
        //let k2_end_pos = k2_start_pos + k2_swipe_direction;
        
        let movement1 = k2_start_pos - k1_end_pos;
        let movement2 = k2_swipe_direction;

        Some(f64::from(movement1.length() + movement2.length()) * weight)
    }
}
