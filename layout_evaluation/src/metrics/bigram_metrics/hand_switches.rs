//! The bigram metric [`Thumb-Key`] assigns a cost to the finger movement within Thumb-Key for a
//! specific bigram computing the planar movement of the finger from the finger's end position
//! of the first letter to the end position of the second letter.

use super::BigramMetric;

use keyboard_layout::{
    layout::{LayerKey, Layout},
};

use serde::Deserialize;

#[derive(Clone, Deserialize, Debug)]
pub struct Parameters {
}

#[derive(Clone, Debug)]
pub struct HandSwitches {
}

impl HandSwitches {
    pub fn new(_params: &Parameters) -> Self {
        Self {}
    }
}

impl BigramMetric for HandSwitches {
    fn name(&self) -> &str {
        "Hand Switches"
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
        Some(if k1.key.hand != k2.key.hand { -weight } else { 0.0 })
    }
}
