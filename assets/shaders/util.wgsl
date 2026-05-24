
const PI: f32 = 3.141592653589793;

/* Gradient noise from Jorge Jimenez's presentation: */
/* http://www.iryoku.com/next-generation-post-processing-in-call-of-duty-advanced-warfare */
fn gradient_noise(uv: vec2<f32>) -> f32 {
    return fract(52.9829189 * fract(dot(uv, vec2(0.06711056, 0.00583715))));
}

fn reduce_banding(pixel_pos: vec2<f32>) -> f32 {
    return (1.0 / 255.0) * gradient_noise(pixel_pos) - (0.5 / 255.0);
}

// Warning: only works for 0 <= x <= 1
fn inv_smoothstep(x: f32) -> f32 {
    return 0.5 - sin(asin(1.0 - 2.0 * x) / 3.0);
}

// Skew should be >= 1. 1 is no skew.
fn smoothstep_skew_left(edge0: f32, edge1: f32, skew: f32, x: f32) -> f32 {
    return smoothstep(edge0, edge1, skew * x);
}

// Skew should be >= 1. 1 is no skew.
fn smoothstep_skew_right(edge0: f32, edge1: f32, skew: f32, x: f32) -> f32 {
    return smoothstep(edge0, edge1, skew * (x - 1.0) + 1.0);
}

fn hash2(p: vec2<f32>) -> f32 {
    return fract(
        sin(dot(p, vec2<f32>(127.1, 311.7))) * 43758.5453
    );
}

fn hash3(p: vec3<f32>) -> f32 {
    return fract(
        sin(dot(p, vec3<f32>(12.9898, 78.233, 37.719)))
        * 43758.5453
    );
}

fn bell(x: f32) -> f32 {
    return 1.0 / (1.0 + x * x);
}
