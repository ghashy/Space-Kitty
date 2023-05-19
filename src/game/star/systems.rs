use bevy::{prelude::*, window::PrimaryWindow};
use rand::prelude::*;

use super::{components::*, resources::StarSpawnTimer, NUMBER_OF_STARS};

pub fn spawn_stars(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let window = window_query.get_single().unwrap();

    // Entity for storing stars
    let stars = SpatialBundle::default();
    let mut children_stars = vec![];

    let mut rand = thread_rng();
    for _ in 0..NUMBER_OF_STARS {
        let rand_x = rand.gen::<f32>() * window.width() * 2.;
        let rand_y = rand.gen::<f32>() * window.height() * 2.;

        children_stars.push(
            commands
                .spawn((
                    SpriteBundle {
                        transform: Transform::from_xyz(rand_x, rand_y, 0.),
                        texture: asset_server.load("sprites/star.png"),
                        ..default()
                    },
                    Star {},
                ))
                .id(),
        );
    }

    // Save all stars in stars entity
    commands
        .spawn(stars)
        .insert(StarsPack)
        .insert(Name::new("Stars_pack"))
        .push_children(&children_stars);
}

pub fn despawn_stars(
    mut commands: Commands,
    star_pack_query: Query<Entity, With<StarsPack>>,
) {
    commands
        .entity(star_pack_query.single())
        .despawn_recursive();
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
    stars_pack_query: Query<Entity, With<StarsPack>>,
    asset_server: Res<AssetServer>,
    star_spawn_timer: Res<StarSpawnTimer>,
) {
    if star_spawn_timer.timer.finished() {
        let window = window_query.get_single().unwrap();
        let mut rand = thread_rng();
        let rand_x = rand.gen::<f32>() * window.width() * 2.;
        let rand_y = rand.gen::<f32>() * window.height() * 2.;

        let stars_pack = stars_pack_query.single();
        let child = commands
            .spawn((
                SpriteBundle {
                    transform: Transform::from_xyz(rand_x, rand_y, 0.),
                    texture: asset_server.load("sprites/star.png"),
                    ..default()
                },
                Star {},
            ))
            .id();
        commands.entity(stars_pack).add_child(child);
    }
}
