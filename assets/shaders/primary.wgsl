#import bevy_pbr::{
    pbr_fragment::pbr_input_from_standard_material,
    pbr_functions::apply_pbr_lighting,
    mesh_functions::get_world_from_local,
    mesh_functions::mesh_position_local_to_clip,
}

@group(#{MATERIAL_BIND_GROUP}) @binding(100) var<uniform> edge_color: vec3<f32>;

struct Vertex {
    @builtin(instance_index) instance_index: u32,
    @location(0) position: vec3<f32>,
    @location(1) normal: vec3<f32>,
    @location(2) uv: vec2<f32>,
    @location(3) test1: vec3<f32>,
}

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
}

@vertex
fn vertex(in: Vertex) -> VertexOutput {
    var out: VertexOutput;
    out.clip_position = mesh_position_local_to_clip(
        get_world_from_local(in.instance_index),
        vec4<f32>(in.position, 1.0),
    );

    return out;
}

@fragment
fn fragment(
    in: VertexOutput,
    @builtin(front_facing) is_front: bool,
) -> @location(0) vec4<f32> {
    var pbr_input = pbr_input_from_standard_material(in, is_front);

    return vec4<f32>(0.0, 0.0, 1.0, 1.0);
}

// @fragment
// fn fragment(in: VertexOutput) -> @location(0) vec4<f32> {

//     // Do the built-in PBR model lighting, giving it `base_color`.

//     var pbr_input = pbr_input_from_vertex_output(in, true, false);

//     pbr_input.material.base_color *= base_color;

//     pbr_input.material.perceptual_roughness = 1.0;
//     pbr_input.material.metallic = 0.0;
//     pbr_input.material.reflectance = vec3<f32>(0.0);
//     pbr_input.material.diffuse_transmission = 0.0;
//     pbr_input.material.specular_transmission = 0.0;
//     pbr_input.material.thickness = 0.0;
//     pbr_input.material.ior = 1.5;
//     pbr_input.material.attenuation_distance = 1.0;

//     var color = apply_pbr_lighting(pbr_input).rgb;

//     // Vignette thing near the edges.

//     // TODO

//     // Final return value.

//     return vec4<f32>(color, 1.0);
// }
