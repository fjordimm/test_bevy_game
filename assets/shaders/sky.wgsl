#import bevy_pbr::forward_io::VertexOutput
#import "shaders/util.wgsl"::PI;
#import "shaders/util.wgsl"::gradient_noise;
#import "shaders/util.wgsl"::reduce_banding;
#import "shaders/util.wgsl"::smoothstep_skew_left;
#import "shaders/util.wgsl"::smoothstep_skew_right;
#import "shaders/util.wgsl"::hash3;
#import "shaders/util.wgsl"::bell;

const DAY_ZENITH_COLOR = vec3<f32>(0.19, 0.58, 0.97);
const DAY_HORIZON_COLOR = vec3<f32>(0.28, 0.66, 1.0);
const DAY_HORIZON_SQUISH_FACTOR: f32 = 2.5;
const NIGHT_ZENITH_COLOR = vec3<f32>(0.0005, 0.001, 0.002);
const NIGHT_HORIZON_COLOR = vec3<f32>(0.0024, 0.0048, 0.0068);
const NIGHT_HORIZON_SQUISH_FACTOR: f32 = 1.7;

const TWILIGHT_OFFSET: f32 = 0.25;

const SUN_COLOR = vec3<f32>(1.0, 0.8, 0.2);
const SUN_SIZE_INV: f32 = 1500.0;
const SUN_SOFTNESS_INV: f32 = 2.9;

const SUNSET_COLOR = vec3<f32>(1.0, 0.65, 0.3);
const SUNSET_BRIGHTNESS: f32 = 0.13;
const SUNSET_HORIZON_CURVE_INV: f32 = 15.0;
const SUNSET_CURVE_INV: f32 = 0.11;
const SUNSET_TIME_RANGE_INV: f32 = 2.9;
const SUNSET_TIME_RANGE_OFFSET: f32 = 0.17;

const STARS_COLOR = vec3<f32>(1.0, 1.0, 1.0);
const STARS_SCALE: f32 = 130.0;
const STARS_DENSITY: f32 = 0.93;
const STARS_TIME_RANGE_FACTOR: f32 = 17.0;

@group(#{MATERIAL_BIND_GROUP}) @binding(0) var<uniform> sun_position: vec3<f32>;

@fragment
fn fragment(vertout: VertexOutput) -> @location(0) vec4<f32> {
    var color = vec3<f32>(0.0, 0.0, 0.0);

    let sun_pos = normalize(sun_position);
    let dir: vec3<f32> = normalize(vertout.world_position.xyz);

    let day_amount = smoothstep(0.0, 1.0, sun_pos.y + TWILIGHT_OFFSET);

    let day_color = mix(DAY_HORIZON_COLOR, DAY_ZENITH_COLOR, smoothstep_skew_left(0.0, 1.0, DAY_HORIZON_SQUISH_FACTOR, dir.y));
    let night_color = mix(NIGHT_HORIZON_COLOR, NIGHT_ZENITH_COLOR, smoothstep_skew_left(0.0, 1.0, NIGHT_HORIZON_SQUISH_FACTOR, dir.y));
    color += mix(night_color, day_color, day_amount);

    // Sun
    color += SUN_COLOR * pow(2.0 * smoothstep_skew_right(0.0, 1.0, SUN_SIZE_INV, dot(dir, sun_pos)), SUN_SOFTNESS_INV);

    // Sunset
    let sunset_horizon_glow = bell(SUNSET_HORIZON_CURVE_INV * dir.y);
    let sunset_side = -1.0 / ((dot(dir, sun_pos) * 0.5 + 0.5) - 1.0 - SUNSET_CURVE_INV);
    let sunset_time_multiplier = (1.0 - smoothstep_skew_left(0.0, 1.0, SUNSET_TIME_RANGE_INV, abs(sun_pos.y - SUNSET_TIME_RANGE_OFFSET)));
    color += SUNSET_BRIGHTNESS * SUNSET_COLOR * (sunset_horizon_glow * sunset_side * sunset_time_multiplier);

    // Stars
    color += STARS_COLOR * local_star_value(dir) * clamp(pow(1.0 - day_amount, STARS_TIME_RANGE_FACTOR), 0.0, 1.0);

    color += reduce_banding(vertout.position.xy);
    return vec4<f32>(color, 1.0);
}

fn local_star_value(dir: vec3<f32>) -> f32 {
    let grid = dir * STARS_SCALE;

    let h = hash3(floor(grid));
    if h > STARS_DENSITY {
        let sdist = length(fract(grid) - 0.5); // Distance from the center of the star.
        return clamp(-pow(abs(sdist * 1.5) - 1.0, 5.0), 0.0, 1.0);
    } else {
        return 0.0;
    }
}
