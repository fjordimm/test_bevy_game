#import bevy_pbr::forward_io::VertexOutput
#import "shaders/util.wgsl"::PI;
#import "shaders/util.wgsl"::gradient_noise;
#import "shaders/util.wgsl"::reduce_banding;
#import "shaders/util.wgsl"::smoothstep_skew_left;
#import "shaders/util.wgsl"::smoothstep_skew_right;
#import "shaders/util.wgsl"::hash3;

const DAY_ZENITH_COLOR = vec3<f32>(0.19, 0.58, 0.97);
const DAY_HORIZON_COLOR = vec3<f32>(0.28, 0.66, 1.0);
const DAY_HORIZON_SQUISH_FACTOR = 2.5;
const NIGHT_ZENITH_COLOR = vec3<f32>(0.0005, 0.001, 0.002);
const NIGHT_HORIZON_COLOR = vec3<f32>(0.002, 0.004, 0.006);
const NIGHT_HORIZON_SQUISH_FACTOR = 1.0;

const TWIGHTLIGHT_OFFSET = 0.05;

const SUN_COLOR = vec3<f32>(1.0, 0.8, 0.2);
const INV_SUN_SIZE = 1500.0;
const INV_SUN_SOFTNESS = 2.9;

const SUNSET_COLOR = vec3<f32>(1.0, 0.65, 0.3);
const SUNSET_BRIGHTNESS = 0.16;

const STARS_COLOR = vec3<f32>(1.0, 1.0, 1.0);
const STARS_SCALE = 130.0;
const STARS_DENSITY = 0.93;

@group(#{MATERIAL_BIND_GROUP}) @binding(0) var<uniform> sun_position: vec3<f32>;

@fragment
fn fragment(vertout: VertexOutput) -> @location(0) vec4<f32> {
    var color = vec3<f32>(0.0, 0.0, 0.0);

    let sun_pos = normalize(sun_position);
    let dir: vec3<f32> = normalize(vertout.world_position.xyz);

    let day_amount = clamp(sun_pos.y + TWIGHTLIGHT_OFFSET, 0.0, 1.0);

    let day_color = mix(DAY_HORIZON_COLOR, DAY_ZENITH_COLOR, smoothstep_skew_left(0.0, 1.0, DAY_HORIZON_SQUISH_FACTOR, dir.y));
    let night_color = mix(NIGHT_HORIZON_COLOR, NIGHT_ZENITH_COLOR, smoothstep_skew_left(0.0, 1.0, NIGHT_HORIZON_SQUISH_FACTOR, dir.y));
    color += mix(night_color, day_color, day_amount);

    // Sun
    color += SUN_COLOR * pow(2.0 * smoothstep_skew_right(0.0, 1.0, INV_SUN_SIZE, dot(dir, sun_pos)), INV_SUN_SOFTNESS);

    // Sunset
    color += SUNSET_BRIGHTNESS * SUNSET_COLOR
        * (1.0 - pow(max(0.0, -sun_pos.y), 0.03))
        * (3.0 * pow(50.0, dot(dir, sun_pos) - 1.2) * 0.01 / (0.01 + pow(dir.y, 2.0))
           + 0.3 * pow(max(0.0, dot(dir, sun_pos)), 25.0));

    // Stars
    color += STARS_COLOR * local_star_value(dir) * clamp(pow(1.0 - day_amount, 30.0), 0.0, 1.0);

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
