use bevy::prelude::*;

use crate::game::{
    core::{global_resources::KeyBindings, states::OverallState},
    gui::{constants::MAIN_COLOR, gui_root_template},
    playing_state::{
        pause_menu::PauseMenuPlugin,
        player::{PlayerPlugin, tags::CameraForPlayer},
        sets::{PLAYING_STATE_ORDERING_ORDER, PlayingStateOrdering},
        states::PauseState,
        tags::PlayingStateEntity,
        world::WorldPlugin,
    },
};

pub struct PlayingStatePlugin;

impl Plugin for PlayingStatePlugin {
    fn build(&self, app: &mut App) {
        #[rustfmt::skip]
        app
            .configure_sets(Update, PLAYING_STATE_ORDERING_ORDER.chain())
            .init_state::<PauseState>()
            .add_systems(OnEnter(OverallState::Playing),
                on_enter
                    .in_set(PlayingStateOrdering::WorldOnEnter)
            )
            .add_systems(OnExit(OverallState::Playing),
                on_exit
                    .in_set(PlayingStateOrdering::WorldOnExit)
            )
            .add_systems(Update,
                toggle_pause
                    .run_if(in_state(OverallState::Playing))
                    .in_set(PlayingStateOrdering::Ui)
            )
            .add_plugins(PauseMenuPlugin)
            .add_plugins(WorldPlugin)
            .add_plugins(PlayerPlugin);
    }
}

#[derive(Resource)]
pub struct PlayingStateGuiRoot(pub Entity);

fn on_enter(mut commands: Commands, mut next_pause_state: ResMut<NextState<PauseState>>) {
    next_pause_state.set(PauseState::Unpaused);

    commands.spawn((
        PlayingStateEntity,
        CameraForPlayer,
        Camera3d::default(),
        Transform::from_xyz(0.0, 0.0, 7.0).looking_at(-Vec3::Z, Vec3::Y),
    ));

    let gui_root = commands.spawn(gui_root_template()).id();
    commands.insert_resource(PlayingStateGuiRoot(gui_root));
}

fn on_exit(
    mut commands: Commands,
    all_entities: Query<Entity, With<PlayingStateEntity>>,
    gui_root: Res<PlayingStateGuiRoot>,
) {
    for entity in &all_entities {
        commands.entity(entity).despawn();
    }

    commands.entity(gui_root.0).despawn();
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
