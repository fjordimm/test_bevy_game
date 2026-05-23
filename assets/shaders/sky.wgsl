#import bevy_pbr::forward_io::VertexOutput
#import "shaders/util.wgsl"::reduce_banding;
#import "shaders/util.wgsl"::smoothstep_skew_left;
#import "shaders/util.wgsl"::smoothstep_skew_right;

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

@group(#{MATERIAL_BIND_GROUP}) @binding(0) var<uniform> sun_position: vec3<f32>;

@fragment
fn fragment(vert_out: VertexOutput) -> @location(0) vec4<f32> {
    var color = vec3<f32>(0.0, 0.0, 0.0);

    let sun_pos = normalize(sun_position);
    let pos: vec3<f32> = normalize(vert_out.world_position.xyz);

    let day_color = mix(DAY_HORIZON_COLOR, DAY_ZENITH_COLOR, smoothstep_skew_left(0.0, 1.0, DAY_HORIZON_SQUISH_FACTOR, pos.y));
    let night_color = mix(NIGHT_HORIZON_COLOR, NIGHT_ZENITH_COLOR, smoothstep_skew_left(0.0, 1.0, NIGHT_HORIZON_SQUISH_FACTOR, pos.y));
    color += mix(night_color, day_color, clamp(sun_pos.y + TWIGHTLIGHT_OFFSET, 0.0, 1.0));

    // Sun
    color += SUN_COLOR * pow(2.0 * smoothstep_skew_right(0.0, 1.0, INV_SUN_SIZE, dot(pos, sun_pos)), INV_SUN_SOFTNESS);

    // Sunset
    color += SUNSET_BRIGHTNESS * SUNSET_COLOR
        * (1.0 - pow(max(0.0, -sun_pos.y), 0.03))
        * (3.0 * pow(50.0, dot(pos, sun_pos) - 1.2) * 0.01 / (0.01 + pow(pos.y, 2.0))
           + 0.3 * pow(max(0.0, dot(pos, sun_pos)), 25.0));

    color += reduce_banding(vert_out.position.xy);
    return vec4<f32>(color, 1.0);
}