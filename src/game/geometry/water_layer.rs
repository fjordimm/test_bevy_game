/* This is used instead of a 4-vertex plane to fix floating-point precision rendering issues. */

use bevy::{asset::RenderAssetUsages, prelude::*};
use bevy_mesh::{Indices, PrimitiveTopology};

const N: u32 = 16; // Should be even.
const HALF_N: u32 = N / 2;
const SQUARE_SIZE: f32 = 1. / (HALF_N as f32);

#[allow(unused)]
pub fn water_layer() -> Mesh {
    let mut positions = Vec::<[f32; 3]>::with_capacity(((N + 1) * (N + 1)) as usize);
    let mut triangles = Vec::<u32>::with_capacity((6 * (N + 1) * (N + 1)) as usize);

    for c in 0..=N {
        for r in 0..=N {
            let cf = (c as f32 - HALF_N as f32) * SQUARE_SIZE;
            let rf = (r as f32 - HALF_N as f32) * SQUARE_SIZE;

            positions.push([cf, 0.0, rf]);

            if c < N && r < N {
                let index = c * (N + 1) + r;
                triangles.extend_from_slice(&[
                    index,
                    index + 1,
                    index + (N + 1),
                    index + (N + 1) + 1,
                    index + (N + 1),
                    index + 1,
                ]);
            }
        }
    }

    Mesh::new(
        PrimitiveTopology::TriangleList,
        RenderAssetUsages::RENDER_WORLD,
    )
    .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, positions)
    .with_inserted_indices(Indices::U32(triangles))
}
