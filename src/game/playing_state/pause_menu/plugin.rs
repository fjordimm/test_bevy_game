use bevy::prelude::*;

use crate::game::{
    core::{resources::GlobalGuiRoot, states::OverallState},
    gui::{
        gui_child, gui_children,
        resources::GuiThemeComputed,
        widgets::{
            button::gui_button,
            div::{GuiDivProps, GuiDivStyle, gui_div},
            screen_div::{GuiScreenDivProps, gui_screen_div},
            text::{gui_text_h1, gui_text_h2},
        },
    },
    playing_state::states::PauseState,
};

pub struct PauseMenuPlugin;

impl Plugin for PauseMenuPlugin {
    fn build(&self, app: &mut App) {
        #[rustfmt::skip]
        app
            .add_systems(OnEnter(PauseState::Paused), spawn_pause_menu)
            .add_systems(OnExit(PauseState::Paused), despawn_pause_menu)
        ;
    }
}

#[derive(Component)]
struct PauseMenuTag;

fn spawn_pause_menu(
    mut commands: Commands,
    gui_root: Res<GlobalGuiRoot>,
    theme: Res<GuiThemeComputed>,
) {
    let main_menu_gui = commands
        .spawn((gui_screen_div(GuiScreenDivProps {
            flex_direction: FlexDirection::Column,
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            bg_color: theme.0.pause_menu_bg_color,
            ..default()
        }),))
        .insert(gui_children(|p| {
            p.spawn(gui_div(GuiDivProps {
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Stretch,
                div_style: GuiDivStyle::Regular,
                ..default()
            }))
            .insert(gui_children(|p: &mut ChildSpawner| {
                p.spawn(gui_div(GuiDivProps {
                    flex_direction: FlexDirection::Column,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    div_style: GuiDivStyle::None,
                    ..default()
                }))
                .insert(gui_child(gui_text_h1("Pause Menu")));

                p.spawn(gui_button(default()))
                    .insert(gui_child(gui_text_h2("Continue")))
                    .observe(|_: On<Pointer<Click>>, mut commands: Commands| {
                        commands.set_state(PauseState::Unpaused);
                    });

                p.spawn(gui_button(default()))
                    .insert(gui_child(gui_text_h2("Exit")))
                    .observe(|_: On<Pointer<Click>>, mut commands: Commands| {
                        commands.set_state(OverallState::MainMenu);
                    });
            }));
        }))
        .insert(PauseMenuTag)
        .insert(ZIndex(3000))
        .id();

    commands.entity(gui_root.0).add_child(main_menu_gui);
}

fn despawn_pause_menu(mut commands: Commands, pause_menu_q: Query<Entity, With<PauseMenuTag>>) {
    pause_menu_q.iter().for_each(|entity| {
        commands.entity(entity).despawn();
    });
}
