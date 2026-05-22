use bevy::prelude::*;

use crate::game::{
    core::{resources::KeyBindings, states::OverallState},
    playing_state::{
        SunPosition,
        pause_menu::plugin::PauseMenuPlugin,
        player::{plugin::PlayerPlugin, tags::CameraForPlayer},
        sets::{
            DURING_PLAYING_UNPAUSED_LIST, DuringPlaying, DuringPlayingUnpaused,
            DuringPlayingUnpausedW,
        },
        skybox::plugin::SkyboxPlugin,
        states::PauseState,
        tags::PlayingStateEntity,
        world::plugin::WorldPlugin,
    },
};

pub struct PlayingStatePlugin;

impl Plugin for PlayingStatePlugin {
    fn build(&self, app: &mut App) {
        #[rustfmt::skip]
        app
            .configure_sets(Update, (
                DuringPlaying
                    .run_if(in_state(OverallState::Playing)),
                DuringPlayingUnpausedW
                    .in_set(DuringPlaying)
                    .run_if(in_state(PauseState::Unpaused)),
                DURING_PLAYING_UNPAUSED_LIST
                    .in_set(DuringPlayingUnpausedW),
                DURING_PLAYING_UNPAUSED_LIST.chain(),
            ))
            .init_state::<PauseState>()
            .insert_resource(SunPosition(Vec3::new(0.0, 0.5, -1.0).normalize()))
            .add_systems(OnEnter(OverallState::Playing),
                on_enter
                    .in_set(DuringPlayingUnpausedW)
                    .before(<[DuringPlayingUnpaused; _]>::from(DURING_PLAYING_UNPAUSED_LIST).first().unwrap().clone())
            )
            .add_systems(OnExit(OverallState::Playing),
                on_exit
                    .in_set(DuringPlayingUnpausedW)
                    .after(<[DuringPlayingUnpaused; _]>::from(DURING_PLAYING_UNPAUSED_LIST).last().unwrap().clone())
            )
            .add_systems(Update,
                toggle_pause
                    .in_set(DuringPlaying)
            )
            .add_systems(Update,
                rotate_sun
                    .in_set(DuringPlayingUnpaused::General)
            )
            .add_plugins(PauseMenuPlugin)
            .add_plugins(SkyboxPlugin)
            .add_plugins(WorldPlugin)
            .add_plugins(PlayerPlugin);
    }
}

fn on_enter(mut commands: Commands, mut next_pause_state: ResMut<NextState<PauseState>>) {
    next_pause_state.set(PauseState::Unpaused);

    commands.spawn((
        PlayingStateEntity,
        CameraForPlayer,
        Camera3d::default(),
        Projection::Perspective(PerspectiveProjection {
            fov: 60.0f32.to_radians(),
            ..default()
        }),
        Transform::from_xyz(0.0, 3.0, 7.0),
        AmbientLight {
            brightness: 80.0,
            ..default()
        },
    ));
}

fn on_exit(mut commands: Commands, all_entities_q: Query<Entity, With<PlayingStateEntity>>) {
    all_entities_q.iter().for_each(|entity| {
        commands.entity(entity).despawn();
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
            PauseState::Unpaused => PauseState::Paused,
            PauseState::Paused => PauseState::Unpaused,
        });
    }
}

fn rotate_sun(
    time: Res<Time>,
    mut sun_position: ResMut<SunPosition>,
    keys: Res<ButtonInput<KeyCode>>,
) {
    if keys.just_pressed(KeyCode::Digit1) {
        sun_position.0 = Vec3::Y.rotate_x(-0.0f32.to_radians())
    }
    if keys.just_pressed(KeyCode::Digit2) {
        sun_position.0 = Vec3::Y.rotate_x(-45.0f32.to_radians())
    }
    if keys.just_pressed(KeyCode::Digit3) {
        sun_position.0 = Vec3::Y.rotate_x(-60.0f32.to_radians())
    }
    if keys.just_pressed(KeyCode::Digit4) {
        sun_position.0 = Vec3::Y.rotate_x(-75.0f32.to_radians())
    }
    if keys.just_pressed(KeyCode::Digit5) {
        sun_position.0 = Vec3::Y.rotate_x(-85.0f32.to_radians())
    }
    if keys.just_pressed(KeyCode::Digit6) {
        sun_position.0 = Vec3::Y.rotate_x(-90.0f32.to_radians())
    }
    if keys.just_pressed(KeyCode::Digit7) {
        sun_position.0 = Vec3::Y.rotate_x(-105.0f32.to_radians())
    }
    if keys.just_pressed(KeyCode::Digit8) {
        sun_position.0 = Vec3::Y.rotate_x(-135.0f32.to_radians())
    }
    if keys.just_pressed(KeyCode::Digit9) {
        sun_position.0 = Vec3::Y.rotate_x(-180.0f32.to_radians())
    }

    // sun_position.0 = sun_position.0.rotate_z(0.3 * time.delta_secs());
}
