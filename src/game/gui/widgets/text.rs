use bevy::{prelude::*, ui::widget::Text};

use crate::game::gui::{
    resources::GuiTheme,
    sets::{GuiWidgetDuringAddFunctionalComponents, GuiWidgetDuringUpdateFunctionalComponents},
};

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
    (GuiTextAttribs {
        content: content.into(),
        size: props.size,
    },)
}

pub struct GuiTextPlugin;

impl Plugin for GuiTextPlugin {
    fn build(&self, app: &mut App) {
        #[rustfmt::skip]
        app
            .add_systems(Update,
                add_functional_components
                    .in_set(GuiWidgetDuringAddFunctionalComponents)
            )
            .add_systems(Update,
                update_functional_components_on_attrib_change
                    .in_set(GuiWidgetDuringUpdateFunctionalComponents)
            )
            .add_systems(Update,
                update_functional_components_on_theme_change
                    .in_set(GuiWidgetDuringUpdateFunctionalComponents)
                    .run_if(resource_changed::<GuiTheme>)
            )
        ;
    }
}

fn add_functional_components(
    mut commands: Commands,
    entity_q: Query<Entity, Added<GuiTextAttribs>>,
) {
    entity_q.iter().for_each(|entity| {
        commands
            .entity(entity)
            .insert((Text::default(), TextFont::default()));
    });
}

fn update_functional_components_on_attrib_change(
    theme: Res<GuiTheme>,
    mut entity_q: Query<
        (&GuiTextAttribs, &mut Text, &mut TextFont),
        Or<(Added<GuiTextAttribs>, Changed<GuiTextAttribs>)>,
    >,
) {
    entity_q
        .iter_mut()
        .for_each(|(attribs, mut text, mut textfont)| {
            update_functional_components(&theme, &attribs, &mut text, &mut textfont);
        });
}

fn update_functional_components_on_theme_change(
    theme: Res<GuiTheme>,
    mut entity_q: Query<(&GuiTextAttribs, &mut Text, &mut TextFont)>,
) {
    entity_q
        .iter_mut()
        .for_each(|(attribs, mut text, mut textfont)| {
            update_functional_components(&theme, &attribs, &mut text, &mut textfont);
        });
}

fn update_functional_components(
    theme: &GuiTheme,
    attribs: &GuiTextAttribs,
    text: &mut Text,
    textfont: &mut TextFont,
) {
    text.0 = attribs.content.clone(); // TODO: could be more efficient
    *textfont = TextFont {
        font_size: what_font_size(&theme, &attribs),
        font: theme.font_main.clone(),
        ..default()
    };
}

fn what_font_size(theme: &GuiTheme, attribs: &GuiTextAttribs) -> f32 {
    match attribs.size {
        GuiTextSize::Regular => theme.font_size_regular,
        GuiTextSize::Medium => theme.font_size_medium,
        GuiTextSize::Title => theme.font_size_title,
        GuiTextSize::Custom(val) => val,
    }
}
