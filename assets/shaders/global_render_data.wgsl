struct GlobalRenderData {
    time_elapsed: f32,
    sun_position: vec3<f32>,
    sky_rotation_inv: mat3x3<f32>,
    cam_is_underwater: u32,
    _padding: vec2<f32>,
}
