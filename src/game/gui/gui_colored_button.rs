use bevy::prelude::*;

use crate::game::gui::{GuiNode, constants::*, plugin::CollectionOfGuiItems};

pub struct GuiColoredButton<E>
where
    E: Event,
    for<'a> E::Trigger<'a>: Default,
{
    size: (i32, i32),
    colors: (Color, Color, Color),
    event_supplier: Option<fn() -> E>,
    children: Vec<Box<dyn GuiNode>>,
}

impl<E> GuiColoredButton<E>
where
    E: Event,
    for<'a> E::Trigger<'a>: Default,
{
    pub fn new<C: Into<CollectionOfGuiItems>>(
        size: (i32, i32),
        colors: (Color, Color, Color),
        event_supplier: fn() -> E,
        children: C,
    ) -> Self {
        Self {
            size: size,
            colors: colors,
            event_supplier: Some(event_supplier),
            children: children.into().0,
        }
    }
}

#[doc(hidden)]
#[derive(Event)]
pub struct _GuiColoredButtonDummyGeneric;

impl GuiColoredButton<_GuiColoredButtonDummyGeneric> {
    pub fn new_eventless<C: Into<CollectionOfGuiItems>>(
        size: (i32, i32),
        colors: (Color, Color, Color),
        children: C,
    ) -> Self {
        Self {
            size: size,
            colors: colors,
            event_supplier: None,
            children: children.into().0,
        }
    }
}

impl<E> GuiNode for GuiColoredButton<E>
where
    E: Event,
    for<'a> E::Trigger<'a>: Default,
{
    fn spawn(self, commands: &mut Commands, parent: Option<Entity>) -> Entity {
        let entity = commands
            .spawn((
                GuiColoredButtonTag {
                    colors: self.colors,
                },
                Button,
                Node {
                    border_radius: BorderRadius::all(px(BORDER_RADIUS)),
                    width: px(self.size.0),
                    height: px(self.size.1),
                    display: Display::Flex,
                    flex_direction: FlexDirection::Column,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    padding: UiRect::all(px(MAIN_PADDING)),
                    ..default()
                },
                main_box_shadow(),
                BackgroundColor(BUTTON_COLOR_MAIN),
            ))
            .id();
        if let Some(par) = parent {
            commands.entity(par).add_child(entity);
        }

        for child in self.children {
            let child_entity = child.spawn_dyn(commands, None);
            commands.entity(entity).add_child(child_entity);
        }

        if let Some(event_supplier_) = &self.event_supplier {
            let event_supplier = event_supplier_.clone();
            commands.entity(entity).observe(
                move |_: On<Pointer<Click>>, mut commands: Commands| {
                    commands.trigger(event_supplier());
                },
            );
        }

        entity
    }

    fn spawn_dyn(self: Box<Self>, commands: &mut Commands, parent: Option<Entity>) -> Entity {
        self.spawn(commands, parent)
    }
}

#[derive(Component)]
pub struct GuiColoredButtonTag {
    colors: (Color, Color, Color),
}

pub fn update(
    mut query: Query<
        (&Interaction, &GuiColoredButtonTag, &mut BackgroundColor),
        Changed<Interaction>,
    >,
) {
    for (interaction, tag, mut color) in &mut query {
        *color = what_style(interaction, tag.colors);
    }
}

fn what_style(interaction: &Interaction, colors: (Color, Color, Color)) -> BackgroundColor {
    match interaction {
        Interaction::None => BackgroundColor(colors.0),
        Interaction::Hovered => BackgroundColor(colors.1),
        Interaction::Pressed => BackgroundColor(colors.2),
    }
}
