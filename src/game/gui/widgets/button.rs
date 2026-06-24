use bevy::prelude::*;

use crate::game::gui::resources::GuiThemeComputed;

#[allow(unused)]
pub struct GuiButtonProps {}

impl Default for GuiButtonProps {
    fn default() -> Self {
        Self {}
    }
}

#[derive(Component)]
struct GuiButtonAttribs {}

#[allow(unused)]
pub fn gui_button(props: GuiButtonProps) -> impl Bundle {
    (GuiButtonAttribs {}, Node::default(), Button)
}

pub struct GuiButtonPlugin;

impl Plugin for GuiButtonPlugin {
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
    mut commands: Commands,
    theme: Res<GuiThemeComputed>,
    mut entity_q: Query<
        (&GuiButtonAttribs, Entity, &mut Node),
        Or<(Added<GuiButtonAttribs>, Changed<GuiButtonAttribs>)>,
    >,
) {
    entity_q.iter_mut().for_each(|(attribs, entity, mut node)| {
        update_functional_components(&mut commands, &theme, &attribs, &entity, &mut node);
    });
}

fn update_functional_components_on_theme_change(
    mut commands: Commands,
    theme: Res<GuiThemeComputed>,
    mut entity_q: Query<(&GuiButtonAttribs, Entity, &mut Node)>,
) {
    entity_q.iter_mut().for_each(|(attribs, entity, mut node)| {
        update_functional_components(&mut commands, &theme, &attribs, &entity, &mut node);
    });
}

fn update_functional_components(
    commands: &mut Commands,
    theme: &GuiThemeComputed,
    attribs: &GuiButtonAttribs,
    entity: &Entity,
    node: &mut Node,
) {
    node.display = Display::Flex;
    node.width = Val::Auto;
    node.height = Val::Auto;
    node.flex_direction = FlexDirection::Column;
    node.justify_content = JustifyContent::Center;
    node.align_items = AlignItems::Center;
    node.padding = UiRect::all(px(theme.0.main_padding));
    node.row_gap = px(theme.0.main_padding);
    commands
        .entity(*entity)
        .insert(BackgroundColor(theme.0.main_bg_color))
        .insert(theme.0.main_box_shadow.clone());
}
