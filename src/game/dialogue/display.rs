use bevy::prelude::*;
use super::scroll::ScrollPlugin; 

pub struct DisplayPlugin;
impl Plugin for DisplayPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(ScrollPlugin)
            .add_message::<DialogueAdvanced>()
            .add_systems(Startup, setup_dialogue)
            .add_systems(
                Update,
                (
                    update_dialogue_text,
                    handle_dialogue_input,
                    advance_dialogue,
                ),
            );
    }
}

#[derive(Message)]
struct DialogueAdvanced {
    entity: Entity,
}

#[derive(Component)]
pub struct DialogueBox {
    pub auto_scroll: bool, 
}

#[derive(Component)]
pub struct DialogueText {
    full_text: String,
    elapsed_time: f32,
    output_speed: f32, // Chars per second
}

impl DialogueText {
    fn new(speed: f32) -> Self {
        Self {
            full_text: String::new(),
            elapsed_time: 0.0,
            output_speed: speed, 
        }
        
    }

    fn is_complete(&self) -> bool {
        let visible = (self.elapsed_time * self.output_speed).floor() as usize;
        visible >= self.full_text.len()
    }

    fn show_all(&mut self) {
        self.elapsed_time = self.full_text.len() as f32 / self.output_speed;
    }

    fn get_visible_text(&self) -> String {
        let visible = (self.elapsed_time * self.output_speed).floor() as usize;
        self.full_text.chars().take(visible).collect()
    }

    fn set_text(&mut self, new_lines: String) {
        self.full_text = new_lines;
        self.elapsed_time = 0.0;
    }
}

fn update_dialogue_text(
    mut query: Query<(Entity, &mut DialogueText), With<Text>>,
    mut writer: TextUiWriter,
    time: Res<Time>,
) {
    for (entity, mut dialogue) in query.iter_mut() {
        if dialogue.is_complete() {
            continue;
        }

        dialogue.elapsed_time += time.delta_secs();
        *writer.text(entity, 0) = dialogue.get_visible_text();
    }
}

fn handle_dialogue_input(
    mut query: Query<(Entity, &mut DialogueText)>,
    mut writer: TextUiWriter,
    mut events: MessageWriter<DialogueAdvanced>,
    input: Res<ButtonInput<KeyCode>>,
) {
    if input.just_pressed(KeyCode::Space) {
        for (entity, mut dialogue) in query.iter_mut() {
            if !dialogue.is_complete() {
                dialogue.show_all();
                *writer.text(entity, 0) = dialogue.get_visible_text();
            } else {
                events.write(DialogueAdvanced { entity });
            }
        }
    }
}

fn advance_dialogue(
    mut query: Query<&mut DialogueText>,
    mut writer: TextUiWriter,
    mut events: MessageReader<DialogueAdvanced>,
    dialogue_box_query: Single<&mut DialogueBox>, 
) {
    let mut dialogue_box = dialogue_box_query.into_inner(); 
    for event in events.read() {
        if let Ok(mut dialogue) = query.get_mut(event.entity) {
            dialogue.set_text("Where  the  hell  are  we,  Jupi?!  This,  without  a  doubt,  is  not  New  Vegas!  Its unbelieveable... If I don't see a slot machine soon, imma go CRAZY!!! Now where the hell is that big ole Bison... Or was it a dinosaur??? ... Lets look at the map again... WELL WHAT IN THE GODDAMN?! THIS IS BARSTOW?!?!? THIS IS THE WRONG WAY!!! NOOOOOOOOOOOOOOOO!!!!!!! MY SLOT MACHINES!!!!!!!".to_string());
            *writer.text(event.entity, 0) = dialogue.get_visible_text();
            dialogue_box.auto_scroll = true;
        }
    }
}

fn setup_dialogue(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let font_handle: Handle<Font> = asset_server.load("fonts/ztn.otf"); 
    let speed: f32 = 12.5; 

    commands
        .spawn((
            Node {
                position_type: PositionType::Absolute,
                top: percent(80),
                left: percent(20),
                width: percent(60),
                height: percent(13),
                ..default()
            },
            BackgroundColor(Color::srgba(0.10, 0.10, 0.10, 0.9)),
        ))
        // Scrollable Component
        .with_children(|parent| {
            parent
                .spawn((
                    Node {
                        flex_direction: FlexDirection::Column,
                        align_self: AlignSelf::Stretch,
                        overflow: Overflow::scroll_y(),
                        ..default()
                    },
                    DialogueBox { auto_scroll: true },
                ))
                // Text Component
                .with_children(|parent| {
                    parent.spawn((
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

