#import bevy_pbr::forward_io::VertexOutput
#import bevy_pbr::mesh_view_bindings as view_bindings

@group(#{MATERIAL_BIND_GROUP}) @binding(0) var<uniform> base_color: vec4<f32>;

@fragment
fn fragment(in: VertexOutput) -> @location(0) vec4<f32> {
    let world_normal = normalize(in.world_normal);
    var lighting = view_bindings::lights.ambient_color.rgb * base_color.rgb;

    let light_count = view_bindings::lights.n_directional_lights;
    for (var i: u32 = 0u; i < light_count; i = i + 1u) {
        let light = view_bindings::lights.directional_lights[i];
        let light_dir = normalize(-light.direction_to_light);
        let diffuse = max(dot(world_normal, light_dir), 0.0);
        lighting += base_color.rgb * diffuse * light.color.rgb;
    }

    return vec4<f32>(lighting, base_color.a);
}
