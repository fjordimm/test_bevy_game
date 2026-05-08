use bevy::{
    input::mouse::{MouseScrollUnit, MouseWheel},
    picking::hover::HoverMap,
    prelude::*,
};

use crate::game::{gui::constants::*, util::warned_ok};

#[derive(EntityEvent)]
#[entity_event(propagate, auto_propagate)]
pub struct GuiScroll {
    entity: Entity,
    delta: Vec2,
}

pub fn send_scroll_events(
    mut commands: Commands,
    mut mouse_wheel_reader: MessageReader<MouseWheel>,
    hover_map: Res<HoverMap>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    for mouse_wheel in mouse_wheel_reader.read() {
        let mut delta = -Vec2::new(mouse_wheel.x, mouse_wheel.y);

        if mouse_wheel.unit == MouseScrollUnit::Line {
            delta *= SCROLL_INTERVAL;
        }

        if keyboard_input.any_pressed([KeyCode::ShiftLeft, KeyCode::ShiftRight]) {
            std::mem::swap(&mut delta.x, &mut delta.y);
        }

        for pointer_map in hover_map.values() {
            for entity in pointer_map.keys().copied() {
                commands.trigger(GuiScroll { entity, delta });
            }
        }
    }
}

pub fn on_scroll_handler(
    mut scroll: On<GuiScroll>,
    mut query: Query<(&mut ScrollPosition, &Node, &ComputedNode)>,
) {
    if let Some((mut scroll_position, node, computed_node)) =
        warned_ok!(query.get_mut(scroll.entity))
    {
        let max_offset = (computed_node.content_size() - computed_node.size())
            * computed_node.inverse_scale_factor();

        let delta = &mut scroll.delta;

        if node.overflow.x == OverflowAxis::Scroll && delta.x != 0.0 {
            let max = if delta.x > 0.0 {
                scroll_position.x >= max_offset.x
            } else {
                scroll_position.x <= 0.0
            };

            if !max {
                scroll_position.x += delta.x;
                delta.x = 0.0;
            }
        }

        if node.overflow.y == OverflowAxis::Scroll && delta.y != 0.0 {
            let max = if delta.y > 0.0 {
                scroll_position.y >= max_offset.y
            } else {
                scroll_position.y <= 0.0
            };

            if !max {
                scroll_position.y += delta.y;
                delta.y = 0.0;
            }
        }

        if *delta == Vec2::ZERO {
            scroll.propagate(false);
        }
    }
}
