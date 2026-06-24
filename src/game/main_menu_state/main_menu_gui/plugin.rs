use bevy::prelude::*;

use crate::game::{
    core::{resources::GlobalGuiRoot, states::OverallState},
    gui::{
        resources::GuiThemeComputed,
        widgets::{
            button::gui_button,
            div::{GuiDivProps, GuiDivStyle, gui_div},
            screen_div::{GuiScreenDivProps, gui_screen_div},
            text::gui_text,
        },
    },
};

pub struct MainMenuGuiPlugin;

impl Plugin for MainMenuGuiPlugin {
    fn build(&self, app: &mut App) {
        #[rustfmt::skip]
        app
            .add_systems(OnEnter(OverallState::MainMenu), spawn_main_menu_gui)
            .add_systems(OnExit(OverallState::MainMenu), despawn_main_menu_gui)
        ;
    }
}

#[derive(Component)]
struct MainMenuGuiTag;

fn spawn_main_menu_gui(
    mut commands: Commands,
    gui_root: Res<GlobalGuiRoot>,
    theme: Res<GuiThemeComputed>,
) {
    let thing = commands
        .spawn((
            MainMenuGuiTag,
            gui_screen_div(GuiScreenDivProps {
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                bg_color: theme.0.bg_color_main,
                ..default()
            }),
        ))
        .with_children(|parent| {
            parent
                .spawn(gui_div(GuiDivProps {
                    flex_direction: FlexDirection::Column,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    div_style: GuiDivStyle::Regular,
                    ..default()
                }))
                .with_children(|parent| {
                    parent.spawn(gui_text("Howdy", default()));
                    parent
                        .spawn(gui_button(default()))
                        .with_children(|parent| {
                            parent.spawn(gui_text("pressme", default()));
                        })
                        .observe(|_: On<Pointer<Click>>| {
                            debug!("oh no i've been clicked!!!!");
                        });
                });
        })
        .id();

    commands.entity(gui_root.0).add_child(thing);
}

fn despawn_main_menu_gui(
    mut commands: Commands,
    main_menu_gui_q: Query<Entity, With<MainMenuGuiTag>>,
) {
    main_menu_gui_q.iter().for_each(|entity| {
        commands.entity(entity).despawn();
    });
}
