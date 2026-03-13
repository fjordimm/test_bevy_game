use bevy::prelude::*;
use bevy_ecs::schedule::ScheduleLabel;
use bevy_egui::{EguiContext, EguiMultipassSchedule, egui};

use crate::game::{
    core::{global_resources::KeyBindings, states::OverallState},
    playing_state::{
        player::PlayerPlugin,
        sets::GameSet,
        states::PauseState,
        tags::{PlayingStateCameraForEgui, PlayingStateEntity},
        world::WorldPlugin,
    },
};

pub struct PlayingStatePlugin;

impl Plugin for PlayingStatePlugin {
    fn build(&self, app: &mut App) {
        #[rustfmt::skip]
        app
            .configure_sets(
                Update, (
                    GameSet::Input,
                    GameSet::Movement,
                    GameSet::Physics,
                    GameSet::World,
                ).chain(),
            )
            .init_state::<PauseState>()
            .add_systems(OnEnter(OverallState::Playing), on_enter_state)
            .add_systems(OnExit(OverallState::Playing), on_exit_state)
            .add_systems(Update, toggle_pause.run_if(in_state(OverallState::Playing)))
            .add_systems(PlayingStateCameraForEgui, pause_gui.run_if(in_state(OverallState::Playing)).run_if(in_state(PauseState::Paused)))
            .add_plugins(WorldPlugin)
            .add_plugins(PlayerPlugin);
    }
}

fn on_enter_state(mut commands: Commands, mut next_pause_state: ResMut<NextState<PauseState>>) {
    debug!("playing_state on_enter_state");

    next_pause_state.set(PauseState::Unpaused);

    commands.spawn((
        PlayingStateEntity,
        PlayingStateCameraForEgui,
        EguiMultipassSchedule(PlayingStateCameraForEgui.intern()),
        Camera3d::default(),
        Transform::from_xyz(0.0, 0.0, 0.0).looking_at(-Vec3::Z, Vec3::Y),
    ));
}

fn on_exit_state(mut commands: Commands, query: Query<Entity, With<PlayingStateEntity>>) {
    debug!("playing_state on_exit_state");

    for entity in &query {
        commands.entity(entity).despawn();
    }
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

fn pause_gui(
    mut egui_context: Single<&mut EguiContext, With<PlayingStateCameraForEgui>>,
    mut next_pause_state: ResMut<NextState<PauseState>>,
    mut next_overall_state: ResMut<NextState<OverallState>>,
) -> Result {
    let ctx = egui_context.get_mut();

    egui::Area::new("pause_gui_background".into())
        .fixed_pos(egui::pos2(0.0, 0.0))
        .order(egui::Order::Background)
        .show(ctx, |ui| {
            ui.painter().rect_filled(
                ctx.viewport_rect(),
                0.0,
                egui::Color32::from_black_alpha(180),
            );
        });

    egui::Area::new("pause_gui_menu".into())
        .anchor(egui::Align2::CENTER_CENTER, egui::vec2(0.0, 0.0))
        .order(egui::Order::Foreground)
        .show(ctx, |ui| {
            egui::Frame::NONE
                .fill(egui::Color32::from_rgb(34, 58, 51))
                .corner_radius(egui::CornerRadius::same(12))
                .inner_margin(egui::Margin::same(32))
                .show(ui, |ui| {
                    ui.vertical_centered(|ui| {
                        ui.heading("Paused");
                        if ui.button("Resume").clicked() {
                            next_pause_state.set(PauseState::Unpaused);
                        }
                        if ui.button("Exit to Main Menu").clicked() {
                            next_overall_state.set(OverallState::MainMenu)
                        }
                    });
                });
        });

    Ok(())
}
