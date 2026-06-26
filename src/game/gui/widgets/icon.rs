use bevy::prelude::*;

use crate::game::{core::resources::UiIconHandles, gui::resources::GuiThemeComputed};

#[allow(unused)]
pub enum GuiIconIcon {
    X,
    Minimize,
    CornerResizer,
}

#[allow(unused)]
pub struct GuiIconProps {}

impl Default for GuiIconProps {
    fn default() -> Self {
        Self {}
    }
}

#[derive(Component)]
struct GuiIconAttribs {
    icon: GuiIconIcon,
    width: f32,
    height: f32,
}

#[allow(unused)]
pub fn gui_icon(icon: GuiIconIcon, width: f32, height: f32, props: GuiIconProps) -> impl Bundle {
    (
        GuiIconAttribs {
            icon: icon,
            width: width,
            height: height,
        },
        Node::default(),
    )
}

fn set_style(
    commands: &mut Commands,
    theme: &GuiThemeComputed,
    icon_handles: &UiIconHandles,
    attribs: &GuiIconAttribs,
    entity: &Entity,
    node: &mut Node,
) {
    node.width = px(attribs.width);
    node.height = px(attribs.height);
    commands.entity(*entity).insert(ImageNode {
        image_mode: NodeImageMode::Stretch,
        color: theme.0.content_color_main,
        image: what_icon_handle(&icon_handles, &attribs.icon),
        ..default()
    });
}

fn what_icon_handle(icon_handles: &UiIconHandles, icon: &GuiIconIcon) -> Handle<Image> {
    match icon {
        GuiIconIcon::X => icon_handles.x.clone(),
        GuiIconIcon::Minimize => icon_handles.minimize.clone(),
        GuiIconIcon::CornerResizer => icon_handles.corner_resizer.clone(),
    }
}

pub struct GuiIconPlugin;

impl Plugin for GuiIconPlugin {
    fn build(&self, app: &mut App) {
        #[rustfmt::skip]
        app
            .add_systems(Update, update_style_on_attrib_change)
            .add_systems(Update,
                update_style_on_theme_change
                    .run_if(resource_changed::<GuiThemeComputed>)
            )
        ;
    }
}

fn update_style_on_attrib_change(
    mut commands: Commands,
    theme: Res<GuiThemeComputed>,
    icon_handles: Res<UiIconHandles>,
    mut entity_q: Query<
        (&GuiIconAttribs, Entity, &mut Node),
        Or<(Added<GuiIconAttribs>, Changed<GuiIconAttribs>)>,
    >,
) {
    entity_q.iter_mut().for_each(|(attribs, entity, mut node)| {
        set_style(
            &mut commands,
            &theme,
            &icon_handles,
            &attribs,
            &entity,
            &mut node,
        );
    });
}

fn update_style_on_theme_change(
    mut commands: Commands,
    theme: Res<GuiThemeComputed>,
    icon_handles: Res<UiIconHandles>,
    mut entity_q: Query<(&GuiIconAttribs, Entity, &mut Node)>,
) {
    entity_q.iter_mut().for_each(|(attribs, entity, mut node)| {
        set_style(
            &mut commands,
            &theme,
            &icon_handles,
            &attribs,
            &entity,
            &mut node,
        );
    });
}
