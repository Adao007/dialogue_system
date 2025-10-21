use super::state::*;
use bevy::prelude::*;

#[derive(Component)]
pub struct DialogueRoot;

#[derive(Component)]
pub struct DialogueBox {
    pub auto_scroll: bool,
}

#[derive(Component)]
pub struct DialogueText {
    pub full_text: String,
    pub elapsed_time: f32,
    pub output_speed: f32, // Chars per second
}

#[derive(Component)]
pub struct Title;

impl DialogueText {
    pub fn new(speed: f32) -> Self {
        Self {
            full_text: String::new(),
            elapsed_time: 0.0,
            output_speed: speed,
        }
    }

    pub fn is_complete(&self) -> bool {
        let visible = (self.elapsed_time * self.output_speed).floor() as usize;
        visible >= self.full_text.len()
    }

    pub fn show_all(&mut self) {
        self.elapsed_time = self.full_text.len() as f32 / self.output_speed;
    }

    pub fn get_visible(&self) -> String {
        let visible = (self.elapsed_time * self.output_speed).floor() as usize;
        self.full_text.chars().take(visible).collect()
    }

    pub fn set_text(&mut self, new_lines: String) {
        self.full_text = new_lines;
        self.elapsed_time = 0.0;
    }
}

pub fn setup_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font_handle: Handle<Font> = asset_server.load("fonts/ztn.otf");
    let speed: f32 = 12.5;

    commands
        .spawn((
            Node {
                position_type: PositionType::Absolute,
                flex_direction: FlexDirection::Column,
                top: percent(68),
                left: percent(20),
                width: percent(60),
                height: percent(25),
                ..default()
            },
            BackgroundColor(Color::srgba(0.10, 0.10, 0.10, 0.9)),
            DialogueRoot,
            Visibility::Visible,
        ))
        .with_children(|parent| {
            // Speaker Ui
            parent
                .spawn(Node {
                    padding: UiRect {
                        left: Val::Px(10.0),
                        top: Val::Px(10.0),
                        right: Val::Px(10.0),
                        bottom: Val::Px(5.0),
                    },
                    width: Val::Percent(100.0),
                    justify_content: JustifyContent::Start,
                    ..default()
                })
                .with_children(|speaker_parent| {
                    speaker_parent.spawn((
                        Text::new(""),
                        TextFont {
                            font: font_handle.clone(),
                            font_size: 40.0,
                            ..default()
                        },
                        TextColor(Color::srgb(1.0, 1.0, 1.0)),
                        Title,
                    ));
                });
            // Scrollable Component
            parent
                .spawn((
                    Node {
                        flex_direction: FlexDirection::Column,
                        overflow: Overflow::scroll_y(),
                        flex_grow:1.0,
                        padding: UiRect::all(Val::Px(10.0)),
                        ..default()
                    },
                    DialogueBox { auto_scroll: true },
                ))
                // Text Component
                .with_children(|text_parent| {
                    text_parent.spawn((
                        Text::new(""),
                        TextFont {
                            font: font_handle.clone(),
                            font_size: 60.,
                            ..default()
                        },
                        TextColor(Color::srgb(0.9, 0.9, 0.9)),
                        DialogueText::new(speed),
                    ));
                });
        });
}

pub fn set_visibility(
    active: Res<ActiveDialogue>,
    mut query: Query<&mut Visibility, With<DialogueRoot>>,
) {
    for mut visibility in query.iter_mut() {
        *visibility = if active.state == DialogueState::Response {
            Visibility::Hidden
        } else {
            Visibility::Visible
        };
    }
}

