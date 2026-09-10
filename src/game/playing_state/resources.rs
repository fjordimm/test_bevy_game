use bevy::prelude::*;

#[derive(Resource)]
pub struct RenderingResolutionScale(pub f32);

impl Default for RenderingResolutionScale {
    fn default() -> Self {
        Self(0.5)
    }
}
