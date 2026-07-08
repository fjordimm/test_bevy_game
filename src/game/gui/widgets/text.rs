use bevy::{prelude::*, ui::widget::Text};
use bevy_ecs::query::QueryData;

use crate::game::gui::{resources::GuiThemeComputed, sets::GuiSystemsOrdering};

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
    pub wraps: bool,
}

impl Default for GuiTextProps {
    fn default() -> Self {
        Self {
            size: GuiTextSize::P,
            wraps: true,
        }
    }
}

#[derive(Component)]
struct GuiTextAttribs {
    size: GuiTextSize,
    wraps: bool,
}

#[derive(Component)]
struct GuiTextState {
    content: String,
}

#[allow(unused)]
pub fn gui_text(content: impl Into<String>, props: GuiTextProps) -> impl Bundle {
    (
        GuiTextAttribs {
            size: props.size,
            wraps: props.wraps,
        },
        GuiTextState {
            content: content.into(),
        },
        Text::default(),
        TextFont::default(),
        TextColor::default(),
        TextLayout::default(),
    )
}

#[allow(unused)]
pub fn gui_text_p(content: impl Into<String>) -> impl Bundle {
    gui_text(
        content,
        GuiTextProps {
            size: GuiTextSize::P,
            ..default()
        },
    )
}

#[allow(unused)]
pub fn gui_text_h1(content: impl Into<String>) -> impl Bundle {
    gui_text(
        content,
        GuiTextProps {
            size: GuiTextSize::H1,
            ..default()
        },
    )
}

#[allow(unused)]
pub fn gui_text_h2(content: impl Into<String>) -> impl Bundle {
    gui_text(
        content,
        GuiTextProps {
            size: GuiTextSize::H2,
            ..default()
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
    textlayout: &mut TextLayout,
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

    *textlayout = match attribs.wraps {
        true => TextLayout::new(Justify::default(), LineBreak::WordBoundary),
        false => TextLayout::new(Justify::default(), LineBreak::NoWrap),
    };
}

fn modify_style_from_state(
    _theme: &GuiThemeComputed,
    _attribs: &GuiTextAttribs,
    state: &GuiTextState,
    text: &mut Text,
) {
    text.0 = state.content.clone(); // TODO: could be more efficient
}

pub struct GuiTextPlugin;

impl Plugin for GuiTextPlugin {
    fn build(&self, app: &mut App) {
        #[rustfmt::skip]
        app
            .add_systems(Update,
                update_style_on_init_or_attrib_change
                    .in_set(GuiSystemsOrdering::UpdateStyle)
            )
            .add_systems(Update,
                update_style_on_theme_change
                    .run_if(resource_changed::<GuiThemeComputed>)
                    .in_set(GuiSystemsOrdering::UpdateStyle)
            )
            .add_systems(Update,
                update_style_on_state_change
                    .in_set(GuiSystemsOrdering::UpdateStyle)
            )
        ;
    }
}

fn update_style_on_init_or_attrib_change(
    theme: Res<GuiThemeComputed>,
    mut entity_q: Query<
        (
            &GuiTextAttribs,
            &GuiTextState,
            &mut Text,
            &mut TextFont,
            &mut TextColor,
            &mut TextLayout,
        ),
        Or<(Added<GuiTextAttribs>, Changed<GuiTextAttribs>)>,
    >,
) {
    entity_q.iter_mut().for_each(
        |(attribs, state, mut text, mut textfont, mut textcolor, mut textlayout)| {
            apply_style(
                &theme,
                &attribs,
                &state,
                &mut text,
                &mut textfont,
                &mut textcolor,
                &mut textlayout,
            );
        },
    );
}

fn update_style_on_theme_change(
    theme: Res<GuiThemeComputed>,
    mut entity_q: Query<(
        &GuiTextAttribs,
        &GuiTextState,
        &mut Text,
        &mut TextFont,
        &mut TextColor,
        &mut TextLayout,
    )>,
) {
    entity_q.iter_mut().for_each(
        |(attribs, state, mut text, mut textfont, mut textcolor, mut textlayout)| {
            apply_style(
                &theme,
                &attribs,
                &state,
                &mut text,
                &mut textfont,
                &mut textcolor,
                &mut textlayout,
            );
        },
    );
}

fn update_style_on_state_change(
    theme: Res<GuiThemeComputed>,
    mut entity_q: Query<(&GuiTextAttribs, &GuiTextState, &mut Text), Changed<GuiTextState>>,
) {
    entity_q.iter_mut().for_each(|(attribs, state, mut text)| {
        modify_style_from_state(&theme, &attribs, &state, &mut text);
    });
}

#[derive(QueryData)]
#[query_data(mutable)]
pub struct GuiTextInterface {
    state: &'static mut GuiTextState,
}

#[allow(unused)]
impl<'w, 's> GuiTextInterfaceItem<'w, 's> {
    pub fn content(&self) -> &String {
        &self.state.content
    }

    pub fn set_content(&mut self, val: String) {
        self.state.content = val;
    }
}
