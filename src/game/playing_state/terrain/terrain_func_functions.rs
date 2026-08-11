#![allow(unused)]

pub fn lerp_remap(
    x: f64,
    lower_bound_from: f64,
    upper_bound_from: f64,
    lower_bound_to: f64,
    upper_bound_to: f64,
) -> f64 {
    ((x - lower_bound_from) / upper_bound_from) * (upper_bound_to - lower_bound_to) + lower_bound_to
}

pub fn sigmoid(x: f64) -> f64 {
    1. / (1. + 2f64.powf(-x))
}
