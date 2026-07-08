#import bevy_pbr::forward_io::VertexOutput

@group(1) @binding(0)
var<uniform> base_color: vec4<f32>;

@fragment
fn fragment(in: VertexOutput) -> @location(0) vec4<f32> {
    let world_normal = normalize(in.world_normal);
    let light_dir = normalize(vec3<f32>(-0.4, 0.8, 0.6));
    let diffuse = max(dot(world_normal, light_dir), 0.0);
    let ambient = 0.2;

    let lit_color = base_color.rgb * (ambient + diffuse);
    return vec4<f32>(lit_color, base_color.a);
}
