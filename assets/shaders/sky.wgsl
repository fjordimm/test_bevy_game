#import bevy_pbr::{
    mesh_functions,
    view_transformations::position_world_to_clip,
}
#import "shaders/util.wgsl"::PI;
#import "shaders/util.wgsl"::gradient_noise;
#import "shaders/util.wgsl"::reduce_banding;
#import "shaders/util.wgsl"::smoothstep_skew_left;
#import "shaders/util.wgsl"::smoothstep_skew_right;
#import "shaders/util.wgsl"::arc_step_up;
#import "shaders/util.wgsl"::hash3;
#import "shaders/util.wgsl"::bell;

const DAY_ZENITH_COLOR = vec3<f32>(0.19, 0.58, 0.97);
const DAY_HORIZON_COLOR = vec3<f32>(0.32, 0.69, 1.0);
const DAY_HORIZON_SQUISH_FACTOR: f32 = 2.9;
const NIGHT_ZENITH_COLOR = vec3<f32>(0.0001, 0.002, 0.003);
const NIGHT_HORIZON_COLOR = vec3<f32>(0.002, 0.004, 0.006);
const NIGHT_HORIZON_SQUISH_FACTOR: f32 = 1.7;

const TWILIGHT_OFFSET: f32 = 0.25;

const SUN_COLOR = vec3<f32>(1.0, 0.8, 0.2);
const SUN_SIZE_INV: f32 = 5000.0;
const SUN_SOFTNESS_INV: f32 = 2.9;

const SUN_GLARE_SIZE_INV: f32 = 80.0;
const SUN_GLARE_DECAY_INV: f32 = 1.15; // must be >= 1
const SUN_GLARE_MULTIPLIER: f32 = 0.7;

const SUNSET_COLOR = vec3<f32>(1.0, 0.65, 0.3);
const SUNSET_BRIGHTNESS: f32 = 0.13;
const SUNSET_HORIZON_CURVE_INV: f32 = 15.0;
const SUNSET_CURVE_INV: f32 = 0.31;
const SUNSET_TIME_RANGE_INV: f32 = 2.9;
const SUNSET_TIME_RANGE_OFFSET: f32 = 0.17;

const STARS_COLOR = vec3<f32>(1.0, 1.0, 1.0);
const STARS_SCALE: f32 = 230.0;
const STARS_DENSITY_INV: f32 = 0.995;
const STARS_TIME_RANGE_FACTOR: f32 = 80.0;

@group(#{MATERIAL_BIND_GROUP}) @binding(0) var<uniform> sun_position: vec3<f32>;
@group(#{MATERIAL_BIND_GROUP}) @binding(1) var<uniform> sky_rotation_matrix: mat3x3<f32>;

struct Vertex {
    @builtin(instance_index) instance_index: u32,
    @location(0) position: vec3<f32>,
}

struct CustomVertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(1) lposition: vec3<f32>,
    @location(3) @interpolate(flat) instance_index: u32,
}

@vertex
fn vertex(in: Vertex) -> CustomVertexOutput {
    // Boilerplate.

    var out: CustomVertexOutput;

    let world_mat = mesh_functions::get_world_from_local(in.instance_index);
    let world_position = mesh_functions::mesh_position_local_to_world(world_mat, vec4<f32>(in.position, 1.0));

    out.position = position_world_to_clip(world_position.xyz);

    // Added this to pass the local 3d position.

    out.lposition = in.position;

    // Boilerplate.

    out.instance_index = in.instance_index;

    // Return value.

    return out;
}

@fragment
fn fragment(vertout: CustomVertexOutput) -> @location(0) vec4<f32> {
    var color = vec3<f32>(0.0, 0.0, 0.0);

    let sun_pos = normalize(sun_position);
    let dir: vec3<f32> = normalize(vertout.lposition.xyz);

    let day_amount = smoothstep(0.0, 1.0, sun_pos.y + TWILIGHT_OFFSET);

    let day_color = mix(DAY_HORIZON_COLOR, DAY_ZENITH_COLOR, smoothstep_skew_left(0.0, 1.0, DAY_HORIZON_SQUISH_FACTOR, dir.y));
    let night_color = mix(NIGHT_HORIZON_COLOR, NIGHT_ZENITH_COLOR, smoothstep_skew_left(0.0, 1.0, NIGHT_HORIZON_SQUISH_FACTOR, dir.y));
    color += mix(night_color, day_color, day_amount);

    // Sun
    color += SUN_COLOR * pow(2.0 * smoothstep_skew_right(0.0, 1.0, SUN_SIZE_INV, dot(dir, sun_pos)), SUN_SOFTNESS_INV);
    let sun_glare_color_lerp = clamp(sun_pos.y, 0.0, 1.0);
    let sun_glare_color = SUN_COLOR * sun_glare_color_lerp + SUNSET_COLOR * (1.0 - sun_glare_color_lerp);
    let sun_glare_amount = SUN_GLARE_MULTIPLIER * arc_step_up(SUN_GLARE_DECAY_INV, clamp(sun_pos.y, 0.0, 1.0));
    color += sun_glare_color * sun_glare_amount * pow(clamp(dot(dir, sun_pos), 0.0, 1.0), SUN_GLARE_SIZE_INV);

    // Sunset
    let sunset_horizon_glow = bell(SUNSET_HORIZON_CURVE_INV * dir.y);
    let sunset_side = -1.0 / ((dot(dir, sun_pos) * 0.5 + 0.5) - 1.0 - SUNSET_CURVE_INV);
    let sunset_time_multiplier = (1.0 - smoothstep_skew_left(0.0, 1.0, SUNSET_TIME_RANGE_INV, abs(sun_pos.y - SUNSET_TIME_RANGE_OFFSET)));
    color += SUNSET_BRIGHTNESS * SUNSET_COLOR * (sunset_horizon_glow * sunset_side * sunset_time_multiplier);

    // Stars
    color += STARS_COLOR * local_star_value(sky_rotation_matrix * dir) * clamp(pow(1.0 - day_amount, STARS_TIME_RANGE_FACTOR), 0.0, 1.0);

    color += reduce_banding(vertout.position.xy);
    return vec4<f32>(color, 1.0);
}

fn local_star_value(dir: vec3<f32>) -> f32 {
    let grid = dir * STARS_SCALE;

    let h = hash3(floor(grid));
    if h > STARS_DENSITY_INV {
        let sdist = length(fract(grid) - 0.5); // Distance from the center of the star.
        return clamp(-pow(abs(sdist * 1.5) - 1.0, 5.0), 0.0, 1.0);
    } else {
        return 0.0;
    }
}
