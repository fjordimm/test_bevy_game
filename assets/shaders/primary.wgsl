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

// @group(#{MATERIAL_BIND_GROUP}) @binding(100) var<uniform> edge_color: vec4<f32>; // TODOr

struct Vertex {
    @builtin(instance_index) instance_index: u32,
    @location(0) position: vec3<f32>,
    @location(1) normal: vec3<f32>,
    @location(2) uv: vec2<f32>,
    @location(3) tangent: vec4<f32>,
}

struct CustomVertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) world_position: vec4<f32>,
    @location(1) world_normal: vec3<f32>,
    @location(2) uv: vec2<f32>,
    @location(3) tangent: vec4<f32>,
    @location(4) @interpolate(flat) instance_index: u32,
}

fn to_pbr_vertex_output(og: CustomVertexOutput) -> VertexOutput {
    var ret: VertexOutput;
    ret.position = og.position;
    ret.world_position = og.world_position;
    ret.world_normal = og.world_normal;
    ret.uv = og.uv;
    ret.world_tangent = og.tangent;
    ret.instance_index = og.instance_index;

    return ret;
}

@vertex
fn vertex(in: Vertex) -> CustomVertexOutput {
    // Boilerplate.

    var out: CustomVertexOutput;
    var world_from_local = mesh_functions::get_world_from_local(in.instance_index);
    out.world_normal = mesh_functions::mesh_normal_local_to_world(
        in.normal,
        in.instance_index
    );
    out.world_position = mesh_functions::mesh_position_local_to_world(world_from_local, vec4<f32>(in.position, 1.0));
    out.position = position_world_to_clip(out.world_position.xyz);
    out.uv = in.uv;
    out.tangent = in.tangent;
    // out.visibility_range_dither = mesh_functions::get_visibility_range_dither_level(
    //     in.instance_index,
    //     mesh_world_from_local[3]
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
    
    var pbr_input = pbr_input_from_standard_material(to_pbr_vertex_output(in), is_front);

    // Could modify color here too. // TODOr

    // let ambient_upwardness = in.world_normal

    // Boilerplate.

    pbr_input.material.base_color = alpha_discard(pbr_input.material, pbr_input.material.base_color);

    var out = vec4<f32>(0.0);
    out = apply_pbr_lighting(pbr_input);

    // Could modify color here too. // TODOr

    let n_directional_lights = view_bindings::lights.n_directional_lights;
    for (var i: u32 = 0u; i < n_directional_lights; i = i + 1u) {
        // let light = &view_bindings::lights.directional_lights[i];

        // let LdotN = dot((*light).direction_to_light, in.world_normal);

        // out += vec4<f32>(vec3<f32>(1.0) * LdotN, 1.0);
    }

    // Boilerplate.

    out = main_pass_post_lighting_processing(pbr_input, out);

    // Return value.

    return out;
}
