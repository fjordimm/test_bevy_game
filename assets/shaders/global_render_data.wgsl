struct GlobalRenderData {
    time_elapsed: f32,
    sun_position: vec3<f32>,
    sky_rotation_inv: mat3x3<f32>,
    _padding: vec3<f32>,
}