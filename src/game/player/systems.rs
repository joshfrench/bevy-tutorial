use bevy::{prelude::*, window::PrimaryWindow};

pub const PLAYER_SIZE: f32 = 64.;
pub const PLAYER_SPEED: f32 = 750.;

use super::components::Player;
use crate::enemy::components::Enemy;
use crate::enemy::ENEMY_SIZE;
use crate::events::GameOver;
use crate::score::resources::*;
use crate::star::components::Star;
use crate::star::STAR_SIZE;

pub fn spawn_player(
    mut commands: Commands,
    q: Query<&Window, With<PrimaryWindow>>,
    assets: Res<AssetServer>,
) {
    let window = q.get_single().unwrap();

    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(window.width() / 2., window.height() / 2., 0.),
            texture: assets.load("sprites/ball_blue_large.png"),
            ..default()
        },
        Player {},
    ));
}

pub fn despawn_player(mut commands: Commands, q: Query<Entity, With<Player>>) {
    for p in q.iter() {
        commands.entity(p).despawn();
    }
}

pub fn player_movement(
    keyboard_input: Res<Input<KeyCode>>,
    mut q: Query<&mut Transform, With<Player>>,
    time: Res<Time>,
) {
    if let Ok(mut transform) = q.get_single_mut() {
        let mut direction = Vec3::ZERO;

        if keyboard_input.pressed(KeyCode::Left) || keyboard_input.pressed(KeyCode::A) {
            direction += Vec3::new(-1., 0., 0.);
        }
        if keyboard_input.pressed(KeyCode::Right) || keyboard_input.pressed(KeyCode::D) {
            direction += Vec3::new(1., 0., 0.);
        }
        if keyboard_input.pressed(KeyCode::Up) || keyboard_input.pressed(KeyCode::W) {
            direction += Vec3::new(0., 1., 0.);
        }
        if keyboard_input.pressed(KeyCode::Down) || keyboard_input.pressed(KeyCode::S) {
            direction += Vec3::new(0., -1., 0.);
        }

        if direction.length() > 0.0 {
            direction = direction.normalize();
        }

        transform.translation += direction * PLAYER_SPEED * time.delta_seconds();
    }
}

pub fn confine_player_movement(
    mut pq: Query<&mut Transform, With<Player>>,
    wq: Query<&Window, With<PrimaryWindow>>,
) {
    if let Ok(mut p_xform) = pq.get_single_mut() {
        let window = wq.get_single().unwrap();
        let half_player_size = PLAYER_SIZE / 2.;
        let x_min = 0. + half_player_size;
        let x_max = window.width() - half_player_size;
        let y_min = 0. + half_player_size;
        let y_max = window.height() - half_player_size;

        let mut translation = p_xform.translation;

        if translation.x < x_min {
            translation.x = x_min;
        } else if translation.x > x_max {
            translation.x = x_max;
        }

        if translation.y < y_min {
            translation.y = y_min;
        } else if translation.y > y_max {
            translation.y = y_max;
        }

        p_xform.translation = translation;
    }
}

pub fn player_hit_star(
    mut commands: Commands,
    pq: Query<&Transform, With<Player>>,
    sq: Query<(Entity, &Transform), With<Star>>,
    audio: Res<Audio>,
    assets: Res<AssetServer>,
    mut score: ResMut<Score>,
) {
    if let Ok(p_xform) = pq.get_single() {
        for (s, s_xform) in sq.iter() {
            let d = p_xform.translation.distance(s_xform.translation);
            let p_radius = PLAYER_SIZE / 2.;
            let s_radius = STAR_SIZE / 2.;
            if d < p_radius + s_radius {
                let sfx = assets.load("audio/laserLarge_000.ogg");
                audio.play(sfx);
                commands.entity(s).despawn();
                score.value += 1;
            }
        }
    }
}

pub fn enemy_hit_player(
    mut commands: Commands,
    mut game_over_writer: EventWriter<GameOver>,
    pq: Query<(Entity, &Transform), With<Player>>,
    eq: Query<&Transform, With<Enemy>>,
    audio: Res<Audio>,
    assets: Res<AssetServer>,
    score: Res<Score>,
) {
    if let Ok((player, p_xform)) = pq.get_single() {
        for e_xform in eq.iter() {
            let d = p_xform.translation.distance(e_xform.translation);
            let p_radius = PLAYER_SIZE / 2.;
            let e_radius = ENEMY_SIZE / 2.;
            if d < p_radius + e_radius {
                let sfx = assets.load("audio/explosionCrunch_000.ogg");
                audio.play(sfx);
                commands.entity(player).despawn();
                game_over_writer.send(GameOver { score: score.value });
            }
        }
    }
}
