use bevy::prelude::*;

use crate::game::{
    core::{resources::GlobalGuiRoot, states::OverallState},
    gui::widgets::{
        div::{GuiDivProps, GuiDivStyle, gui_div},
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
                .spawn(gui_div(GuiDivProps {
                    flex_direction: FlexDirection::Column,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    div_style: GuiDivStyle::Regular,
                    ..default()
                }))
                .with_children(|parent| {
                    parent.spawn(gui_text("Howdy", default()));
                });
        })
        .id();

    commands.entity(gui_root.0).add_child(thing);
}
