use bevy::{
    core_pipeline::{Core3d, Core3dSystems, FullscreenShader},
    prelude::*,
    render::{
        RenderApp, RenderStartup,
        extract_component::{ExtractComponent, ExtractComponentPlugin, UniformComponentPlugin},
        render_asset::RenderAssets,
        render_resource::{
            BindGroup, BindGroupEntries, BindGroupLayoutDescriptor, BindGroupLayoutEntries,
            CachedRenderPipelineId, ColorTargetState, ColorWrites, FragmentState, Operations,
            PipelineCache, RenderPassColorAttachment, RenderPassDescriptor,
            RenderPipelineDescriptor, Sampler, SamplerBindingType, SamplerDescriptor, ShaderStages,
            ShaderType, TextureFormat, TextureSampleType, TextureViewId,
            binding_types::{sampler, storage_buffer_read_only, texture_2d, uniform_buffer},
        },
        renderer::{RenderContext, RenderDevice, ViewQuery},
        storage::GpuShaderBuffer,
        uniform::{ComponentUniforms, DynamicUniformIndex},
        view::ViewTarget,
    },
};

use crate::game::{
    core::states::OverallState,
    graphics::global_render_data::resources::{GlobalRenderData, GlobalRenderDataHandle},
    playing_state::{sets::OnEnterPlaying, tags::PrimaryCamera},
    util::{alrms, alrrs},
};

pub struct PostProcessorPlugin;

impl Plugin for PostProcessorPlugin {
    fn build(&self, app: &mut App) {
        #[rustfmt::skip]
        app
            .add_plugins((
                ExtractComponentPlugin::<PostProcessorSettings>::default(),
                UniformComponentPlugin::<PostProcessorSettings>::default(),
            ))
            .add_systems(OnEnter(OverallState::Playing),
                add_to_camera
                    .in_set(OnEnterPlaying::General)
            )
        ;

        if let Some(render_app) = alrms!(app.get_sub_app_mut(RenderApp)) {
            #[rustfmt::skip]
            render_app
                .add_systems(RenderStartup, init_post_processor_pipeline)
                .add_systems(Core3d,
                    post_processor_system
                        .in_set(Core3dSystems::PostProcess)
                )
            ;
        }
    }
}

const SHADER_ASSET_PATH: &str = "shaders/materials/post_processor.wgsl";

#[derive(Component, Default, Clone, Copy, ExtractComponent, ShaderType)]
struct PostProcessorSettings {
    _unused: Vec4,
}

#[derive(Resource)]
struct PostProcessorPipeline {
    layout: BindGroupLayoutDescriptor,
    sampler: Sampler,
    pipeline_id: CachedRenderPipelineId,
}

#[derive(Default)]
struct PostProcessorBindGroupCache {
    cached: Option<(TextureViewId, BindGroup)>,
}

fn init_post_processor_pipeline(
    mut commands: Commands,
    render_device: Res<RenderDevice>,
    asset_server: Res<AssetServer>,
    fullscreen_shader: Res<FullscreenShader>,
    pipeline_cache: Res<PipelineCache>,
) {
    let layout = BindGroupLayoutDescriptor::new(
        "post_processor_bind_group_layout",
        &BindGroupLayoutEntries::sequential(
            ShaderStages::FRAGMENT,
            (
                texture_2d(TextureSampleType::Float { filterable: true }),
                sampler(SamplerBindingType::Filtering),
                uniform_buffer::<PostProcessorSettings>(true),
                storage_buffer_read_only::<GlobalRenderData>(false),
            ),
        ),
    );
    let sampler = render_device.create_sampler(&SamplerDescriptor::default());

    let shader = asset_server.load(SHADER_ASSET_PATH);
    let vertex_state = fullscreen_shader.to_vertex_state();
    let pipeline_id = pipeline_cache.queue_render_pipeline(RenderPipelineDescriptor {
        label: Some("post_processor_pipeline".into()),
        layout: vec![layout.clone()],
        vertex: vertex_state,
        fragment: Some(FragmentState {
            shader,
            targets: vec![Some(ColorTargetState {
                format: TextureFormat::Rgba8UnormSrgb,
                blend: None,
                write_mask: ColorWrites::ALL,
            })],
            ..default()
        }),
        ..default()
    });
    commands.insert_resource(PostProcessorPipeline {
        layout,
        sampler,
        pipeline_id,
    });
}

fn post_processor_system(
    view: ViewQuery<(
        &ViewTarget,
        &PostProcessorSettings,
        &DynamicUniformIndex<PostProcessorSettings>,
    )>,
    post_processor_pipeline: Option<Res<PostProcessorPipeline>>,
    pipeline_cache: Res<PipelineCache>,
    settings_uniforms: Res<ComponentUniforms<PostProcessorSettings>>,
    mut cache: Local<PostProcessorBindGroupCache>,
    mut ctx: RenderContext,
    global_render_data_handle: Res<GlobalRenderDataHandle>,
    shader_buffers: Res<RenderAssets<GpuShaderBuffer>>,
) {
    let post_processor_pipeline = alrrs!(post_processor_pipeline);

    let (view_target, _post_processor_settings, settings_index) = view.into_inner();

    let pipeline = alrrs!(pipeline_cache.get_render_pipeline(post_processor_pipeline.pipeline_id));

    let settings_binding = alrrs!(settings_uniforms.uniforms().binding());

    let global_render_data =
        alrrs!(shader_buffers.get(global_render_data_handle.get_handle().id()));

    let post_processor = view_target.post_process_write();

    let bind_group = match &mut cache.cached {
        Some((texture_id, bind_group)) if post_processor.source.id() == *texture_id => bind_group,
        cached => {
            let bind_group = ctx.render_device().create_bind_group(
                "post_processor_bind_group",
                &pipeline_cache.get_bind_group_layout(&post_processor_pipeline.layout),
                &BindGroupEntries::sequential((
                    post_processor.source,
                    &post_processor_pipeline.sampler,
                    settings_binding.clone(),
                    global_render_data.buffer.as_entire_binding(),
                )),
            );

            let (_, bind_group) = cached.insert((post_processor.source.id(), bind_group));
            bind_group
        }
    };

    let mut render_pass = ctx
        .command_encoder()
        .begin_render_pass(&RenderPassDescriptor {
            label: Some("post_processor_pass"),
            color_attachments: &[Some(RenderPassColorAttachment {
                view: post_processor.destination,
                depth_slice: None,
                resolve_target: None,
                ops: Operations::default(),
            })],
            depth_stencil_attachment: None,
            timestamp_writes: None,
            occlusion_query_set: None,
            multiview_mask: None,
        });

    render_pass.set_pipeline(pipeline);
    render_pass.set_bind_group(0, bind_group, &[settings_index.index()]);
    render_pass.draw(0..3, 0..1);
}

fn add_to_camera(mut commands: Commands, camera_q: Query<Entity, Added<PrimaryCamera>>) {
    camera_q.iter().for_each(|camera| {
        commands
            .entity(camera)
            .insert(PostProcessorSettings { _unused: default() });
    });
}
