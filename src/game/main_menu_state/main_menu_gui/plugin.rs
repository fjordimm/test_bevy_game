use bevy::prelude::*;

use crate::game::{
    core::{resources::GlobalGuiRoot, states::OverallState},
    gui::widgets::text::{GuiTextProps, GuiTextSize, gui_text},
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
        .spawn((
            Node {
                display: Display::Flex,
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::FlexStart,
                align_items: AlignItems::FlexStart,
                padding: UiRect::all(px(10)),
                column_gap: px(10),
                ..default()
            },
            BackgroundColor(Color::linear_rgb(0.0, 0.0, 1.0)),
        ))
        .with_children(|parent| {
            parent.spawn(gui_text(
                "yay",
                GuiTextProps {
                    size: GuiTextSize::Custom(5.),
                },
            ));
        })
        .id();
    commands.entity(gui_root.0).add_child(thing);
}
