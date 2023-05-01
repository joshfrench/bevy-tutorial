use bevy::prelude::*;

pub mod components;
mod systems;

use systems::*;

use crate::AppState;

use super::SimulationState;

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub struct MovementSet;

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub struct ConfinementSet;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.configure_set(MovementSet.before(ConfinementSet))
            .add_system(spawn_player.in_schedule(OnEnter(AppState::Game)))
            //
            /* .add_system(player_movement)
            .add_system(confine_player_movement.after(player_movement)) */
            //
            /* .add_systems((
                player_movement,
                // still need .before/.after, this just calls these out
                confine_player_movement.after(player_movement),
            )) */
            //
            // .chain() explicitly orders these
            //.add_systems((player_movement, confine_player_movement).chain())
            //
            .add_system(
                player_movement
                    .in_set(MovementSet)
                    .run_if(in_state(AppState::Game))
                    .run_if(in_state(SimulationState::Running)),
            )
            .add_system(
                confine_player_movement
                    .in_set(ConfinementSet)
                    .run_if(in_state(AppState::Game))
                    .run_if(in_state(SimulationState::Running)),
            )
            .add_systems(
                (enemy_hit_player, player_hit_star)
                    .in_set(OnUpdate(AppState::Game))
                    .in_set(OnUpdate(SimulationState::Running)),
            )
            .add_system(despawn_player.in_schedule(OnExit(AppState::Game)));
    }
}
