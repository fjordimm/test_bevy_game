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

    let sky_pos = normalize(in.world_position.xyz);

    let zenith_color = mix(night_zenith_color, day_zenith_color, max(0.0, sun_pos.y));
    let horizon_color = mix(night_horizon_color, day_horizon_color, max(0.0, sun_pos.y));
    var color = mix(horizon_color, zenith_color, max(0.0, sky_pos.y));

    let sun_amount = pow(max(dot(sky_pos, sun_pos), 0.0), 512.0);

    color += vec3<f32>(1.0, 0.9, 0.6) * sun_amount;

    return vec4<f32>(color, 1.0);

    // return vec4<f32>(0.0, 0.0, 0.0, 1.0);
}
