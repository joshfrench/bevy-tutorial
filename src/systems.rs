use bevy::{prelude::*, window::PrimaryWindow};

use crate::{events::*, game::SimulationState, AppState};

pub fn spawn_camera(mut commands: Commands, q: Query<&Window, With<PrimaryWindow>>) {
    let window = q.get_single().unwrap();

    commands.spawn(Camera2dBundle {
        transform: Transform::from_xyz(window.width() / 2., window.height() / 2., 0.),
        ..default()
    });
}

pub fn handle_game_over(mut commands: Commands, mut game_over_reader: EventReader<GameOver>) {
    for e in game_over_reader.iter() {
        println!("Final score: {}", e.score);
        commands.insert_resource(NextState(Some(AppState::GameOver)))
    }
}

pub fn transition_to_game(
    mut commands: Commands,
    keys: Res<Input<KeyCode>>,
    state: Res<State<AppState>>,
) {
    if keys.just_pressed(KeyCode::Return) {
        if state.0 != AppState::Game {
            commands.insert_resource(NextState(Some(AppState::Game)));
        }
    }
}

pub fn transition_to_menu(
    mut commands: Commands,
    keys: Res<Input<KeyCode>>,
    state: Res<State<AppState>>,
) {
    if keys.just_pressed(KeyCode::Q) {
        if state.0 != AppState::MainMenu {
            commands.insert_resource(NextState(Some(AppState::MainMenu)));
            commands.insert_resource(NextState(Some(SimulationState::Paused)));
        }
    }
}
