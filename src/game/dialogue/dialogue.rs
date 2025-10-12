use bevy::prelude::*; 

pub struct DialoguePlugin;
impl Plugin for DialoguePlugin {
    fn build(&self, app: &mut App) {
        app
        .add_message::<DialogueAdvanced>()
        .add_systems(Startup, setup_dialogue)
        .add_systems(Update, (update_dialogue_text, handle_dialogue_input, advance_dialogue));
    }
}

#[derive(Message)]
struct DialogueAdvanced {
    entity: Entity,
}

#[derive(Component)]
struct DialogueText{
    full_text: String, 
    elapsed_time: f32,
    chars_per_second: f32, 
}

impl DialogueText{
    fn new(text: String, chars_per_second: f32) -> Self {
        Self {
            full_text: text, 
            elapsed_time: 0.0,
            chars_per_second, 
        }
    }

   fn is_complete(&self) -> bool {
        let visible = (self.elapsed_time * self.chars_per_second).floor() as usize;
        visible >= self.full_text.len()
    }

    fn show_all(&mut self) {
        self.elapsed_time = self.full_text.len() as f32 / self.chars_per_second;
    }

    fn get_visible_text(&self) -> String {
        let visible = (self.elapsed_time * self.chars_per_second).floor() as usize;
        self.full_text.chars().take(visible).collect()
    }

    fn set_text(&mut self, new_lines: String) {
        self.full_text = new_lines;
        self.elapsed_time = 0.0; 
    }
}

fn update_dialogue_text(
    mut query: Query<(Entity, &mut DialogueText)>,
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
            }
            else {
                events.write(DialogueAdvanced {entity}); 
            }
        }
    }
}

fn advance_dialogue(
    mut query: Query<&mut DialogueText>,
    mut writer: TextUiWriter,
    mut events: MessageReader<DialogueAdvanced>, 
) {
    for event in events.read() {
        if let Ok(mut dialogue) = query.get_mut(event.entity) { 
            dialogue.set_text("Now where the hell is that big ole Bison... Or was it a dinosaur??? ... Lets look at the map again...".to_string()); 
            *writer.text(event.entity, 0) = dialogue.get_visible_text();
        }
        
    }
}

fn setup_dialogue(mut commands: Commands) {
    
    let dialogue = "Where the hell are we? Is this really where the map said to go?! Thats unbelieveable.... Well off to find New Vegas!!!";
    
    commands.spawn((
        Text::new(dialogue),
        TextLayout::new_with_justify(Justify::Left),
        TextColor(Color::srgb(1.0, 1.0, 1.0)), 
        TextFont{
            font_size: 20.0,
            ..default()
        }, 
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(100.0),
            left: Val::Px(50.0),
            width: Val::Px(500.0),
            overflow: Overflow::clip(), 
            ..default()
        }, 
        DialogueText::new(
            dialogue.to_string(),
            12.5,
        ),
    ));
}