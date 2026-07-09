#import bevy_pbr::{
    forward_io::{UncompressedVertex, VertexOutput, decompress_vertex},
    pbr_fragment::pbr_input_from_standard_material,
    pbr_functions::apply_pbr_lighting,
    mesh::morph_vertex,
    mesh_functions,
    skinning,
    view_transformations::position_world_to_clip,
}

@group(#{MATERIAL_BIND_GROUP}) @binding(100) var<uniform> edge_color: vec3<f32>;

struct Vertex {
    @builtin(instance_index) instance_index: u32,
    @location(0) position: vec3<f32>,
    @location(1) normal: vec3<f32>,
    @location(2) uv: vec2<f32>,
    @location(3) test1: vec3<f32>,
}

@vertex
fn vertex(in: Vertex) -> VertexOutput {
    // Most code here was copied from https://github.com/bevyengine/bevy/blob/main/crates/bevy_pbr/src/render/mesh.wgsl

    var out: VertexOutput;
    let uncompressed_vertex_no_morph = decompress_vertex(in, in.instance_index);
#ifdef MORPH_TARGETS
    var vertex = morph_vertex(uncompressed_vertex_no_morph, in.instance_index);
#else
    var vertex = uncompressed_vertex_no_morph;
#endif

    let mesh_world_from_local = mesh_functions::get_world_from_local(in.instance_index);

#ifdef SKINNED
    var world_from_local = skinning::skin_model(
        vertex.joint_indices,
        vertex.joint_weights,
        in.instance_index
    );
#else
    var world_from_local = mesh_world_from_local;
#endif

#ifdef VERTEX_NORMALS
#ifdef SKINNED
    out.world_normal = skinning::skin_normals(world_from_local, vertex.normal);
#else
    out.world_normal = mesh_functions::mesh_normal_local_to_world(
        vertex.normal,
        in.instance_index
    );
#endif
#endif

#ifdef VERTEX_POSITIONS
    out.world_position = mesh_functions::mesh_position_local_to_world(world_from_local, vec4<f32>(vertex.position, 1.0));
    out.position = position_world_to_clip(out.world_position.xyz);
#endif

#ifdef VERTEX_UVS_A
    out.uv = vertex.uv;
#endif
#ifdef VERTEX_UVS_B
    out.uv_b = vertex.uv_b;
#endif

#ifdef VERTEX_TANGENTS
    out.world_tangent = mesh_functions::mesh_tangent_local_to_world(
        world_from_local,
        vertex.tangent,
        in.instance_index
    );
#endif

#ifdef VERTEX_COLORS
    out.color = vertex.color;
#endif

#ifdef VERTEX_OUTPUT_INSTANCE_INDEX
    out.instance_index = in.instance_index;
#endif

#ifdef VISIBILITY_RANGE_DITHER
    out.visibility_range_dither = mesh_functions::get_visibility_range_dither_level(
        in.instance_index, mesh_world_from_local[3]);
#endif

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
