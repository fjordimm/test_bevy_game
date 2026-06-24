use bevy::{prelude::*, ui::widget::Text};

use crate::game::gui::resources::GuiTheme;

pub enum GuiTextSize {
    #[allow(unused)]
    Regular,
    #[allow(unused)]
    Medium,
    #[allow(unused)]
    Title,
    #[allow(unused)]
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
struct GuiTextAttribs {
    content: String,
    size: GuiTextSize,
}

pub fn gui_text(content: impl Into<String>, props: GuiTextProps) -> impl Bundle {
    (
        GuiTextAttribs {
            content: content.into(),
            size: props.size,
        },
        Text::default(),
        TextFont::default(),
        TextColor::default(),
    )
}

pub struct GuiTextPlugin;

impl Plugin for GuiTextPlugin {
    fn build(&self, app: &mut App) {
        #[rustfmt::skip]
        app
            .add_systems(Update,
                update_functional_components_on_attrib_change
            )
            .add_systems(Update,
                update_functional_components_on_theme_change
                    .run_if(resource_changed::<GuiTheme>)
            )
        ;
    }
}

fn update_functional_components_on_attrib_change(
    theme: Res<GuiTheme>,
    mut entity_q: Query<
        (&GuiTextAttribs, &mut Text, &mut TextFont, &mut TextColor),
        Or<(Added<GuiTextAttribs>, Changed<GuiTextAttribs>)>,
    >,
) {
    entity_q
        .iter_mut()
        .for_each(|(attribs, mut text, mut textfont, mut textcolor)| {
            update_functional_components(
                &theme,
                &attribs,
                &mut text,
                &mut textfont,
                &mut textcolor,
            );
        });
}

fn update_functional_components_on_theme_change(
    theme: Res<GuiTheme>,
    mut entity_q: Query<(&GuiTextAttribs, &mut Text, &mut TextFont, &mut TextColor)>,
) {
    entity_q
        .iter_mut()
        .for_each(|(attribs, mut text, mut textfont, mut textcolor)| {
            update_functional_components(
                &theme,
                &attribs,
                &mut text,
                &mut textfont,
                &mut textcolor,
            );
        });
}

fn update_functional_components(
    theme: &GuiTheme,
    attribs: &GuiTextAttribs,
    text: &mut Text,
    textfont: &mut TextFont,
    textcolor: &mut TextColor,
) {
    text.0 = attribs.content.clone(); // TODO: could be more efficient

    *textfont = TextFont {
        font: theme.font_main.clone(),
        font_size: match attribs.size {
            GuiTextSize::Regular => theme.font_size_regular,
            GuiTextSize::Medium => theme.font_size_medium,
            GuiTextSize::Title => theme.font_size_title,
            GuiTextSize::Custom(val) => val,
        },
        ..default()
    };

    textcolor.0 = theme.main_content_color;
}
