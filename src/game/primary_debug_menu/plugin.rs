use std::time::Duration;

use bevy::{
    diagnostic::{DiagnosticsStore, EntityCountDiagnosticsPlugin, FrameTimeDiagnosticsPlugin},
    prelude::*,
    time::common_conditions::on_timer,
};

use crate::game::{
    core::resources::{GlobalGuiRoot, KeyBindings},
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
                    .run_if(on_timer(Duration::from_millis(250)))
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
struct EntityCountText;

#[derive(Component)]
struct OverallStatePlayingSection;

#[derive(Component)]
struct TransformEntityCountText;

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
            Without<EntityCountText>,
            Without<TransformEntityCountText>,
        ),
    >,
    mut entity_count_text: Query<
        GuiTextInterface,
        (
            With<EntityCountText>,
            Without<FpsText>,
            Without<TransformEntityCountText>,
        ),
    >,
    transform_q: Query<(), With<Transform>>,
    mut transform_entity_count_text: Query<
        GuiTextInterface,
        (
            With<TransformEntityCountText>,
            Without<FpsText>,
            Without<EntityCountText>,
        ),
    >,
) {
    fps_text.iter_mut().for_each(|mut text| {
        text.set_content(
            match diagnostics.get_measurement(&FrameTimeDiagnosticsPlugin::FPS) {
                None => String::from("-"),
                Some(measurement) => (measurement.value as i32).to_string(),
            },
        );
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
}
