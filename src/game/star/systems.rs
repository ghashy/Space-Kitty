use bevy::{prelude::*, window::PrimaryWindow};
use rand::prelude::*;

use super::{components::Star, resources::StarSpawnTimer, NUMBER_OF_STARS};

pub fn spawn_stars(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let window = window_query.get_single().unwrap();

    let mut rand = thread_rng();
    for _ in 0..NUMBER_OF_STARS {
        let rand_x = rand.gen::<f32>() * window.width();
        let rand_y = rand.gen::<f32>() * window.height();

        commands.spawn((
            SpriteBundle {
                transform: Transform::from_xyz(rand_x, rand_y, 0.),
                texture: asset_server.load("sprites/star.png"),
                ..default()
            },
            Star {},
        ));
    }
}

pub fn despawn_stars(
    mut commands: Commands,
    star_query: Query<Entity, With<Star>>,
) {
    for star in star_query.iter() {
        commands.entity(star).despawn();
    }
}

pub fn tick_star_spawn_timer(
    mut star_spawn_timer: ResMut<StarSpawnTimer>,
    time: Res<Time>,
) {
    star_spawn_timer.timer.tick(time.delta());
}

pub fn spawn_stars_over_time(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
    star_spawn_timer: Res<StarSpawnTimer>,
) {
    if star_spawn_timer.timer.finished() {
        let window = window_query.get_single().unwrap();
        let mut rand = thread_rng();
        let rand_x = rand.gen::<f32>() * window.width();
        let rand_y = rand.gen::<f32>() * window.height();

        commands.spawn((
            SpriteBundle {
                transform: Transform::from_xyz(rand_x, rand_y, 0.),
                texture: asset_server.load("sprites/star.png"),
                ..default()
            },
            Star {},
        ));
    }
}
