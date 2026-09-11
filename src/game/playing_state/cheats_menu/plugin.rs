use bevy::prelude::*;

use crate::game::{
    core::{resources::GlobalGuiRoot, states::OverallState},
    gui::{
        gui_child, gui_children,
        resources::GuiThemeComputed,
        widgets::{
            button::gui_button,
            checkbox::{bind_checkbox_with_resource, gui_checkbox},
            div::{GuiDivCustomStyle, GuiDivProps, GuiDivStyle, gui_div, gui_div_p},
            floating_panel::{GuiFloatingPanelProps, gui_floating_panel},
            text::gui_text_p,
        },
    },
    playing_state::{
        player::{resources::FreecamEnabled, tags::PlayerBody},
        sets::{OnEnterPlaying, OnExitPlaying},
        tags::PrimaryCamera,
    },
    util::alrrs,
};

pub struct CheatsMenuPlugin;

impl Plugin for CheatsMenuPlugin {
    fn build(&self, app: &mut App) {
        #[rustfmt::skip]
        app
            .add_systems(OnEnter(OverallState::Playing),
                spawn_cheats_menu
                    .in_set(OnEnterPlaying::SpawnThings)
            )
            .add_systems(OnExit(OverallState::Playing),
                despawn_cheats_menu
                    .in_set(OnExitPlaying::General)
            )
            .add_systems(Update, bind_checkbox_with_resource!(EnableFreecamCheckbox, FreecamEnabled))
        ;
    }
}

#[derive(Component)]
pub struct CheatsMenuTag;

#[derive(Component)]
struct EnableFreecamCheckbox;

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
                p.spawn(gui_div_p()).insert(gui_children(|p| {
                    p.spawn(gui_text_p("Enable freecam: "));
                    p.spawn((EnableFreecamCheckbox, gui_checkbox(default())));
                }));

                p.spawn(gui_button(default()))
                    .insert(gui_child(gui_text_p("Player to cam")))
                    .observe(player_to_cam_button_observer);
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

fn player_to_cam_button_observer(
    _: On<Pointer<Click>>,
    camera_q: Option<Single<&Transform, With<PrimaryCamera>>>,
    player_body_q: Option<Single<&mut Transform, (With<PlayerBody>, Without<PrimaryCamera>)>>,
) {
    let camera = alrrs!(camera_q);
    let mut player_body = alrrs!(player_body_q);

    player_body.translation = camera.translation;
}
