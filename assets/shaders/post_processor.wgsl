#import bevy_core_pipeline::fullscreen_vertex_shader::FullscreenVertexOutput
#import "shaders/util_noise.wgsl"::simplex_noise_2d

const BLUR_RADIUS: f32 = 0.002;

@group(0) @binding(0) var screen_texture: texture_2d<f32>;
@group(0) @binding(1) var texture_sampler: sampler;
struct PostProcessorSettings {
    _unused: vec4<f32>,
}
@group(0) @binding(2) var<uniform> settings: PostProcessorSettings;

@fragment
fn fragment(in: FullscreenVertexOutput) -> @location(0) vec4<f32> {
    // let m = textureSample(screen_texture, texture_sampler, in.uv);
    // let n = textureSample(screen_texture, texture_sampler, in.uv + vec2<f32>(0.0, -BLUR_RADIUS));
    // let s = textureSample(screen_texture, texture_sampler, in.uv + vec2<f32>(0.0, BLUR_RADIUS));
    // let e = textureSample(screen_texture, texture_sampler, in.uv + vec2<f32>(BLUR_RADIUS, 0.0));
    // let w = textureSample(screen_texture, texture_sampler, in.uv + vec2<f32>(-BLUR_RADIUS, 0.0));

    // return (m + n + s + e + w) * 0.2;




    // var col = textureSample(screen_texture, texture_sampler, in.uv);

    // let noise = 0.1 * simplex_noise_3d(in.position * 30.0);
    // col += vec4<f32>(noise, noise, noise, 1.0);

    // return col;




    // let scale = 30.0;
    // let intensity = 0.01;
    // let flow_x = simplex_noise_2d(scale * in.uv);
    // let flow_y = simplex_noise_2d(scale * (1.0 + in.uv));
    // var flow_vec = vec2<f32>(flow_x, flow_y);
    // flow_vec = intensity * normalize(flow_vec);

    // return textureSample(screen_texture, texture_sampler, in.uv + flow_vec);







    // let texture_dimensions = textureDimensions(screen_texture);
    // let dx = 1.0 / f32(texture_dimensions.x);
    // let dy = 1.0 / f32(texture_dimensions.y);
    
    // let m = textureSample(screen_texture, texture_sampler, in.uv);
    // let n = textureSample(screen_texture, texture_sampler, in.uv + vec2<f32>(0.0, -dy));
    // let s = textureSample(screen_texture, texture_sampler, in.uv + vec2<f32>(0.0, dy));
    // let e = textureSample(screen_texture, texture_sampler, in.uv + vec2<f32>(dx, 0.0));
    // let w = textureSample(screen_texture, texture_sampler, in.uv + vec2<f32>(-dx, 0.0));

    // let cdx = color_brightness(e) - color_brightness(m);
    // let cdy = color_brightness(s) - color_brightness(m);
    // var f = normalize(vec2<f32>(cdx, cdy));

    // var cdx = 0.0;
    // var cdy = 0.0;
    // {
    //     let N = 15;
    //     for (var i = 0; i < N; i++) {
    //         for (var j = 0; j < N; j++) {
    //             let pixel = textureSample(screen_texture, texture_sampler, in.uv);
    //             let right_pixel = textureSample(screen_texture, texture_sampler, in.uv + vec2<f32>(dx, 0.0));
    //             let below_pixel = textureSample(screen_texture, texture_sampler, in.uv + vec2<f32>(0.0, dy));

    //             cdx += color_brightness(right_pixel) - color_brightness(pixel);
    //             cdy += color_brightness(below_pixel) - color_brightness(pixel);
    //         }
    //     }
    //     cdx /= f32(N * N);
    //     cdy /= f32(N * N);
    // }

    // var f = vec2<f32>(cdx, cdy);
    // f = rotate_vec2_90(f);

    // let intensity = 0.1;
    // return textureSample(screen_texture, texture_sampler, in.uv + intensity * f);
    // // return vec4<f32>(f.x, 0.0, -f.x, 1.0);














    // TODO: optimize all of this

    let texture_dimensions = textureDimensions(screen_texture);
    let sdx = 1.0 / f32(texture_dimensions.x);
    let sdy = 1.0 / f32(texture_dimensions.y);

    let N = 5;

    var tl_mean = vec3<f32>(0.0, 0.0, 0.0);
    var tr_mean = vec3<f32>(0.0, 0.0, 0.0);
    var bl_mean = vec3<f32>(0.0, 0.0, 0.0);
    var br_mean = vec3<f32>(0.0, 0.0, 0.0);
    for (var i = 0; i < N; i++) {
        for (var j = 0; j < N; j++) {
            // Top left
            {
                let pixel_coords = in.uv + vec2<f32>(-f32(i) * sdx, -f32(j) * sdy);
                let pixel = textureSample(screen_texture, texture_sampler, pixel_coords);

                tl_mean += pixel.rgb;
            }
            // Top right
            {
                let pixel_coords = in.uv + vec2<f32>(f32(i) * sdx, -f32(j) * sdy);
                let pixel = textureSample(screen_texture, texture_sampler, pixel_coords);

                tr_mean += pixel.rgb;
            }
            // Bottom left
            {
                let pixel_coords = in.uv + vec2<f32>(-f32(i) * sdx, f32(j) * sdy);
                let pixel = textureSample(screen_texture, texture_sampler, pixel_coords);

                bl_mean += pixel.rgb;
            }
            // Bottom right
            {
                let pixel_coords = in.uv + vec2<f32>(f32(i) * sdx, f32(j) * sdy);
                let pixel = textureSample(screen_texture, texture_sampler, pixel_coords);

                br_mean += pixel.rgb;
            }
        }
    }
    tl_mean /= f32(N * N);
    tr_mean /= f32(N * N);
    bl_mean /= f32(N * N);
    br_mean /= f32(N * N);

    var tl_variance = 0.0;
    var tr_variance = 0.0;
    var bl_variance = 0.0;
    var br_variance = 0.0;
    for (var i = 0; i < N; i++) {
        for (var j = 0; j < N; j++) {
            // Top left
            {
                let pixel_coords = in.uv + vec2<f32>(-f32(i) * sdx, -f32(j) * sdy);
                let pixel = textureSample(screen_texture, texture_sampler, pixel_coords);

                tl_variance += dist2(pixel.rgb, tl_mean);
            }
            // Top right
            {
                let pixel_coords = in.uv + vec2<f32>(f32(i) * sdx, -f32(j) * sdy);
                let pixel = textureSample(screen_texture, texture_sampler, pixel_coords);

                tr_variance += dist2(pixel.rgb, tl_mean);
            }
            // Bottom left
            {
                let pixel_coords = in.uv + vec2<f32>(-f32(i) * sdx, f32(j) * sdy);
                let pixel = textureSample(screen_texture, texture_sampler, pixel_coords);

                bl_variance += dist2(pixel.rgb, tl_mean);
            }
            // Bottom right
            {
                let pixel_coords = in.uv + vec2<f32>(f32(i) * sdx, f32(j) * sdy);
                let pixel = textureSample(screen_texture, texture_sampler, pixel_coords);

                br_variance += dist2(pixel.rgb, tl_mean);
            }
        }
    }

    if (tl_variance < tr_variance && tl_variance < bl_variance && tl_variance < br_variance) {
        return vec4<f32>(tl_mean, 1.0);
    } else if (tr_variance < bl_variance && tr_variance < br_variance) {
        return vec4<f32>(tr_mean, 1.0);
    } else if (bl_variance < br_variance) {
        return vec4<f32>(bl_mean, 1.0);
    } else {
        return vec4<f32>(br_mean, 1.0);
    }
    








    // return textureSample(screen_texture, texture_sampler, in.uv);
}

fn dist2(c1: vec3<f32>, c2: vec3<f32>) -> f32 {
    let r = c1.r - c2.r;
    let g = c1.g - c2.g;
    let b = c1.b - c2.b;
    return r * r + g * g + b * b;
}

fn color_brightness(c: vec4<f32>) -> f32 {
    return (c.r + c.g + c.b) / 3.0;
}

fn rotate_vec2_90(v: vec2<f32>) -> vec2<f32> {
    return vec2<f32>(v.y, -v.x);
}