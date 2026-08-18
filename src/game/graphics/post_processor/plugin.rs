use bevy::{
    core_pipeline::{
        FullscreenShader,
        core_3d::graph::{Core3d, Node3d},
    },
    prelude::*,
    render::{
        RenderApp, RenderStartup,
        extract_component::{
            ComponentUniforms, DynamicUniformIndex, ExtractComponent, ExtractComponentPlugin,
            UniformComponentPlugin,
        },
        render_graph::{
            NodeRunError, RenderGraphContext, RenderGraphExt, RenderLabel, ViewNode, ViewNodeRunner,
        },
        render_resource::{
            BindGroupEntries, BindGroupLayoutDescriptor, BindGroupLayoutEntries,
            CachedRenderPipelineId, ColorTargetState, ColorWrites, FragmentState, Operations,
            PipelineCache, RenderPassColorAttachment, RenderPassDescriptor,
            RenderPipelineDescriptor, Sampler, SamplerBindingType, SamplerDescriptor, ShaderStages,
            ShaderType, TextureFormat, TextureSampleType,
            binding_types::{sampler, texture_2d, uniform_buffer},
        },
        renderer::{RenderContext, RenderDevice},
        view::ViewTarget,
    },
};
use bevy_ecs::query::QueryItem;

use crate::game::{playing_state::player::tags::CameraForPlayer, util::alrms};

pub struct PostProcessorPlugin;

impl Plugin for PostProcessorPlugin {
    fn build(&self, app: &mut App) {
        #[rustfmt::skip]
        app
            .add_plugins((
                ExtractComponentPlugin::<PostProcessorSettings>::default(),
                UniformComponentPlugin::<PostProcessorSettings>::default(),
            ))
            .add_systems(Update, add_to_camera) // TODO: optimize condition for this to run
        ;

        if let Some(render_app) = alrms!(app.get_sub_app_mut(RenderApp)) {
            #[rustfmt::skip]
            render_app
                .add_systems(RenderStartup, init_post_process_pipeline)
                .add_render_graph_node::<ViewNodeRunner<PostProcessorNode>>(Core3d,
                    PostProcessorLabel,
                )
                .add_render_graph_edges(Core3d, (
                    Node3d::Tonemapping,
                    PostProcessorLabel,
                    Node3d::EndMainPassPostProcessing,
                ))
            ;
        }
    }
}

const SHADER_ASSET_PATH: &str = "shaders/post_processor.wgsl";

#[derive(Debug, Hash, PartialEq, Eq, Clone, RenderLabel)]
struct PostProcessorLabel;

#[derive(Default)]
struct PostProcessorNode;

impl ViewNode for PostProcessorNode {
    type ViewQuery = (
        &'static ViewTarget,
        &'static PostProcessorSettings,
        &'static DynamicUniformIndex<PostProcessorSettings>,
    );

    fn run(
        &self,
        _graph: &mut RenderGraphContext,
        render_context: &mut RenderContext,
        (view_target, _post_process_settings, settings_index): QueryItem<Self::ViewQuery>,
        world: &World,
    ) -> Result<(), NodeRunError> {
        let post_process_pipeline = world.resource::<PostProcessorPipeline>();

        let pipeline_cache = world.resource::<PipelineCache>();

        let Some(pipeline) = pipeline_cache.get_render_pipeline(post_process_pipeline.pipeline_id)
        else {
            return Ok(());
        };

        let settings_uniforms = world.resource::<ComponentUniforms<PostProcessorSettings>>();
        let Some(settings_binding) = settings_uniforms.uniforms().binding() else {
            return Ok(());
        };

        let post_process = view_target.post_process_write();

        let bind_group = render_context.render_device().create_bind_group(
            "post_process_bind_group",
            &pipeline_cache.get_bind_group_layout(&post_process_pipeline.layout),
            &BindGroupEntries::sequential((
                post_process.source,
                &post_process_pipeline.sampler,
                settings_binding.clone(),
            )),
        );

        let mut render_pass = render_context.begin_tracked_render_pass(RenderPassDescriptor {
            label: Some("post_process_pass"),
            color_attachments: &[Some(RenderPassColorAttachment {
                view: post_process.destination,
                depth_slice: None,
                resolve_target: None,
                ops: Operations::default(),
            })],
            depth_stencil_attachment: None,
            timestamp_writes: None,
            occlusion_query_set: None,
        });

        render_pass.set_render_pipeline(pipeline);
        render_pass.set_bind_group(0, &bind_group, &[settings_index.index()]);
        render_pass.draw(0..3, 0..1);

        Ok(())
    }
}

#[derive(Resource)]
struct PostProcessorPipeline {
    layout: BindGroupLayoutDescriptor,
    sampler: Sampler,
    pipeline_id: CachedRenderPipelineId,
}

fn init_post_process_pipeline(
    mut commands: Commands,
    render_device: Res<RenderDevice>,
    asset_server: Res<AssetServer>,
    fullscreen_shader: Res<FullscreenShader>,
    pipeline_cache: Res<PipelineCache>,
) {
    let layout = BindGroupLayoutDescriptor::new(
        "post_process_bind_group_layout",
        &BindGroupLayoutEntries::sequential(
            ShaderStages::FRAGMENT,
            (
                texture_2d(TextureSampleType::Float { filterable: true }),
                sampler(SamplerBindingType::Filtering),
                uniform_buffer::<PostProcessorSettings>(true),
            ),
        ),
    );

    let sampler = render_device.create_sampler(&SamplerDescriptor::default());

    let shader = asset_server.load(SHADER_ASSET_PATH);
    let vertex_state = fullscreen_shader.to_vertex_state();
    let pipeline_id = pipeline_cache.queue_render_pipeline(RenderPipelineDescriptor {
        label: Some("post_process_pipeline".into()),
        layout: vec![layout.clone()],
        vertex: vertex_state,
        fragment: Some(FragmentState {
            shader,
            targets: vec![Some(ColorTargetState {
                format: TextureFormat::bevy_default(),
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

#[derive(Component, Default, Clone, Copy, ExtractComponent, ShaderType)]
struct PostProcessorSettings {
    _unused: Vec4,
}

fn add_to_camera(mut commands: Commands, camera_q: Query<Entity, Added<CameraForPlayer>>) {
    camera_q.iter().for_each(|camera| {
        commands
            .entity(camera)
            .insert(PostProcessorSettings { _unused: default() });
    });
}
