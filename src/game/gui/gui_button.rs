use bevy::prelude::*;

use crate::game::gui::{GuiNode, GuiText, constants::*, plugin::CollectionOfGuiItems};

pub struct GuiButton<E>
where
    E: Event,
    for<'a> E::Trigger<'a>: Default,
{
    event_supplier: Option<fn() -> E>,
    children: Vec<Box<dyn GuiNode>>,
}

impl<E> GuiButton<E>
where
    E: Event,
    for<'a> E::Trigger<'a>: Default,
{
    pub fn new<C: Into<CollectionOfGuiItems>>(
        event_supplier: Option<fn() -> E>,
        children: C,
    ) -> Self {
        Self {
            event_supplier: event_supplier,
            children: children.into().0,
        }
    }

    pub fn plain(event_supplier: Option<fn() -> E>, text: impl Into<String>) -> Self {
        Self {
            event_supplier: event_supplier,
            children: vec![Box::new(GuiText::regular(text))],
        }
    }
}

#[derive(Component)]
pub struct GuiButtonTag;

impl<E> GuiNode for GuiButton<E>
where
    E: Event,
    for<'a> E::Trigger<'a>: Default,
{
    fn spawn(&self, commands: &mut Commands) -> Entity {
        let entity = commands
            .spawn((
                GuiButtonTag,
                Button,
                Node {
                    display: Display::Flex,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    padding: UiRect::all(px(MAIN_PADDING)),
                    border_radius: BorderRadius::all(px(BORDER_RADIUS)),
                    ..default()
                },
                main_box_shadow(),
                BackgroundColor(BUTTON_COLOR_MAIN),
            ))
            .id();

        for child in &self.children {
            let child_entity = child.spawn(commands);
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
}

pub fn update_style(
    mut query: Query<
        (&Interaction, &mut BackgroundColor),
        (With<GuiButtonTag>, Changed<Interaction>),
    >,
) {
    for (interaction, mut color) in &mut query {
        *color = what_style(interaction);
    }
}

fn what_style(interaction: &Interaction) -> BackgroundColor {
    match interaction {
        Interaction::None => BackgroundColor(BUTTON_COLOR_MAIN),
        Interaction::Hovered => BackgroundColor(BUTTON_COLOR_HOVER),
        Interaction::Pressed => BackgroundColor(BUTTON_COLOR_PRESSED),
    }
}
