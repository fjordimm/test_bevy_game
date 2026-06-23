use bevy::{prelude::*, ui::widget::Text};

use crate::game::gui::resources::GuiTheme;

pub enum GuiTextSize {
    Regular,
    Medium,
    Title,
    Custom(f32),
}

pub struct GuiTextProps {
    pub size: GuiTextSize,
}

impl Default for GuiTextProps {
    fn default() -> Self {
        Self {
            size: GuiTextSize::Regular,
        }
    }
}

#[derive(Component)]
struct GuiTextTempAttribs {
    content: String,
    props: GuiTextProps,
}

pub fn gui_text(content: impl Into<String>, props: GuiTextProps) -> impl Bundle {
    GuiTextTempAttribs {
        content: content.into(),
        props: props,
    }
}

pub struct GuiTextPlugin;

impl Plugin for GuiTextPlugin {
    fn build(&self, app: &mut App) {
        #[rustfmt::skip]
        app
            .add_systems(Update, on_creation)
        ;
    }
}

fn on_creation(
    mut commands: Commands,
    theme: Res<GuiTheme>,
    temp_attribs_q: Query<(Entity, &GuiTextTempAttribs), Added<GuiTextTempAttribs>>,
) {
    temp_attribs_q.iter().for_each(|(entity, temp_attribs)| {
        commands.entity(entity).insert((
            Text::new(temp_attribs.content.clone()), // TODO: could be more efficient
            TextFont {
                font_size: match temp_attribs.props.size {
                    GuiTextSize::Regular => theme.font_size_regular,
                    GuiTextSize::Medium => theme.font_size_medium,
                    GuiTextSize::Title => theme.font_size_title,
                    GuiTextSize::Custom(val) => val,
                },
                ..default()
            },
        ));

        commands.entity(entity).remove::<GuiTextTempAttribs>();
    });
}
