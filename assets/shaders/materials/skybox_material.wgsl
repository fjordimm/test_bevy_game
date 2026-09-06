#import bevy_pbr::{
    mesh_functions,
    view_transformations::position_world_to_clip,
}
#import "shaders/global_render_data.wgsl"::GlobalRenderData;
#import "shaders/helpers/sky.wgsl"::sky;

@group(#{MATERIAL_BIND_GROUP}) @binding(0) var<storage, read> global_render_data: GlobalRenderData;

struct Vertex {
    @builtin(instance_index) instance_index: u32,
    @location(0) position: vec3<f32>,
}

struct CustomVertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(1) cam_relative_pos: vec3<f32>,
    @location(2) @interpolate(flat) instance_index: u32,
}

@vertex
fn vertex(in: Vertex) -> CustomVertexOutput {
    // Boilerplate.

    var out: CustomVertexOutput;

    let world_mat = mesh_functions::get_world_from_local(in.instance_index);
    let world_position = mesh_functions::mesh_position_local_to_world(world_mat, vec4<f32>(in.position, 1.0));

    out.position = position_world_to_clip(world_position.xyz);

    // Added this to pass the local 3d position.

    out.cam_relative_pos = world_position.xyz;

    // Boilerplate.

    out.instance_index = in.instance_index;

    // Return value.

    return out;
}

@fragment
fn fragment(in: CustomVertexOutput) -> @location(0) vec4<f32> {
    return vec4(sky(global_render_data, in.cam_relative_pos, in.position.xy), 1.0);
}
