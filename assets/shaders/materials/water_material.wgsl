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
#import "shaders/global_render_data.wgsl"::GlobalRenderData;
#import "shaders/helpers/sky.wgsl"::{sky_without_sun_and_stars, sky};
#import "shaders/helpers/sky.wgsl"::FOG_START;
#import "shaders/helpers/sky.wgsl"::FOG_END;
#import "shaders/helpers/util_noise.wgsl"::simplex_noise_3d;
#import "shaders/helpers/water.wgsl"::{
    WATER_WAVES_SCALE,
    WATER_WAVES_TIME_SCALE,
    WATER_WAVES_HEIGHT,
}

@group(#{MATERIAL_BIND_GROUP}) @binding(100) var<storage, read> global_render_data: GlobalRenderData;
@group(#{MATERIAL_BIND_GROUP}) @binding(101) var<uniform> texturing_scale: f32;

struct Vertex {
    @builtin(instance_index) instance_index: u32,
    @location(0) position: vec3<f32>,
}

struct CustomVertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) world_position: vec4<f32>,
    @location(1) cam_relative_pos: vec3<f32>,
    @location(2) fog_amount: f32,
    @location(3) @interpolate(flat) instance_index: u32,
}

fn to_pbr_vertex_output(og: CustomVertexOutput) -> VertexOutput {
    var ret: VertexOutput;
    ret.position = og.position;
    ret.world_position = og.world_position;
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

    var world_pos = in.world_position.xyz;

    // Water stuff.

    world_pos.y += WATER_WAVES_HEIGHT * simplex_noise_3d(vec3(WATER_WAVES_SCALE * world_pos.x, WATER_WAVES_SCALE * world_pos.z, WATER_WAVES_TIME_SCALE * global_render_data.time_elapsed));

    // Boilerplate.

    pbr_vertex_output.world_normal = normalize(cross(dpdy(world_pos), dpdx(world_pos)));
    
    var pbr_input = pbr_input_from_standard_material(pbr_vertex_output, is_front);

    // Could modify color here too. // TODOr

    // Boilerplate.

    pbr_input.material.base_color = alpha_discard(pbr_input.material, pbr_input.material.base_color);

    var out = vec4<f32>(0.0);

    // Water stuff.
    
    let reflected_cam_relative_pos = reflect(in.cam_relative_pos, pbr_vertex_output.world_normal);
    let sky_reflection = sky(global_render_data, reflected_cam_relative_pos, in.position.xy);
    out = vec4(sky_reflection, pbr_input.material.base_color.a);

    // Fog.

    let fog_color = sky_without_sun_and_stars(global_render_data, in.cam_relative_pos, in.position.xy);
    out = vec4((1.0 - in.fog_amount) * out.rgb + (in.fog_amount) * fog_color, out.a);

    // Return value.

    return out;
}
