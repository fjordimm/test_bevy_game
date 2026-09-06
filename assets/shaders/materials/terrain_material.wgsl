#import bevy_pbr::{
    forward_io::VertexOutput,
    mesh_functions,
    view_transformations::position_world_to_view,
    view_transformations::position_view_to_clip,
    pbr_fragment::pbr_input_from_standard_material,
    pbr_functions::{
        alpha_discard,
        apply_pbr_lighting,
        main_pass_post_lighting_processing
    },
    mesh_view_bindings as view_bindings,
    lighting,
}
#import "shaders/helpers/util_noise.wgsl"::simplex_noise_3d;
#import "shaders/global_render_data.wgsl"::GlobalRenderData;
#import "shaders/helpers/sky.wgsl"::sky_without_sun_and_stars;
#import "shaders/helpers/sky.wgsl"::FOG_START;
#import "shaders/helpers/sky.wgsl"::FOG_END;

@group(#{MATERIAL_BIND_GROUP}) @binding(100) var<storage, read> global_render_data: GlobalRenderData;
@group(#{MATERIAL_BIND_GROUP}) @binding(101) var<uniform> texturing_scale: f32;

struct Vertex {
    @builtin(instance_index) instance_index: u32,
    @location(0) position: vec3<f32>,
#ifdef FEATURE_TERRAIN_DEBUG_COLS
    @location(1) color: vec4<f32>,
#endif
}

struct CustomVertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) world_position: vec4<f32>,
    @location(1) cam_relative_pos: vec3<f32>,
    @location(2) fog_amount: f32,
#ifdef FEATURE_TERRAIN_DEBUG_COLS
    @location(3) color: vec4<f32>,
#endif
    @location(4) @interpolate(flat) instance_index: u32,
}

fn to_pbr_vertex_output(og: CustomVertexOutput) -> VertexOutput {
    var ret: VertexOutput;
    ret.position = og.position;
    ret.world_position = og.world_position;
#ifdef FEATURE_TERRAIN_DEBUG_COLS
    ret.color = og.color;
#endif
    ret.instance_index = og.instance_index;

    return ret;
}

@vertex
fn vertex(in: Vertex) -> CustomVertexOutput {
    // Boilerplate.

    var out: CustomVertexOutput;

    let world_mat = mesh_functions::get_world_from_local(in.instance_index);
    out.world_position = mesh_functions::mesh_position_local_to_world(world_mat, vec4<f32>(in.position, 1.0));

    let view_position = position_world_to_view(out.world_position.xyz);

    out.position = position_view_to_clip(view_position);

    // Added this to pass the camera-relative position.

    out.cam_relative_pos = out.world_position.xyz;
    out.cam_relative_pos -= view_bindings::view.world_position;

    // Fog.

    out.fog_amount = (-view_position.z - 0.5 - FOG_START) / (FOG_END - FOG_START);
    out.fog_amount = clamp(out.fog_amount, 0.0, 1.0);

    // Boilerplate.

#ifdef FEATURE_TERRAIN_DEBUG_COLS
    out.color = in.color;
#endif
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

#ifndef FEATURE_TERRAIN_DEBUG_COLS
    let color = vec4(pbr_vertex_output.world_normal.y, 0.0, 0.0, 1.0);

    pbr_input.material.base_color = color;
#endif

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

    // Fog.

    let fog_color = sky_without_sun_and_stars(global_render_data, in.cam_relative_pos, in.position.xy);
    out = vec4((1.0 - in.fog_amount) * out.rgb + (in.fog_amount) * fog_color, 1.0);

    // Return value.

    return out;
}
