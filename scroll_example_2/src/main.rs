use bevy::prelude::*;

#[derive(Component)]
struct DialogueBox;

#[derive(Component)]
struct DialogueContent {
    full_text: String,        // Complete dialogue
    displayed_text: String,   // What's currently visible
    typing_started: f32,      // When this dialogue started typing
}

impl DialogueContent {
    fn new() -> Self {
        Self {
            full_text: String::new(),
            displayed_text: String::new(),
            typing_started: 0.0,
        }
    }
}

fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, (typewriter_effect, auto_scroll, add_dialogue))
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2d);

    let font_handle: Handle<Font> = asset_server.load("fonts/IntraNet.otf");

    // Dialogue box container
    commands
        .spawn((
            Node {
                width: px(400),
                height: px(300),
                flex_direction: FlexDirection::Column,
                padding: UiRect::all(px(10)),
                ..default()
            },
            BackgroundColor(Color::srgb(0.15, 0.15, 0.15)),
        ))
        .with_children(|parent| {
            // Scrollable content area
            parent
                .spawn((
                    Node {
                        flex_direction: FlexDirection::Column,
                        align_self: AlignSelf::Stretch,
                        overflow: Overflow::scroll_y(),
                        ..default()
                    },
                    BackgroundColor(Color::srgb(0.10, 0.10, 0.10)),
                    DialogueBox,
                ))
                .with_children(|parent| {
                    parent.spawn((
                        Text::new(""),
                        TextFont {
                            font: font_handle.clone(),
                            font_size: 16.,
                            ..default()
                        },
                        TextColor(Color::srgb(0.9, 0.9, 0.9)),
                        DialogueContent::new(),
                    ));
                });
        });
}

fn add_dialogue(
    mut dialogue_query: Query<&mut DialogueContent>,
    input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) {
    // Press SPACE to add a new dialogue line
    if input.just_pressed(KeyCode::Space) {
        if let Ok(mut content) = dialogue_query.single_mut() {
            let line_num = content.full_text.lines().count() + 1;
            content.full_text.push_str(&format!("Dialogue line {}\n", line_num));
            content.typing_started = time.elapsed_secs();
        }
    }
}

fn typewriter_effect(
    mut dialogue_query: Query<(&mut Text, &mut DialogueContent)>,
    time: Res<Time>,
) {
    if let Ok((mut text, mut content)) = dialogue_query.single_mut() {
        let chars_per_second = 20.0;
        let elapsed = time.elapsed_secs() - content.typing_started;
        let chars_to_show = (elapsed * chars_per_second).max(0.0) as usize;

        // Take only the first N characters from full_text
        content.displayed_text = content
            .full_text
            .chars()
            .take(chars_to_show)
            .collect();

        text.0 = content.displayed_text.clone();
    }
}

fn auto_scroll(
    mut scroll_query: Query<(&mut ScrollPosition, &ComputedNode), With<DialogueBox>>,
) {
    if let Ok((mut scroll_pos, computed)) = scroll_query.single_mut() {
        let max_offset = (computed.content_size() - computed.size())
            * computed.inverse_scale_factor();

        // Scroll to the bottom
        if max_offset.y > 0.0 {
            scroll_pos.y = max_offset.y;
        }
    }
}