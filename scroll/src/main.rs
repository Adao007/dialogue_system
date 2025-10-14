use accesskit::{Node as Accessible, Role};
use bevy::a11y::AccessibilityNode;
//use bevy::ecs::spawn::SpawnIter;
use bevy::input::mouse::{MouseScrollUnit, MouseWheel};
use bevy::picking::hover::HoverMap;
use bevy::prelude::*;

fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, send_scroll_events)
        .add_observer(on_scroll_handler)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2d);

    let font_handle: Handle<Font> = asset_server.load("fonts/IntraNet.otf");

    commands
        .spawn(Node {
            width: percent(100),
            height: percent(100),
            justify_content: JustifyContent::SpaceBetween,
            flex_direction: FlexDirection::Row,
            ..default()
        })
        .with_children(|parent| {
            parent.spawn((
                Node {
                    width: percent(100),
                    height: percent(100),
                    flex_direction: FlexDirection::Row,
                    justify_content: JustifyContent::SpaceBetween,
                    ..default()
                },
                children![vertically_scrolling_list(
                    asset_server.load("fonts/IntraNet.otf")
                ),],
            ));
        });
}

fn vertically_scrolling_list(font_handle: Handle<Font>) -> impl Bundle {
    (
        Node {
            flex_direction: FlexDirection::Column,
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            width: px(200),
            ..default()
        },
        children![
            (
                Text::new("Vertically Scrolling List"),
                TextFont {
                    font: font_handle.clone(),
                    font_size: 20.,
                    ..default()
                },
                Label,
            ),
            (
                // Scrolling List
                Node {
                    flex_direction: FlexDirection::Column,
                    align_self: AlignSelf::Stretch,
                    height: percent(75),
                    overflow: Overflow::scroll_y(), // n.b
                    ..default()
                },
                BackgroundColor(Color::srgb(0.10, 0.10, 0.10)),
                Children::spawn(SpawnIter((0..100).map(move |i| {
                    (
                        Node {
                            min_height: px(21.),
                            max_height: px(21.),
                            ..default()
                        },
                        children![(
                            Text(format!("Item {i}")),
                            TextFont {
                                font: font_handle.clone(),
                                ..default()
                            },
                            Label,
                            AccessibilityNode(Accessible::new(Role::ListItem)),
                        )],
                    )
                })))
            ),
        ],
    )
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
            delta *= 21.;
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
    mut query: Query<(&mut ScrollPosition, &Node, &ComputedNode)>,
) {
    let Ok((mut scroll_position, node, computed)) = query.get_mut(scroll.entity) else {
        return;
    };

    let max_offset = (computed.content_size() - computed.size()) * computed.inverse_scale_factor();

    let delta = &mut scroll.delta;

    if node.overflow.x == OverflowAxis::Scroll && delta.x != 0.0 {
        let max = if delta.x > 0. {
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
