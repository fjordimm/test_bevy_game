use std::array;

use bevy::prelude::*;

use crate::game::util::make_image_from_array2d;

pub(super) fn create_terrain_texture() -> Image {
    let mut pixels: [[[f32; 4]; 10]; 10] =
        array::from_fn(|_| array::from_fn(|_| [0.0, 1.0, 0.0, 1.0]));

    for c in 0..pixels.len() {
        for r in 0..pixels[0].len() {
            if c == r {
                pixels[c][r] = [1.0, 0.0, 0.0, 1.0];
            }
        }
    }

    make_image_from_array2d(&pixels)
}
