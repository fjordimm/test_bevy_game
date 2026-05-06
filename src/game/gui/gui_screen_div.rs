use bevy::prelude::*;

use crate::game::{
    gui::{GuiNode, constants::*, plugin::CollectionOfGuiItems},
    util::DummyOnCreation,
};

#[derive(Component)]
pub struct GuiScreenDivTag {
    pub is_active: bool,
}

pub struct GuiScreenDiv {
    starts_active: bool,
    color: Color,
    flex_direction: FlexDirection,
    children: Vec<Box<dyn GuiNode>>,
}

impl GuiScreenDiv {
    pub fn new<C: Into<CollectionOfGuiItems>>(
        starts_active: bool,
        color: Color,
        flex_direction: FlexDirection,
        children: C,
    ) -> Self {
        Self {
            starts_active: starts_active,
            color: color,
            flex_direction: flex_direction,
            children: children.into().0,
        }
    }
}

impl GuiNode for GuiScreenDiv {
    fn spawn(self, commands: &mut Commands, parent: Option<Entity>) -> Entity {
        let entity = commands
            .spawn((
                GuiScreenDivTag { is_active: true },
                Node {
                    position_type: PositionType::Absolute,
                    width: vw(100),
                    height: vh(100),
                    display: Display::Flex,
                    flex_direction: self.flex_direction,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    row_gap: px(MAIN_PADDING),
                    ..default()
                },
                BackgroundColor(self.color),
            ))
            .id();
        if let Some(par) = parent {
            commands.entity(par).add_child(entity);
        }

        for child in self.children {
            let child_entity = child.spawn_dyn(commands, None);
            commands.entity(entity).add_child(child_entity);
        }

        commands.add_observer(
            move |ent: On<DummyOnCreation>, mut query: Query<&mut GuiScreenDivTag>| {
                if let Ok(mut screen_div) = query.get_mut(ent.0) {
                    screen_div.is_active = self.starts_active;
                }
            },
        );
        commands.trigger(DummyOnCreation(entity));

        entity
    }

    fn spawn_dyn(self: Box<Self>, commands: &mut Commands, parent: Option<Entity>) -> Entity {
        self.spawn(commands, parent)
    }
}

pub fn update_is_active(
    mut screen_div_q: Query<(&GuiScreenDivTag, &mut Node), Changed<GuiScreenDivTag>>,
) {
    for (screen_div, mut screen_div_node) in &mut screen_div_q {
        screen_div_node.display = match screen_div.is_active {
            false => Display::None,
            true => Display::Flex,
        }
    }
}
