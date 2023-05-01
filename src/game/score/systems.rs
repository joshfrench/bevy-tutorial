use bevy::prelude::*;

use super::resources::*;
use crate::events::GameOver;

pub fn insert_score(mut commands: Commands) {
    commands.insert_resource(Score::default())
}

pub fn remove_score(mut commands: Commands) {
    commands.remove_resource::<Score>()
}

pub fn update_score(score: Res<Score>) {
    if score.is_changed() {
        println!("Score: {}", score.value);
    }
}

pub fn handle_high_scores(
    mut game_over_reader: EventReader<GameOver>,
    mut high_scores: ResMut<HighScores>,
) {
    for e in game_over_reader.iter() {
        high_scores.scores.push(("Player".to_string(), e.score))
    }
}

pub fn high_scores_updated(high_scores: Res<HighScores>) {
    if high_scores.is_changed() {
        println!("{:?}", high_scores);
    }
}
