#import bevy_pbr::{
    forward_io::VertexOutput,
    mesh_functions,
    view_transformations::position_world_to_clip,
    pbr_fragment::pbr_input_from_standard_material,
    pbr_functions::{
        alpha_discard,
        apply_pbr_lighting,
        main_pass_post_lighting_processing
    },
    mesh_view_bindings as view_bindings,
    lighting,
}
#import "shaders/util_noise.wgsl"::simplex_noise_3d;

@group(#{MATERIAL_BIND_GROUP}) @binding(100) var<uniform> texturing_scale: f32;

struct Vertex {
    @builtin(instance_index) instance_index: u32,
    @location(0) position: vec3<f32>,
    @location(1) color: vec4<f32>,
}

struct CustomVertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) world_position: vec4<f32>,
    @location(1) lposition: vec3<f32>,
    @location(2) color: vec4<f32>,
    @location(3) @interpolate(flat) instance_index: u32,
}

fn to_pbr_vertex_output(og: CustomVertexOutput) -> VertexOutput {
    var ret: VertexOutput;
    ret.position = og.position;
    ret.world_position = og.world_position;
    ret.color = og.color;
    ret.instance_index = og.instance_index;

    return ret;
}

@vertex
fn vertex(in: Vertex) -> CustomVertexOutput {
    // Boilerplate.

    var out: CustomVertexOutput;

    let world_mat = mesh_functions::get_world_from_local(in.instance_index);
    out.world_position = mesh_functions::mesh_position_local_to_world(world_mat, vec4<f32>(in.position, 1.0));

    out.position = position_world_to_clip(out.world_position.xyz);

    // Added this to pass the local 3d position.
    out.lposition = in.position;

    // Boilerplate.

    out.color = in.color;
    // out.visibility_range_dither = mesh_functions::get_visibility_range_dither_level(
    //     in.instance_index,
    //     world_mat[3]
    // );

    out.instance_index = in.instance_index;

    // Return value.

    return out;
}

@fragment
fn fragment(
    in: CustomVertexOutput,
    @builtin(front_facing) is_front: bool,
) -> @location(0) vec4<f32> {
    // Boilerplate.

    var pbr_vertex_output = to_pbr_vertex_output(in);

    // Compute normal (only allows for flat shading).

    let world_pos = in.world_position.xyz;
    pbr_vertex_output.world_normal = normalize(cross(dpdy(world_pos), dpdx(world_pos)));

    // Boilerplate.
    
    var pbr_input = pbr_input_from_standard_material(pbr_vertex_output, is_front);

    // Could modify color here too. // TODOr

    // Boilerplate.

    pbr_input.material.base_color = alpha_discard(pbr_input.material, pbr_input.material.base_color);

    var out = vec4<f32>(0.0);
    out = apply_pbr_lighting(pbr_input);

    // Could modify color here too. // TODOr

    // let n_directional_lights = view_bindings::lights.n_directional_lights;
    // for (var i: u32 = 0u; i < n_directional_lights; i = i + 1u) {
    // }

    // Boilerplate.

    out = main_pass_post_lighting_processing(pbr_input, out);

    // Return value.

    return out;
}
