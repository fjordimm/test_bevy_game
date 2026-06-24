use bevy::prelude::*;

use crate::game::{
    core::{resources::GlobalGuiRoot, states::OverallState},
    gui::widgets::{
        button::gui_button,
        div::{GuiDivProps, GuiDivStyle, gui_div},
        screen_div::{GuiScreenDivProps, gui_screen_div},
        text::gui_text,
    },
};

pub struct MainMenuGuiPlugin;

impl Plugin for MainMenuGuiPlugin {
    fn build(&self, app: &mut App) {
        #[rustfmt::skip]
        app
            .add_systems(OnEnter(OverallState::MainMenu), todor1)
        ;
    }
}

fn todor1(mut commands: Commands, gui_root: Res<GlobalGuiRoot>) {
    let thing = commands
        .spawn(gui_div(GuiDivProps {
            flex_direction: FlexDirection::Column,
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            div_style: GuiDivStyle::Regular,
            ..default()
        }))
        .with_children(|parent| {
            parent
                .spawn(gui_screen_div(GuiScreenDivProps {
                    flex_direction: FlexDirection::Column,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    bg_color: Color::BLACK,
                    ..default()
                }))
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
                });
        })
        .id();

    commands.entity(gui_root.0).add_child(thing);
}
