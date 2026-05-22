#import bevy_pbr::forward_io::VertexOutput
#import bevy_pbr::mesh_view_bindings
#import bevy_pbr::mesh_functions

@group(#{MATERIAL_BIND_GROUP}) @binding(0) var<uniform> day_zenith_color: vec3<f32>;
@group(#{MATERIAL_BIND_GROUP}) @binding(1) var<uniform> day_horizon_color: vec3<f32>;
@group(#{MATERIAL_BIND_GROUP}) @binding(2) var<uniform> night_zenith_color: vec3<f32>;
@group(#{MATERIAL_BIND_GROUP}) @binding(3) var<uniform> night_horizon_color: vec3<f32>;
@group(#{MATERIAL_BIND_GROUP}) @binding(4) var<uniform> sun_position: vec3<f32>;

@fragment
fn fragment(in: VertexOutput) -> @location(0) vec4<f32> {
    let sun_pos = normalize(sun_position);

    let pos: vec3<f32> = normalize(in.world_position.xyz);
    let clamped_pos_y: f32 = min(1.0, max(0.0, pos.y));

    let twilight_offset = 0.05;
    let zenith_color = mix(night_zenith_color, day_zenith_color, min(1.0, max(0.0, sun_pos.y + twilight_offset)));
    let horizon_color = mix(night_horizon_color, day_horizon_color, min(1.0, max(0.0, sun_pos.y + twilight_offset)));
    var color: vec3<f32> = mix(horizon_color, zenith_color, pow(smoothstep(clamped_pos_y), 0.5));

    // the sun
    let sun_color = vec3<f32>(1.0, 0.9, 0.6);
    color += sun_color * pow(max(0.0, dot(pos, sun_pos)), 3000.0);

    // sunset
    let sunset_color = vec3<f32>(1.0, 0.75, 0.3);
    color += 0.06 * sunset_color
        * (1.0 - pow(max(0.0, -sun_pos.y), 0.1))
        * (3.0 * pow(50.0, dot(pos, sun_pos) - 1.2) * 0.01 / (0.01 + pow(pos.y, 2.0))
           + 0.3 * pow(max(0.0, dot(pos, sun_pos)), 25.0));

    color += (1.0 / 255.0) * gradient_noise(in.position.xy) - (0.5 / 255.0); // Fights banding
    return vec4<f32>(color, 1.0);
}

fn smoothstep(x: f32) -> f32 {
    return 6.0 * pow(x, 5.0) - 15.0 * pow(x, 4.0) + 10.0 * pow(x, 3.0);
}

/* Gradient noise from Jorge Jimenez's presentation: */
/* http://www.iryoku.com/next-generation-post-processing-in-call-of-duty-advanced-warfare */
fn gradient_noise(uv: vec2<f32>) -> f32 {
    return fract(52.9829189 * fract(dot(uv, vec2(0.06711056, 0.00583715))));
}