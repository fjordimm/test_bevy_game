use bevy::{prelude::*, ui::widget::Text};

use crate::game::gui::resources::GuiThemeComputed;

#[allow(unused)]
pub enum GuiTextSize {
    Custom(f32),
    P,
    H1,
    H2,
}

#[allow(unused)]
pub struct GuiTextProps {
    pub size: GuiTextSize,
}

impl Default for GuiTextProps {
    fn default() -> Self {
        Self {
            size: GuiTextSize::P,
        }
    }
}

#[derive(Component)]
struct GuiTextAttribs {
    size: GuiTextSize,
}

#[derive(Component)]
struct GuiTextState {
    content: String,
}

#[allow(unused)]
pub fn gui_text(content: impl Into<String>, props: GuiTextProps) -> impl Bundle {
    (
        GuiTextAttribs { size: props.size },
        GuiTextState {
            content: content.into(),
        },
        Text::default(),
        TextFont::default(),
        TextColor::default(),
    )
}

#[allow(unused)]
pub fn gui_text_p(content: impl Into<String>) -> impl Bundle {
    gui_text(
        content,
        GuiTextProps {
            size: GuiTextSize::P,
        },
    )
}

#[allow(unused)]
pub fn gui_text_h1(content: impl Into<String>) -> impl Bundle {
    gui_text(
        content,
        GuiTextProps {
            size: GuiTextSize::H1,
        },
    )
}

#[allow(unused)]
pub fn gui_text_h2(content: impl Into<String>) -> impl Bundle {
    gui_text(
        content,
        GuiTextProps {
            size: GuiTextSize::H2,
        },
    )
}

fn apply_style(
    theme: &GuiThemeComputed,
    attribs: &GuiTextAttribs,
    state: &GuiTextState,
    text: &mut Text,
    textfont: &mut TextFont,
    textcolor: &mut TextColor,
) {
    text.0 = state.content.clone(); // TODO: could be more efficient

    *textfont = TextFont {
        font: theme.0.font_main.clone(),
        font_size: match attribs.size {
            GuiTextSize::Custom(val) => val,
            GuiTextSize::P => theme.0.font_size_p,
            GuiTextSize::H1 => theme.0.font_size_h1,
            GuiTextSize::H2 => theme.0.font_size_h2,
        },
        ..default()
    };

    textcolor.0 = theme.0.content_color_main;
}

pub struct GuiTextPlugin;

impl Plugin for GuiTextPlugin {
    fn build(&self, app: &mut App) {
        #[rustfmt::skip]
        app
            .add_systems(Update, update_style_on_attrib_change)
            .add_systems(Update,
                update_style_on_theme_change
                    .run_if(resource_changed::<GuiThemeComputed>)
            )
            .add_observer(set_text_observer)
        ;
    }
}

#[derive(EntityEvent)]
pub struct SetText {
    entity: Entity,
    text: String,
}

fn set_text_observer(event: On<SetText>) {}

fn update_style_on_attrib_change(
    theme: Res<GuiThemeComputed>,
    mut entity_q: Query<
        (
            &GuiTextAttribs,
            &GuiTextState,
            &mut Text,
            &mut TextFont,
            &mut TextColor,
        ),
        Or<(Added<GuiTextAttribs>, Changed<GuiTextAttribs>)>,
    >,
) {
    entity_q
        .iter_mut()
        .for_each(|(attribs, state, mut text, mut textfont, mut textcolor)| {
            apply_style(
                &theme,
                &attribs,
                &state,
                &mut text,
                &mut textfont,
                &mut textcolor,
            );
        });
}

fn update_style_on_theme_change(
    theme: Res<GuiThemeComputed>,
    mut entity_q: Query<(
        &GuiTextAttribs,
        &GuiTextState,
        &mut Text,
        &mut TextFont,
        &mut TextColor,
    )>,
) {
    entity_q
        .iter_mut()
        .for_each(|(attribs, state, mut text, mut textfont, mut textcolor)| {
            apply_style(
                &theme,
                &attribs,
                &state,
                &mut text,
                &mut textfont,
                &mut textcolor,
            );
        });
}
