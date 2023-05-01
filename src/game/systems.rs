use bevy::prelude::*;

use super::SimulationState;

pub fn toggle_simulation(
    mut commands: Commands,
    keys: Res<Input<KeyCode>>,
    state: Res<State<SimulationState>>,
) {
    if keys.just_pressed(KeyCode::Space) {
        if state.0 == SimulationState::Running {
            commands.insert_resource(NextState(Some(SimulationState::Paused)));
            println!("Paused")
        }
        if state.0 == SimulationState::Paused {
            commands.insert_resource(NextState(Some(SimulationState::Running)));
            println!("Unpaused")
        }
    }
}
