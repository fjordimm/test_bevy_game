#import bevy_core_pipeline::fullscreen_vertex_shader::FullscreenVertexOutput
#import "shaders/helpers/util_noise.wgsl"::simplex_noise_2d
#import "shaders/global_render_data.wgsl"::GlobalRenderData
#import "shaders/helpers/sky.wgsl"::sky_without_sun_and_stars

const UNDERWATER_OVERLAY_OPACITY: f32 = 0.2;

@group(0) @binding(0) var screen_texture: texture_2d<f32>;
@group(0) @binding(1) var texture_sampler: sampler;
struct PostProcessorSettings {
    _unused: vec4<f32>,
}
@group(0) @binding(2) var<uniform> settings: PostProcessorSettings;
@group(0) @binding(3) var<storage, read> global_render_data: GlobalRenderData;

@fragment
fn fragment(in: FullscreenVertexOutput) -> @location(0) vec4<f32> {
    var out = textureSample(screen_texture, texture_sampler, in.uv).rgb;

    if bool(global_render_data.cam_is_underwater) {
        let water_color = sky_without_sun_and_stars(global_render_data, vec3(0.0, -1.0, 0.0), vec2(0.0));

        out = (1.0 - UNDERWATER_OVERLAY_OPACITY) * out + (UNDERWATER_OVERLAY_OPACITY) * water_color;
    }

    return vec4(out, 1.0);
}
