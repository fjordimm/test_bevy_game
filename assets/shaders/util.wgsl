fn smoothstep_skew_left(edge0: f32, edge1: f32, skew: f32, x: f32) -> f32 {
    return smoothstep(edge0, edge1, skew * x);
}

fn smoothstep_skew_right(edge0: f32, edge1: f32, skew: f32, x: f32) -> f32 {
    return smoothstep(edge0, edge1, skew * (x - 1.0) + 1.0);
}