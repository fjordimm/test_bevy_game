use std::time::Duration;

use bevy::{
    diagnostic::{DiagnosticsStore, EntityCountDiagnosticsPlugin, FrameTimeDiagnosticsPlugin},
    prelude::*,
    time::common_conditions::on_timer,
};

use crate::game::{
    core::resources::{GlobalGuiRoot, KeyBindings},
    diagnosis::resources::LagSpikeDiag,
    gui::{
        gui_children,
        resources::GuiThemeComputed,
        widgets::{
            div::{GuiDivCustomStyle, GuiDivProps, GuiDivStyle, gui_div, gui_div_p},
            floating_panel::{
                GuiFloatingPanelInterface, GuiFloatingPanelProps, gui_floating_panel,
            },
            text::{GuiTextInterface, gui_text_h2, gui_text_m},
        },
    },
    playing_state::player::tags::CameraForPlayer,
};

pub struct PrimaryDebugMenuPlugin;

impl Plugin for PrimaryDebugMenuPlugin {
    fn build(&self, app: &mut App) {
        #[rustfmt::skip]
        app
            .add_systems(Update,
                spawn_primary_debug_menu
                    .run_if(run_once)
            )
            .add_systems(Update, toggle_debug_menu)
            .add_systems(Update,
                update
                    .run_if(on_timer(Duration::from_millis(100)))
            )
        ;
    }
}

#[derive(Component)]
struct PrimaryDebugMenuTag;

#[derive(Component)]
struct CoreSection;

#[derive(Component)]
struct FpsText;

#[derive(Component)]
struct LagSpikeText;

#[derive(Component)]
struct EntityCountText;

#[derive(Component)]
struct OverallStatePlayingSection;

#[derive(Component)]
struct TransformEntityCountText;

#[derive(Component)]
struct CamPositionText;

fn spawn_primary_debug_menu(
    mut commands: Commands,
    gui_root: Res<GlobalGuiRoot>,
    theme: Res<GuiThemeComputed>,
) {
    let theme_padding_main = theme.0.padding_main;

    let debug_menu = commands
        .spawn(gui_floating_panel(
            "Primary Debug Menu",
            GuiFloatingPanelProps {
                starts_active: false,
                starting_content_height: theme.0.primary_debug_menu_starting_height,
                ..default()
            },
        ))
        .insert(gui_children(move |p| {
            p.spawn(gui_div(GuiDivProps {
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::FlexStart,
                align_items: AlignItems::FlexStart,
                div_style: GuiDivStyle::Custom(GuiDivCustomStyle {
                    gap: theme_padding_main,
                    ..default()
                }),
                expands_along_main_axis: true,
                expands_along_cross_axis: true,
                ..default()
            }))
            .insert(gui_children(|p| {
                p.spawn((
                    CoreSection,
                    gui_div(GuiDivProps {
                        flex_direction: FlexDirection::Column,
                        justify_content: JustifyContent::FlexStart,
                        align_items: AlignItems::FlexStart,
                        div_style: GuiDivStyle::RegularStyled,
                        expands_along_cross_axis: true,
                        ..default()
                    }),
                ))
                .insert(gui_children(|p| {
                    p.spawn(gui_div_p()).insert(gui_children(|p| {
                        p.spawn(gui_text_m("FPS: "));
                        p.spawn((FpsText, gui_text_m("-")));
                    }));

                    p.spawn(gui_div_p()).insert(gui_children(|p| {
                        p.spawn(gui_text_m("Recent Stutter (ms): "));
                        p.spawn((LagSpikeText, gui_text_m("-")));
                    }));

                    p.spawn(gui_div_p()).insert(gui_children(|p| {
                        p.spawn(gui_text_m("Entity Count: "));
                        p.spawn((EntityCountText, gui_text_m("-")));
                    }));
                }));

                p.spawn((
                    OverallStatePlayingSection,
                    gui_div(GuiDivProps {
                        flex_direction: FlexDirection::Column,
                        justify_content: JustifyContent::FlexStart,
                        align_items: AlignItems::FlexStart,
                        div_style: GuiDivStyle::RegularStyled,
                        expands_along_cross_axis: true,
                        ..default()
                    }),
                ))
                .insert(gui_children(|p| {
                    p.spawn(gui_text_h2("OverallState::Playing"));

                    p.spawn(gui_div_p()).insert(gui_children(|p| {
                        p.spawn(gui_text_m("Transform Entity Count: "));
                        p.spawn((TransformEntityCountText, gui_text_m("-")));
                    }));

                    p.spawn(gui_div_p()).insert(gui_children(|p| {
                        p.spawn(gui_text_m("Cam Position: "));
                        p.spawn((CamPositionText, gui_text_m("-")));
                    }));
                }));
            }));
        }))
        .insert(PrimaryDebugMenuTag)
        .insert(ZIndex(4000))
        .id();

    commands.entity(gui_root.0).add_child(debug_menu);
}

