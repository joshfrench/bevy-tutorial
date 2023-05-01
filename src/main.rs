pub mod events;
mod game;
mod main_menu;
mod systems;

use game::*;
use main_menu::*;
use systems::*;

use bevy::prelude::*;

fn main() {
    App::new()
        // plugins
        .add_plugins(DefaultPlugins)
        .add_state::<AppState>()
        .add_plugin(MainMenuPlugin)
        .add_plugin(GamePlugin)
        // startup systems
        .add_startup_system(spawn_camera)
        // systems
        .add_system(transition_to_game)
        .add_system(transition_to_menu)
        .add_system(bevy::window::close_on_esc)
        .add_system(handle_game_over)
        .run();
}

#[derive(States, Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum AppState {
    #[default]
    MainMenu,
    Game,
    GameOver,
}
