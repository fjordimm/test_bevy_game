use bevy::prelude::*;
use bevy_ecs::entity::Entities;

use crate::game::{
    core::{resources::KeyBindings, states::OverallState},
    graphics::primary_shader::plugin::PrimaryShaderMaterial,
    playing_state::{
        coord_rebasing::{plugin::CoordRebasingPlugin, world_space_transf},
        environment_light::plugin::EnvironmentLightPlugin,
        pause_menu::plugin::PauseMenuPlugin,
        player::{plugin::PlayerPlugin, tags::CameraForPlayer},
        reusable_materials::ReusableMaterials,
        sets::{
            DURING_PLAYING_UNPAUSED_LIST, DuringPlaying, ON_ENTER_PLAYING_LIST,
            ON_EXIT_PLAYING_LIST, OnEnterPlaying, OnExitPlaying,
        },
        skybox::plugin::SkyboxPlugin,
        states::PauseState,
        tags::PlayingStateEntity,
        terrain::plugin::TerrainPlugin,
    },
    util::get_entity_components,
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
            .add_systems(OnEnter(OverallState::Playing),
                on_enter
                    .in_set(OnEnterPlaying::PlayingStatePluginUseOnly)
            )
            .add_systems(OnExit(OverallState::Playing),
                on_exit
                    .in_set(OnExitPlaying::PlayingStatePluginUseOnly)
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
        ;
    }
}

fn on_enter(
    mut commands: Commands,
    mut next_pause_state: ResMut<NextState<PauseState>>,
    mut materials: ResMut<Assets<PrimaryShaderMaterial>>,
) {
    commands.insert_resource(ReusableMaterials::new(&mut materials));

    commands.spawn((
        PlayingStateEntity,
        CameraForPlayer,
        Camera3d::default(),
        Projection::Perspective(PerspectiveProjection {
            fov: 60.0f32.to_radians(),
            ..default()
        }),
        world_space_transf(Transform::from_xyz(0., 3., 7.)),
        AmbientLight {
            brightness: 0.,
            ..default()
        },
    ));

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