fn toggle_debug_menu(
    keys: Res<ButtonInput<KeyCode>>,
    key_bindings: Res<KeyBindings>,
    mut debug_menu_q: Query<GuiFloatingPanelInterface, With<PrimaryDebugMenuTag>>,
) {
    if keys.just_pressed(key_bindings.toggle_debug_menu) {
        debug_menu_q.iter_mut().for_each(|mut debug_menu| {
            debug_menu.set_is_active(!debug_menu.is_active());
        });
    }
}

fn update(
    diagnostics: Res<DiagnosticsStore>,
    mut fps_text: Query<
        GuiTextInterface,
        (
            With<FpsText>,
            Without<LagSpikeText>,
            Without<EntityCountText>,
            Without<TransformEntityCountText>,
            Without<CamPositionText>,
        ),
    >,
    mut lag_spike_text: Query<
        GuiTextInterface,
        (
            With<LagSpikeText>,
            Without<FpsText>,
            Without<EntityCountText>,
            Without<TransformEntityCountText>,
            Without<CamPositionText>,
        ),
    >,
    lag_spike_diag: Res<LagSpikeDiag>,
    mut entity_count_text: Query<
        GuiTextInterface,
        (
            With<EntityCountText>,
            Without<FpsText>,
            Without<LagSpikeText>,
            Without<TransformEntityCountText>,
            Without<CamPositionText>,
        ),
    >,
    mut transform_entity_count_text: Query<
        GuiTextInterface,
        (
            With<TransformEntityCountText>,
            Without<FpsText>,
            Without<LagSpikeText>,
            Without<EntityCountText>,
            Without<CamPositionText>,
        ),
    >,
    transform_q: Query<(), With<Transform>>,
    mut cam_position_text: Query<
        GuiTextInterface,
        (
            With<CamPositionText>,
            Without<TransformEntityCountText>,
            Without<FpsText>,
            Without<LagSpikeText>,
            Without<EntityCountText>,
        ),
    >,
    camera_q: Option<Single<&Transform, With<CameraForPlayer>>>,
) {
    fps_text.iter_mut().for_each(|mut text| {
        text.set_content(
            match diagnostics.get_measurement(&FrameTimeDiagnosticsPlugin::FPS) {
                None => String::from("-"),
                Some(measurement) => (measurement.value as i32).to_string(),
            },
        );
    });

    lag_spike_text.iter_mut().for_each(|mut text| {
        text.set_content(format!("{}", lag_spike_diag.0));
    });

    entity_count_text.iter_mut().for_each(|mut text| {
        text.set_content(
            match diagnostics.get_measurement(&EntityCountDiagnosticsPlugin::ENTITY_COUNT) {
                None => String::from("-"),
                Some(measurement) => (measurement.value as i32).to_string(),
            },
        );
    });

    transform_entity_count_text.iter_mut().for_each(|mut text| {
        text.set_content(transform_q.iter().count().to_string());
    });

    if let Some(camera_transf) = camera_q {
        cam_position_text.iter_mut().for_each(|mut text| {
            let t = camera_transf.translation;
            text.set_content(format!("{:.2}, {:.2}, {:.2}", t.x, t.y, t.z));
        });
    } else {
        cam_position_text.iter_mut().for_each(|mut text| {
            text.set_content(String::from("-"));
        });
    }
}
