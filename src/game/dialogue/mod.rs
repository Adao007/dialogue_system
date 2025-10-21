use bevy::prelude::*;
pub mod choices;
pub mod data;
pub mod input;
pub mod loader;
pub mod output;
pub mod scroll;
pub mod sprites;
pub mod state;
pub mod ui;

pub struct DialoguePlugin;
impl Plugin for DialoguePlugin {
    fn build(&self, app: &mut App) {
        app.add_message::<state::SpriteSpawner>()
            .add_systems(Startup, (loader::load_dialogue, ui::setup_ui))
            .add_systems(
                Update,
                (
                    state::active_dialogue,
                    (
                        output::output_text,
                        output::output_speaker,
                        output::output_sprites,
                        input::handle_input,
                        ui::set_visibility,
                        scroll::send_scroll_events,
                        scroll::auto_scroll,
                        choices::setup_response,
                        choices::highlight_responses,
                        choices::handle_selection,
                        choices::clean_response_ui,
                        state::end_dialogue,
                    )
                        .run_if(resource_exists::<state::ActiveDialogue>),
                ),
            )
            .add_observer(scroll::on_scroll_handler);
    }
}
