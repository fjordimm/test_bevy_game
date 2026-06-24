use bevy::{prelude::*, ui::widget::Text};

use crate::game::gui::resources::GuiThemeComputed;

#[allow(unused)]
pub enum GuiTextSize {
    Regular,
    Medium,
    Title,
    Custom(f32),
}

#[allow(unused)]
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

#[allow(unused)]
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
                    .run_if(resource_changed::<GuiThemeComputed>)
            )
        ;
    }
}

fn update_functional_components_on_attrib_change(
    theme: Res<GuiThemeComputed>,
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
    theme: Res<GuiThemeComputed>,
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
    theme: &GuiThemeComputed,
    attribs: &GuiTextAttribs,
    text: &mut Text,
    textfont: &mut TextFont,
    textcolor: &mut TextColor,
) {
    text.0 = attribs.content.clone(); // TODO: could be more efficient

    *textfont = TextFont {
        font: theme.0.font_main.clone(),
        font_size: match attribs.size {
            GuiTextSize::Regular => theme.0.font_size_regular,
            GuiTextSize::Medium => theme.0.font_size_medium,
            GuiTextSize::Title => theme.0.font_size_title,
            GuiTextSize::Custom(val) => val,
        },
        ..default()
    };

    textcolor.0 = theme.0.main_content_color;
}
