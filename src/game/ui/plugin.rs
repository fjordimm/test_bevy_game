use bevy::prelude::*;
use bevy_ecs::schedule::ScheduleLabel;
use bevy_egui::{EguiContext, EguiMultipassSchedule, EguiPlugin, egui};

use crate::game::{playing_state::sets::PlayingStateOrdering, ui::tags::CameraForEgui};

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        #[rustfmt::skip]
        app
            .add_plugins(EguiPlugin::default())
            // .add_systems(Update, funny1.after(PlayingStateOrdering::Ui));
        .add_systems(Update, on_creation_of_camera_for_egui)
        .add_systems(Update, on_creation_of_egui_context);
    }
}

fn funny1(
    cameras: Query<Entity, Added<CameraForEgui>>,
    mut commands: Commands,
    mut egui_contexts: Query<&mut EguiContext>,
) {
    for camera in &cameras {
        debug!("camera added");
        debug!("eguis: {}", egui_contexts.iter().count());

        commands
            .entity(camera)
            .insert(EguiMultipassSchedule(CameraForEgui.intern()));

        for mut egui_context in &mut egui_contexts {
            let ctx = egui_context.get_mut();

            ctx.style_mut(|style| {
                style.visuals.override_text_color = Some(egui::Color32::from_rgb(0, 128, 0));
            });
        }
    }
}

fn on_creation_of_camera_for_egui(
    cameras: Query<Entity, Added<CameraForEgui>>,
    mut commands: Commands,
) {
    for camera in &cameras {
        debug!("camera added");

        commands
            .entity(camera)
            .insert(EguiMultipassSchedule(CameraForEgui.intern()));
    }
}

fn on_creation_of_egui_context(mut egui_contexts: Query<&mut EguiContext, Added<CameraForEgui>>) {
    for mut egui_context in &mut egui_contexts {
        debug!("context added");

        let ctx = egui_context.get_mut();

        ctx.style_mut(|style| {
            style.visuals.override_text_color = Some(egui::Color32::from_rgb(0, 128, 0));
        });
    }
}
