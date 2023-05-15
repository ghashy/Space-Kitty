use bevy::{prelude::*, window::PrimaryWindow};
use rand::prelude::*;

// ----- Crate -------------------------------------------------------------- //

use crate::{
    audio_system::resources::SamplePack, game::score::resources::Score,
    helper_functions::*,
};

// ----- Module ------------------------------------------------------------- //

use super::{components::Enemy, *};

// ----- Body --------------------------------------------------------------- //

pub fn spawn_enemies(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let window = window_query.get_single().unwrap();

    let mut rand = thread_rng();
    for _ in 0..NUMBER_OF_ENEMIES {
        let random_x = rand.gen::<f32>() * window.width();
        let random_y = rand.gen::<f32>() * window.height();

        commands.spawn((
            SpriteBundle {
                transform: Transform::from_xyz(random_x, random_y, 0.),
                texture: asset_server.load("sprites/ball_red_large.png"),
                ..default()
            },
            Enemy {
                direction: Vec2::new(rand.gen::<f32>(), rand.gen::<f32>())
                    .normalize(),
            },
        ));
    }
}

pub fn despawn_enemies(
    mut commands: Commands,
    enemy_query: Query<Entity, With<Enemy>>,
) {
    for enemy in enemy_query.iter() {
        commands.entity(enemy).despawn();
    }
}

pub fn enemy_movement(
    mut enemy_query: Query<(&mut Transform, &Enemy)>,
    time: Res<Time>,
) {
    for (mut transform, enemy) in enemy_query.iter_mut() {
        let direction = enemy.direction.extend(0.);
        transform.translation += direction * ENEMY_SPEED * time.delta_seconds();
    }
}

pub fn update_enemy_direction(
    mut enemy_query: Query<(&Transform, &mut Enemy)>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    audio: Res<Audio>,
    sample_pack: Res<SamplePack>,
) {
    let window = window_query.get_single().unwrap();

    let (x_min, x_max, y_min, y_max) = get_window_borders(window);

    for (transform, mut enemy) in enemy_query.iter_mut() {
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
            // Randomly play one of the two sound effects
            let sound_effect = if random::<f32>() > 0.5 {
                &sample_pack.pluck1
            } else {
                &sample_pack.pluck2
            };

            audio.play(sound_effect.clone());
        }
    }
}

pub fn confine_enemy_movement(
    mut enemy_query: Query<&mut Transform, With<Enemy>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window_query.get_single().unwrap();
    let (x_min, x_max, y_min, y_max) = get_window_borders(window);

    for mut enemy_transform in enemy_query.iter_mut() {
        let translation = clamp_translation(
            enemy_transform.translation,
            x_min,
            x_max,
            y_min,
            y_max,
        );

        enemy_transform.translation = translation;
    }
}

pub fn spawn_enemy_on_game_progress(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
    mut score: ResMut<Score>,
) {
    if score.value % 5 == 0 && score.value > score.last_value {
        let window = window_query.get_single().unwrap();
        let mut rand = thread_rng();
        let rand_x = rand.gen::<f32>() * window.width();
        let rand_y = rand.gen::<f32>() * window.height();

        commands.spawn((
            SpriteBundle {
                transform: Transform::from_xyz(rand_x, rand_y, 0.),
                texture: asset_server.load("sprites/ball_red_large.png"),
                ..default()
            },
            Enemy {
                direction: Vec2::new(rand.gen::<f32>(), rand.gen::<f32>())
                    .normalize(),
            },
        ));

        score.last_value += 1;
    }
}
