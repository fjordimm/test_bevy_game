use bevy::{
    camera::{RenderTarget, visibility::RenderLayers},
    prelude::*,
    render::render_resource::{Extent3d, TextureFormat},
    window::WindowResized,
};
use bevy_ecs::entity::Entities;

use crate::game::{
    core::{resources::KeyBindings, states::OverallState},
    graphics::{
        global_render_data::resources::GlobalRenderDataHandle,
        primary_material::plugin::PrimaryMaterial, terrain_material::plugin::TerrainMaterial,
    },
    playing_state::{
        coord_rebasing::{plugin::CoordRebasingPlugin, world_space_transf},
        environment_light::plugin::EnvironmentLightPlugin,
        pause_menu::plugin::PauseMenuPlugin,
        player::{plugin::PlayerPlugin, tags::CameraForPlayer},
        resources::RenderingResolutionScale,
        reusable_materials::ReusableMaterials,
        sets::{
            DURING_PLAYING_UNPAUSED_LIST, DuringPlaying, ON_ENTER_PLAYING_LIST,
            ON_EXIT_PLAYING_LIST, OnEnterPlaying, OnExitPlaying,
        },
        skybox::plugin::SkyboxPlugin,
        states::PauseState,
        tags::PlayingStateEntity,
        terrain::plugin::TerrainPlugin,
        water_layer::plugin::WaterLayerPlugin,
    },
    util::{alrrs, get_entity_components},
};

pub struct PlayingStatePlugin;

impl Plugin for PlayingStatePlugin {
    fn build(&self, app: &mut App) {
        #[rustfmt::skip]
        app
            .configure_sets(Update, (
                DuringPlaying
                    .run_if(in_state(OverallState::Playing)),
                DURING_PLAYING_UNPAUSED_LIST
                    .in_set(DuringPlaying)
                    .run_if(in_state(PauseState::Unpaused)),
                DURING_PLAYING_UNPAUSED_LIST.chain(),
            ))
            .configure_sets(OnEnter(OverallState::Playing),
                ON_ENTER_PLAYING_LIST.chain()
            )
            .configure_sets(OnExit(OverallState::Playing),
                ON_EXIT_PLAYING_LIST.chain()
            )
            .init_state::<PauseState>()
            .add_message::<UpdatePrerenderingStuff>()
            .add_systems(OnEnter(OverallState::Playing),
                on_enter
                    .in_set(OnEnterPlaying::PlayingStatePluginUseOnly)
            )
            .add_systems(OnExit(OverallState::Playing),
                on_exit
                    .in_set(OnExitPlaying::PlayingStatePluginUseOnly)
            )
            .add_systems(Update,
                handle_update_prerendering_stuff
                    .in_set(DuringPlaying)
            )
            .add_systems(Update,
                update_prerendering_stuff_on_window_resize
                    .in_set(DuringPlaying)
            )
            .add_systems(Update,
                toggle_pause
                    .in_set(DuringPlaying)
            )
            .add_systems(Update,
                playing_state_entity_check
                    .in_set(DuringPlaying)
            ) // TODO: only do this in debug mode
            .add_plugins(SkyboxPlugin)
            .add_plugins(EnvironmentLightPlugin)
            .add_plugins(TerrainPlugin)
            .add_plugins(PauseMenuPlugin)
            .add_plugins(PlayerPlugin)
            .add_plugins(CoordRebasingPlugin)
            .add_plugins(WaterLayerPlugin)
        ;
    }
}

