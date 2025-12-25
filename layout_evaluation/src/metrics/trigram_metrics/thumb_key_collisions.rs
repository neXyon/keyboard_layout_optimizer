//! The bigram metric [`Thumb-Key`] assigns a cost to the finger movement within Thumb-Key for a
//! specific bigram computing the planar movement of the finger from the finger's end position
//! of the first letter to the end position of the second letter.

use super::TrigramMetric;

use keyboard_layout::{
    layout::{LayerKey, Layout},
    thumb_key::{key_to_position, key_to_movement}
};

use serde::Deserialize;
use std::cmp::{min, max};

#[derive(Clone, Deserialize, Debug)]
pub struct Parameters {
    tap_cost: f64,
    position_coincidence_bonus: f64,
    swipe_penalty: f64,
    switch_bonus: f64,
    switch_collision_penalty: f64,
    double_switch_collision_penalty: f64,
}

#[derive(Clone, Debug)]
pub struct ThumbKeyCollisions {
    tap_cost: f64,
    position_coincidence_bonus: f64,
    swipe_penalty: f64,
    switch_bonus: f64,
    switch_collision_penalty: f64,
    double_switch_collision_penalty: f64,
}

impl ThumbKeyCollisions {
    pub fn new(params: &Parameters) -> Self {
        Self {
            tap_cost: params.tap_cost,
            position_coincidence_bonus: params.position_coincidence_bonus,
            swipe_penalty: params.swipe_penalty,
            switch_bonus: params.switch_bonus,
            switch_collision_penalty: params.switch_collision_penalty,
            double_switch_collision_penalty: params.double_switch_collision_penalty,
        }
    }
}

impl TrigramMetric for ThumbKeyCollisions {
    fn name(&self) -> &str {
        "Thumb-Key Collisions"
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
        let k1_swipe_direction = key_to_movement(k1);
        let k2_swipe_direction = key_to_movement(k2);
        let k3_swipe_direction = key_to_movement(k3);

        let k1_start_pos = key_to_position(k1);
        let k2_start_pos = key_to_position(k2);
        let k3_start_pos = key_to_position(k3);

        let k1_end_pos = k1_start_pos + k1_swipe_direction;
        let k2_end_pos = k2_start_pos + k2_swipe_direction;
        let k3_end_pos = k3_start_pos + k3_swipe_direction;

        let mut k1_side = i32::from(k1.key.matrix_position.0) / 3 - 1;
        let mut k2_side = i32::from(k2.key.matrix_position.0) / 3 - 1;
        let mut k3_side = i32::from(k3.key.matrix_position.0) / 3 - 1;

        let collision12 = (k1_end_pos == k2_start_pos) | (k1_start_pos == k2_start_pos) | (k1_end_pos == k2_end_pos);
        let collision23 = (k2_end_pos == k3_start_pos) | (k2_start_pos == k3_start_pos) | (k2_end_pos == k3_end_pos);

        let min_x = min(k1_end_pos.0, k3_start_pos.0);
        let max_x = max(k1_end_pos.0, k3_start_pos.0);
        let min_y = min(k1_end_pos.1, k3_start_pos.1);
        let max_y = max(k1_end_pos.1, k3_start_pos.1);

        let collision123 = ((min_x <= k2_start_pos.0) & (k2_start_pos.0 <= max_x) & (min_y <= k2_start_pos.1) & (k2_start_pos.1 <= max_y)) | ((min_x <= k2_end_pos.0) & (k2_end_pos.0 <= max_x) & (min_y <= k2_end_pos.1) & (k2_end_pos.1 <= max_y));

        if (k1_side == 0) && (k2_side == 0) && (k3_side == 0) {
            k1_side = -1;
        }

        if k1_side == 0 {
            if collision12 {
                k1_side = k2_side;
            } else {
                k1_side = -k2_side;
            }
        }

        if k2_side == 0 {
            if k1_side != 0 {
                if collision12 {
                    k2_side = k1_side;
                } else {
                    k2_side = -k1_side;
                }
            } else {
                if collision23 {
                    k2_side = k3_side;
                } else {
                    k2_side = -k3_side;
                }
            }
        }

        if k3_side == 0 {
            if collision23 {
                k3_side = k2_side;
            } else {
                k3_side = -k2_side;
            }
        }

        if k1_side == 0 {
            if collision12 {
                k1_side = k2_side;
            } else {
                k1_side = -k2_side;
            }
        }

            /*println!("Pos: {x} {y} {z} {w}",
                x=k1_start_pos.0,
                y=k1_start_pos.1,
                z=k1.key.matrix_position.0,
                w=k1.key.matrix_position.1,
            );*/

            /*
            println!("Pos: {x1} {y1} {x2} {y2} {x3} {y3}",
                x1=k1_start_pos.0,
                y1=k1_start_pos.1,
                x2=k2_start_pos.0,
                y2=k2_start_pos.1,
                x3=k3_start_pos.0,
                y3=k3_start_pos.1,
            );

        //if ((k1_side != 1) && (k1_side != -1)) || ((k2_side != 1) && (k2_side != -1)) || ((k3_side != 1) && (k3_side != -1)) {
            println!("Sides: {k1}{k2}{k3}",
                k1=if k1_side == -1 { 'L' } else { 'R' },
                k2=if k2_side == -1 { 'L' } else { 'R' },
                k3=if k3_side == -1 { 'L' } else { 'R' },
            );
            println!("Coll: {c1} {c2} {c3}",
                c1=collision12,
                c2=collision23,
                c3=collision123,
            );
        //}
        // */

        // 2 tap + potentially swipe - 12switch bonus or 12 position coincidence bonus + 12 switch collision penalty +
        // 3 tap + potentially swipe - 23switch bonus or 23 position coincidence bonus + 23 switch collision penalty +
        // double switch collision penalty

        /*let tap_cost = 400;
        let position_coincidence_bonus = 25;
        let swipe_penalty = 90;
        let switch_bonus = 50; // made up
        let switch_collision_penalty = 70; // made up
        let double_switch_collision_penalty = 25; // made up*/

        let mut result = 2.0 * self.tap_cost;

        if (k2_swipe_direction.0 != 0) | (k2_swipe_direction.1 != 0) {
            result += self.swipe_penalty;
        }

        if k1_side != k2_side {
            result -= self.switch_bonus;

            if collision12 {
                result += self.switch_collision_penalty;
            }
        } else {
            if k1_end_pos == k2_start_pos {
                result -= self.position_coincidence_bonus;
            }
        }

        /*
        println!("Result: {r}",
            r=result,
        );
        // */

        if (k3_swipe_direction.0 != 0) | (k3_swipe_direction.1 != 0) {
            result += self.swipe_penalty;
        }

        if k2_side != k3_side {
            result -= self.switch_bonus;

            if collision23 {
                result += self.switch_collision_penalty;
            }
        } else {
            if k2_end_pos == k3_start_pos {
                result -= self.position_coincidence_bonus;
            }
        }

        if !collision12 && !collision23 && collision123 && (k1_side == k3_side) && (k1_side != k2_side) {
            result += self.double_switch_collision_penalty;
        }

        /*
        println!("Result: {r}",
            r=result,
        );
        // */


        /*let movement1 = k2_start_pos - k1_end_pos;
        let movement2 = k2_swipe_direction;*/

        Some(result * weight) //movement1.length() + movement2.length()) * weight
    }
}
