fn reduce_banding(pixel_pos: vec2<f32>) -> f32 {
    return (1.0 / 255.0) * _gradient_noise(pixel_pos) - (0.5 / 255.0);
}

/* Gradient noise from Jorge Jimenez's presentation: */
/* http://www.iryoku.com/next-generation-post-processing-in-call-of-duty-advanced-warfare */
fn _gradient_noise(uv: vec2<f32>) -> f32 {
    return fract(52.9829189 * fract(dot(uv, vec2(0.06711056, 0.00583715))));
}

fn smoothstep_skew_left(edge0: f32, edge1: f32, skew: f32, x: f32) -> f32 {
    return smoothstep(edge0, edge1, skew * x);
}

fn smoothstep_skew_right(edge0: f32, edge1: f32, skew: f32, x: f32) -> f32 {
    return smoothstep(edge0, edge1, skew * (x - 1.0) + 1.0);
}