fn on_enter(
    mut commands: Commands,
    global_render_data_handle: Res<GlobalRenderDataHandle>,
    mut materials_primary: ResMut<Assets<PrimaryMaterial>>,
    mut materials_terrain: ResMut<Assets<TerrainMaterial>>,
    mut images: ResMut<Assets<Image>>,
    mut update_prerendering_stuff_messages: MessageWriter<UpdatePrerenderingStuff>,
    window: Single<&Window>,
    mut next_pause_state: ResMut<NextState<PauseState>>,
) {
    commands.insert_resource(ReusableMaterials::new(
        global_render_data_handle.get_handle(),
        &mut materials_primary,
        &mut materials_terrain,
    ));

    commands.insert_resource(RenderingResolutionScale(1.0));

    // Prerendering stuff (so that screen resolution can be changed).
    {
        let prerender_target_texture = images.add(Image::new_target_texture(
            1,
            1,
            TextureFormat::Rgba8Unorm,
            Some(TextureFormat::Rgba8UnormSrgb),
        ));
        commands.insert_resource(PrerenderTargetTexture(prerender_target_texture.clone()));

        // The camera that sees all the actual stuff in the world and
        //   renders it to prerender_target_texture.
        commands.spawn((
            PlayingStateEntity,
            CameraForPlayer,
            Camera3d::default(),
            Camera {
                order: -1,
                ..default()
            },
            RenderTarget::Image(prerender_target_texture.clone().into()),
            Projection::Perspective(PerspectiveProjection {
                fov: 60f32.to_radians(),
                ..default()
            }),
            AmbientLight {
                brightness: 0.,
                ..default()
            },
            world_space_transf(Transform::from_xyz(0., 3., 7.)),
        ));

        let final_render_pass_layer = RenderLayers::layer(1);

        // The camera that only sees the final rendering sprite.
        // Is that camera that actually displays to the real screen (in OverallState::Playing).
        commands.spawn((
            PlayingStateEntity,
            Camera2d::default(),
            final_render_pass_layer.clone(),
        ));

        // The final rendering sprite (just displays the prerender_target_texture).
        commands.spawn((
            PlayingStateEntity,
            FinalRenderingSpriteTag,
            Sprite {
                image: prerender_target_texture.clone(),
                custom_size: Some(Vec2::new(1., 1.)),
                ..default()
            },
            final_render_pass_layer.clone(),
        ));

        update_prerendering_stuff_messages.write(UpdatePrerenderingStuff {
            window_size: Vec2::new(window.width(), window.height()),
        });
    }

    next_pause_state.set(PauseState::Unpaused);
}

fn on_exit(
    mut commands: Commands,
    entities: &Entities,
    all_entities_q: Query<Entity, With<PlayingStateEntity>>,
    mut next_pause_state: ResMut<NextState<PauseState>>,
) {
    all_entities_q.iter().for_each(|entity| {
        if entities.contains(entity) {
            commands.entity(entity).try_despawn();
        }
    });

    next_pause_state.set(PauseState::Limbo);
}

#[derive(Resource)]
struct PrerenderTargetTexture(Handle<Image>);

#[derive(Component)]
struct FinalRenderingSpriteTag;

#[derive(Message)]
struct UpdatePrerenderingStuff {
    window_size: Vec2,
}

fn handle_update_prerendering_stuff(
    mut messages: MessageReader<UpdatePrerenderingStuff>,
    scale: Res<RenderingResolutionScale>,
    mut images: ResMut<Assets<Image>>,
    prerender_target_texture: Res<PrerenderTargetTexture>,
    mut final_rendering_sprite_q: Option<Single<&mut Sprite, With<FinalRenderingSpriteTag>>>,
) {
    messages.read().for_each(|msg| {
        let width = (msg.window_size.x * scale.0) as u32;
        let height = (msg.window_size.y * scale.0) as u32;

        images
            .get_mut(&prerender_target_texture.0)
            .unwrap()
            .resize(Extent3d {
                width: width,
                height: height,
                ..default()
            });

        alrrs!(&mut final_rendering_sprite_q).custom_size =
            Some(Vec2::new(msg.window_size.x, msg.window_size.y));
    });
}

fn update_prerendering_stuff_on_window_resize(
    mut window_resized: MessageReader<WindowResized>,
    mut update_prerendering_stuff_messages: MessageWriter<UpdatePrerenderingStuff>,
) {
    window_resized.read().for_each(|msg| {
        update_prerendering_stuff_messages.write(UpdatePrerenderingStuff {
            window_size: Vec2::new(msg.width, msg.height),
        });
    });
}

fn toggle_pause(
    keys: Res<ButtonInput<KeyCode>>,
    key_bindings: Res<KeyBindings>,
    pause_state: Res<State<PauseState>>,
    mut next_pause_state: ResMut<NextState<PauseState>>,
) {
    if keys.just_pressed(key_bindings.pause) {
        next_pause_state.set(match pause_state.get() {
            PauseState::Limbo => {
                error!("PauseState was in Limbo. Setting it to Unpaused.");
                PauseState::Unpaused
            }
            PauseState::Unpaused => PauseState::Paused,
            PauseState::Paused => PauseState::Unpaused,
        });
    }
}

fn playing_state_entity_check(
    world: &World,
    entity_q: Query<Entity, (Added<Transform>, Without<PlayingStateEntity>)>,
) {
    entity_q.iter().for_each(|entity| {
        warn!("The following entity with a Transform was spawned without the PlayingStateEntity tag, meaning it won't get despawned when OverallState::Playing is exited: \n{}", get_entity_components(world, entity));
    });
}
