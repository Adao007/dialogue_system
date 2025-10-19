use super::{
    data::DialogueData,
    input::advance,
    state::{ActiveDialogue, DialogueState},
    ui::{DialogueBox, DialogueRoot, DialogueText},
};
use bevy::prelude::*;

#[derive(Component)]
pub struct ResponseUi;

#[derive(Component)]
pub struct ResponseButton {
    pub response_index: usize,
}

pub fn setup_response(
    mut commands: Commands,
    active: Res<ActiveDialogue>,
    dialogue_query: Query<&DialogueData>,
    existing_choices: Query<Entity, With<ResponseUi>>,
    asset_server: Res<AssetServer>,
) {
    if active.state != DialogueState::Response {
        return;
    }

    if !existing_choices.is_empty() {
        return;
    }

    let Ok(data) = dialogue_query.get(active.source) else {
        return;
    };
    let current_node = &data.nodes[&active.node_id];
    let font: Handle<Font> = asset_server.load("fonts/ztn.otf");

    commands
        .spawn((
            Node {
                position_type: PositionType::Absolute,
                flex_direction: FlexDirection::Column,
                row_gap: Val::Px(10.0),
                top: Val::Percent(30.0),
                left: Val::Percent(20.0),
                width: Val::Percent(60.0),
                padding: UiRect::all(Val::Px(20.0)),
                ..default()
            },
            BackgroundColor(Color::srgba(0.1, 0.1, 0.1, 0.9)),
            ResponseUi,
            Visibility::Visible,
        ))
        .with_children(|parent| {
            for (index, choice) in current_node.choices.iter().enumerate() {
                parent
                    .spawn((
                        Button,
                        Node {
                            width: Val::Percent(100.0),
                            padding: UiRect::all(Val::Px(15.0)),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        BackgroundColor(Color::srgb(0.2, 0.2, 0.2)),
                        ResponseButton {
                            response_index: index,
                        },
                    ))
                    .with_children(|parent| {
                        parent.spawn((
                            Text::new(format!("{}. {}", index + 1, &choice.text)),
                            TextFont {
                                font: font.clone(),
                                font_size: 40.0,
                                ..default()
                            },
                            TextColor(Color::srgb(0.9, 0.9, 0.9)),
                        ));
                    });
            }
        });
}

pub fn highlight_responses(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<ResponseButton>),
    >,
) {
    for (interaction, mut background) in interaction_query.iter_mut() {
        match *interaction {
            Interaction::Hovered => {
                *background = BackgroundColor(Color::srgb(0.4, 0.4, 0.5));
            }
            Interaction::None => {
                *background = BackgroundColor(Color::srgb(0.2, 0.2, 0.2));
            }
            Interaction::Pressed => {
                // Handled by "handle_selection" system
                *background = BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.2));
            }
        }
    }
}

pub fn handle_selection(
    mut commands: Commands,
    mut active: ResMut<ActiveDialogue>,
    mut text_query: Query<(Entity, &mut DialogueText)>,
    mut box_query: Query<&mut DialogueBox, With<DialogueRoot>>,
    data_query: Query<&DialogueData>,
    button_query: Query<(&Interaction, &ResponseButton), Changed<Interaction>>,
    response_ui_query: Query<Entity, With<ResponseUi>>,
) {
    if active.state != DialogueState::Response {
        return;
    }

    for (interaction, response_button) in button_query.iter() {
        if *interaction == Interaction::Pressed {
            let Ok(data) = data_query.get(active.source) else {
                return;
            };
            let current_node = &data.nodes[&active.node_id];
            let selected = &current_node.choices[response_button.response_index];

            for entity in response_ui_query.iter() {
                commands.entity(entity).despawn();
            }

            advance(
                &mut active,
                &mut text_query,
                &mut box_query,
                data,
                selected.next,
            );
            break;
        }
    }
}

pub fn clean_response_ui(
    mut commands: Commands,
    active: Res<ActiveDialogue>,
    response_ui_query: Query<Entity, With<ResponseUi>>,
) {
    if active.state != DialogueState::Response && !response_ui_query.is_empty() {
        for entity in response_ui_query.iter() {
            commands.entity(entity).despawn();
        }
    }
}
