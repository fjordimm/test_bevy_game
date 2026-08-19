
const PI: f32 = 3.141592653589793;

/* Gradient noise from Jorge Jimenez's presentation: */
/* http://www.iryoku.com/next-generation-post-processing-in-call-of-duty-advanced-warfare */
fn gradient_noise(uv: vec2<f32>) -> f32 {
    return fract(52.9829189 * fract(dot(uv, vec2(0.06711056, 0.00583715))));
}

const REDUCE_BANDING_AMOUNT: f32 = 1.0; // Must be in range [0.0, 1.0];
const REDUCE_BANDING_AMOUNT_HALF: f32 = 0.5 * REDUCE_BANDING_AMOUNT;
fn reduce_banding(pixel_pos: vec2<f32>) -> f32 {
    return (REDUCE_BANDING_AMOUNT / 255.0) * gradient_noise(pixel_pos) - (REDUCE_BANDING_AMOUNT_HALF / 255.0);
}

// Warning: only works for 0 <= x <= 1
fn inv_smoothstep(x: f32) -> f32 {
    return 0.5 - sin(asin(1.0 - 2.0 * x) / 3.0);
}

// Warning: only works for skew >= 1
// 1 is no skew
fn smoothstep_skew_left(edge0: f32, edge1: f32, skew: f32, x: f32) -> f32 {
    return smoothstep(edge0, edge1, skew * x);
}

// Warning: only works for skew >= 1
// 1 is no skew
fn smoothstep_skew_right(edge0: f32, edge1: f32, skew: f32, x: f32) -> f32 {
    return smoothstep(edge0, edge1, skew * (x - 1.0) + 1.0);
}

// Warning: only works for r >= 1
// Larger r is closer to linear
fn arc_step_up(r: f32, x: f32) -> f32 {
    return (1.0 - r) + sqrt(r * r + (1.0 - r) * (1.0 - r) - (x - r) * (x - r));
}

// Warning: only works for r >= 1
// Larger r is closer to linear
fn arc_step_down(r: f32, x: f32) -> f32 {
    return (1.0 + r) - sqrt(2.0 * r * r + 2.0 * r + 1.0 - (x + r) * (x + r));
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

fn lerp_remap(
    x: f32,
    lower_bound_from: f32,
    upper_bound_from: f32,
    lower_bound_to: f32,
    upper_bound_to: f32,
) -> f32 {
    return ((x - lower_bound_from) / upper_bound_from) * (upper_bound_to - lower_bound_to) + lower_bound_to;
}
