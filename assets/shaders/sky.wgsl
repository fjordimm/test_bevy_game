#import bevy_pbr::forward_io::VertexOutput
#import bevy_pbr::mesh_view_bindings
#import bevy_pbr::mesh_functions

@group(#{MATERIAL_BIND_GROUP}) @binding(0) var<uniform> sun_direction: vec3<f32>;

@fragment
fn fragment(in: VertexOutput) -> @location(0) vec4<f32> {
    let dir = normalize(in.world_position.xyz);

    let horizon = vec3<f32>(0.8, 0.9, 1.0);
    let zenith = vec3<f32>(0.1, 0.3, 0.8);

    var t = max(dir.y * 0.5 + 0.5, 0.0);

    var color = mix(horizon, zenith, t);

    let sun_amount = pow(max(dot(dir, normalize(sun_direction)), 0.0), 512.0);

    color += vec3<f32>(1.0, 0.9, 0.6) * sun_amount;

    return vec4<f32>(color, 1.0);

    // return vec4<f32>(0.0, 0.0, 0.0, 1.0);
}
