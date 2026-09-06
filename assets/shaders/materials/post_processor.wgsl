#import bevy_core_pipeline::fullscreen_vertex_shader::FullscreenVertexOutput
#import "shaders/helpers/util_noise.wgsl"::simplex_noise_2d

const BLUR_RADIUS: f32 = 0.002;

@group(0) @binding(0) var screen_texture: texture_2d<f32>;
@group(0) @binding(1) var texture_sampler: sampler;
struct PostProcessorSettings {
    _unused: vec4<f32>,
}
@group(0) @binding(2) var<uniform> settings: PostProcessorSettings;

@fragment
fn fragment(in: FullscreenVertexOutput) -> @location(0) vec4<f32> {
    if in.uv.x < 0.1 {
        return vec4(0.0, 1.0, 0.0, 1.0);
    }

    return textureSample(screen_texture, texture_sampler, in.uv);
}
