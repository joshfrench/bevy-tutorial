use bevy::{prelude::*, window::PrimaryWindow};
use rand::prelude::*;

use super::components::*;
use super::resources::*;
use super::{ENEMY_SIZE, ENEMY_SPEED, NUMBER_OF_ENEMIES};

pub fn spawn_enemies(
    mut commands: Commands,
    wq: Query<&Window, With<PrimaryWindow>>,
    assets: Res<AssetServer>,
) {
    let window = wq.get_single().unwrap();

    for _ in 0..NUMBER_OF_ENEMIES {
        let rand_x = random::<f32>() * window.width();
        let rand_y = random::<f32>() * window.height();

        commands.spawn((
            SpriteBundle {
                transform: Transform::from_xyz(rand_x, rand_y, 0.),
                texture: assets.load("sprites/ball_red_large.png"),
                ..default()
            },
            Enemy {
                direction: Vec2::new(random::<f32>(), random::<f32>()).normalize(),
            },
        ));
    }
}

pub fn despawn_enemies(mut commands: Commands, q: Query<Entity, With<Enemy>>) {
    for e in q.iter() {
        commands.entity(e).despawn();
    }
}

pub fn enemy_movement(mut q: Query<(&mut Transform, &Enemy)>, time: Res<Time>) {
    for (mut transform, enemy) in q.iter_mut() {
        let direction = Vec3::new(enemy.direction.x, enemy.direction.y, 0.);
        transform.translation += direction * ENEMY_SPEED * time.delta_seconds();
    }
}

pub fn update_enemy_direction(
    mut q: Query<(&Transform, &mut Enemy)>,
    w: Query<&Window, With<PrimaryWindow>>,
    audio: Res<Audio>,
    assets: Res<AssetServer>,
) {
    let window = w.get_single().unwrap();

    let half_enemy_size = ENEMY_SIZE / 2.;
    let x_min = 0. + half_enemy_size;
    let x_max = window.width() - half_enemy_size;
    let y_min = 0. + half_enemy_size;
    let y_max = window.height() - half_enemy_size;

    for (transform, mut enemy) in q.iter_mut() {
        let mut direction_changed = false;
        let translation = transform.translation;
        if translation.x < x_min || translation.x > x_max {
            enemy.direction.x *= -1.;
            direction_changed = true;
        }
        if translation.y < y_min || translation.y > y_max {
            enemy.direction.y *= -1.;
            direction_changed = true;
        }
        if direction_changed {
            let sfx1 = assets.load("audio/impactGlass_heavy_000.ogg");
            let sfx2 = assets.load("audio/impactGlass_heavy_004.ogg");
            let sfx = if random::<f32>() > 0.5 { sfx1 } else { sfx2 };
            audio.play(sfx);
        }
    }
}

pub fn confine_enemy_movement(
    mut eq: Query<&mut Transform, With<Enemy>>,
    wq: Query<&Window, With<PrimaryWindow>>,
) {
    let window = wq.get_single().unwrap();
    let half_enemy_size = ENEMY_SIZE / 2.;
    let x_min = 0. + half_enemy_size;
    let x_max = window.width() - half_enemy_size;
    let y_min = 0. + half_enemy_size;
    let y_max = window.height() - half_enemy_size;

    for mut xform in eq.iter_mut() {
        let mut translation = xform.translation;
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

        xform.translation = translation;
    }
}

pub fn tick_enemy_spawn_timer(mut timer: ResMut<EnemySpawnTimer>, time: Res<Time>) {
    timer.0.tick(time.delta());
}

pub fn spawn_enemies_over_time(
    mut commands: Commands,
    q: Query<&Window, With<PrimaryWindow>>,
    assets: Res<AssetServer>,
    timer: Res<EnemySpawnTimer>,
) {
    if timer.0.finished() {
        let window = q.get_single().unwrap();
        let rand_x = random::<f32>() * window.width();
        let rand_y = random::<f32>() * window.height();

        commands.spawn((
            SpriteBundle {
                texture: assets.load("sprites/ball_red_large.png"),
                transform: Transform::from_xyz(rand_x, rand_y, 0.),
                ..default()
            },
            Enemy {
                direction: Vec2::new(random::<f32>(), random::<f32>()).normalize(),
            },
        ));
    }
}
