use bevy::prelude::*;

use crate::game::{
    playing_state::terrain::{plugin::CW, terrain_func::TerrainFunc},
    util::make_image_from_array2d,
};

const TEXTURE_SIZE: usize = 20;

pub(super) fn create_terrain_texture(
    terrain_func: &TerrainFunc,
    scale: f32,
    off_x: i64,
    off_z: i64,
) -> Image {
    let off_x_real = CW as f32 * scale * off_x as f32;
    let off_z_real = CW as f32 * scale * off_z as f32;

    let mut pixels: [[[f32; 4]; TEXTURE_SIZE]; TEXTURE_SIZE] =
        [[[0.0, 0.0, 0.0, 1.0]; TEXTURE_SIZE]; TEXTURE_SIZE];

    for c in 0..TEXTURE_SIZE {
        for r in 0..TEXTURE_SIZE {
            let cf = scale * c as f32 * (CW as f32 / (TEXTURE_SIZE as f32 - 1.0));
            let rf = scale * r as f32 * (CW as f32 / (TEXTURE_SIZE as f32 - 1.0));

            let color = terrain_func
                .color_at(cf + off_x_real, rf + off_z_real)
                .to_srgba();

            pixels[c][r] = [color.red, color.green, color.blue, 1.0];
        }
    }

    make_image_from_array2d(&pixels)
}
