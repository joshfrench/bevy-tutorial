use bevy::{prelude::*, window::PrimaryWindow};
use rand::prelude::*;

use super::components::Star;
use super::resources::StarSpawnTimer;
use super::NUMBER_OF_STARS;

pub fn spawn_stars(
    mut commands: Commands,
    wq: Query<&Window, With<PrimaryWindow>>,
    assets: Res<AssetServer>,
) {
    let window = wq.get_single().unwrap();

    for _ in 0..NUMBER_OF_STARS {
        let rand_x = random::<f32>() * window.width();
        let rand_y = random::<f32>() * window.height();
        commands.spawn((
            SpriteBundle {
                transform: Transform::from_xyz(rand_x, rand_y, 0.),
                texture: assets.load("sprites/star.png"),
                ..default()
            },
            Star {},
        ));
    }
}

pub fn despawn_stars(mut commands: Commands, q: Query<Entity, With<Star>>) {
    for s in q.iter() {
        commands.entity(s).despawn();
    }
}

pub fn tick_star_spawn_timer(mut timer: ResMut<StarSpawnTimer>, time: Res<Time>) {
    timer.timer.tick(time.delta());
}

pub fn spawn_stars_over_time(
    mut commands: Commands,
    w: Query<&Window, With<PrimaryWindow>>,
    assets: Res<AssetServer>,
    timer: Res<StarSpawnTimer>,
) {
    if timer.timer.finished() {
        let window = w.get_single().unwrap();
        let rand_x = random::<f32>() * window.width();
        let rand_y = random::<f32>() * window.height();

        commands.spawn((
            SpriteBundle {
                texture: assets.load("sprites/star.png"),
                transform: Transform::from_xyz(rand_x, rand_y, 0.),
                ..default()
            },
            Star {},
        ));
    }
}
