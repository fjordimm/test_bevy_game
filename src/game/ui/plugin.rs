use bevy::prelude::*;
use bevy_ecs::schedule::ScheduleLabel;
use bevy_egui::{EguiContext, EguiMultipassSchedule, EguiPlugin};

use crate::game::ui::tags::CameraForEgui;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        #[rustfmt::skip]
        app
            .add_plugins(EguiPlugin::default())
            .add_systems(Update, on_creation_of_camera_for_egui)
            .add_systems(Update, on_creation_of_egui_context);
    }
}

fn on_creation_of_camera_for_egui(
    cameras: Query<Entity, Added<CameraForEgui>>,
    mut commands: Commands,
) {
    for camera in &cameras {
        commands
            .entity(camera)
            .insert(EguiMultipassSchedule(CameraForEgui.intern()));
    }
}

fn on_creation_of_egui_context(mut egui_contexts: Query<&mut EguiContext, Added<EguiContext>>) {
    for mut egui_context in &mut egui_contexts {
        let ctx = egui_context.get_mut();

        ctx.style_mut(|style| {
            style.animation_time = 0.0;
        });
    }
}
