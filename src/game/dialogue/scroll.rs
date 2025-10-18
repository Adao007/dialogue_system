use bevy::input::mouse::{MouseScrollUnit, MouseWheel}; 
use bevy::picking::hover::HoverMap; 
use bevy::prelude::*;
use super::ui::{ DialogueBox, DialogueText }; 

pub struct ScrollPlugin; 
impl Plugin for ScrollPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, (send_scroll_events, auto_scroll))
            .add_observer(on_scroll_handler); 
    }
}

#[derive(EntityEvent, Debug)]
#[entity_event(propagate, auto_propagate)]
struct Scroll {
    entity: Entity, 
    delta: Vec2, 
}

fn send_scroll_events(
    mut mouse_wheel_reader: MessageReader<MouseWheel>, 
    hover_map: Res<HoverMap>, 
    mut commands: Commands,
) {
    for mouse_wheel in mouse_wheel_reader.read() {
        let mut delta = -Vec2::new(mouse_wheel.x, mouse_wheel.y); 
        if mouse_wheel.unit == MouseScrollUnit::Line {
            delta *= 21. 
        }

        for pointer_map in hover_map.values() {
            for entity in pointer_map.keys().copied() {
                commands.trigger(Scroll { entity, delta }); 
            }
        }
    }
}

fn on_scroll_handler(
    mut scroll: On<Scroll>,
    mut query: Query<(&mut ScrollPosition, &Node, &ComputedNode, Option<&mut DialogueBox>)>, 
) {
    let Ok((mut scroll_position, node, computed, dialogue_box)) = query.get_mut(scroll.entity) else {
        return; 
    }; 

    let max_offset = (computed.content_size() - computed.size()) * computed.inverse_scale_factor(); 
    let delta = &mut scroll.delta; 
    let mut did_scroll = false; 

    if node.overflow.x == OverflowAxis::Scroll && delta.x != 0.0 {
        let max = if delta.x > 0.0 {
            scroll_position.x >= max_offset.x
        } else {
            scroll_position.x <= 0.0
        }; 

        if !max {
            scroll_position.x += delta.x; 
            delta.x = 0.0;
            did_scroll = true; 
        }
    }

    if node.overflow.y == OverflowAxis::Scroll && delta.y != 0.0 {
        let max = if delta.y > 0.0 {
            scroll_position.y >= max_offset.y
        } else {
            scroll_position.y <= 0.0
        }; 

        if !max{
            scroll_position.y += delta.y; 
            delta.y = 0.0; 
            did_scroll = true; 
        }
    }

    // This disables auto-scrolling if the user manually scrolls text. 
    if did_scroll {
        if let Some(mut dialogue_box) = dialogue_box {
            dialogue_box.auto_scroll = false; 
        }
    }

    if *delta == Vec2::ZERO {
        scroll.propagate(false);
    }
}

fn auto_scroll(
    dialogue_query: Query<&DialogueText, Changed<DialogueText>>,
    mut scroll_query: Query<(&mut ScrollPosition, &ComputedNode, &DialogueBox), With<DialogueBox>>
) {
    if dialogue_query.is_empty() {
        return; 
    }

    if let Ok((mut scroll_pos, computed, dialogue_box)) = scroll_query.single_mut() {
        if !dialogue_box.auto_scroll {
            return; 
        }

        let max_offset = (computed.content_size() - computed.size())
            * computed.inverse_scale_factor(); 

        if max_offset.y > 0.0 {
            scroll_pos.y = max_offset.y;
        }
    }
}