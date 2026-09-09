use bevy::prelude::*;

use crate::game::{
    core::{resources::GlobalGuiRoot, states::OverallState},
    gui::{
        gui_child, gui_children,
        resources::GuiThemeComputed,
        widgets::{
            button::gui_button,
            div::{GuiDivCustomStyle, GuiDivProps, GuiDivStyle, gui_div},
            floating_panel::{GuiFloatingPanelProps, gui_floating_panel},
            text::gui_text_p,
        },
    },
    playing_state::sets::{OnEnterPlaying, OnExitPlaying},
};

pub struct CheatsMenuPlugin;

impl Plugin for CheatsMenuPlugin {
    fn build(&self, app: &mut App) {
        #[rustfmt::skip]
        app
            .add_systems(OnEnter(OverallState::Playing),
                spawn_cheats_menu
                    .in_set(OnEnterPlaying::General)
            )
            .add_systems(OnExit(OverallState::Playing),
                despawn_cheats_menu
                    .in_set(OnExitPlaying::General)
            )
        ;
    }
}

#[derive(Component)]
struct CheatsMenuTag;

fn spawn_cheats_menu(
    mut commands: Commands,
    gui_root: Res<GlobalGuiRoot>,
    theme: Res<GuiThemeComputed>,
) {
    let theme_padding_main = theme.0.padding_main;

    let cheats_menu = commands
        .spawn(gui_floating_panel(
            "Cheats",
            GuiFloatingPanelProps {
                starts_active: true,
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
                p.spawn(gui_button(default()))
                    .insert(gui_child(gui_text_p("Bruh")))
                    .observe(|_: On<Pointer<Click>>| {
                        debug!("ahahahahaa");
                    });
            }));
        }))
        .insert(CheatsMenuTag)
        .insert(ZIndex(4000))
        .id();

    commands.entity(gui_root.0).add_child(cheats_menu);
}

fn despawn_cheats_menu(mut commands: Commands, cheats_menu_q: Query<Entity, With<CheatsMenuTag>>) {
    cheats_menu_q.iter().for_each(|entity| {
        commands.entity(entity).despawn();
    });
}
