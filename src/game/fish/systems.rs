use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use bevy_tweening::{lens::*, *};
use interpolation::EaseFunction;
use rand::prelude::*;

// ───── Current Crate Imports ────────────────────────────────────────────── //

use crate::audio_system::resources::SamplePack;
use crate::game::{enemy::components::Enemy, player::components::Player};
use crate::{WORLD_MAX_EDGE, WORLD_MIN_EDGE};

use super::{
    components::*, resources::FishSpawnTimer, FISH_SIZE, NUMBER_OF_FISH,
};

// ───── Body ─────────────────────────────────────────────────────────────── //

pub fn spawn_fish(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Entity for storing stars
    let fish = SpatialBundle::default();
    let mut children_fish = vec![];

    let mut rand = thread_rng();
    for _ in 0..NUMBER_OF_FISH {
        let rand_x = rand.gen_range(WORLD_MIN_EDGE..WORLD_MAX_EDGE);
        let rand_y = rand.gen_range(WORLD_MIN_EDGE..WORLD_MAX_EDGE);

        children_fish.push(
            commands
                .spawn((
                    SpriteBundle {
                        sprite: Sprite {
                            custom_size: Some(FISH_SIZE),
                            ..default()
                        },
                        transform: Transform::from_xyz(rand_x, rand_y, 1.)
                            .with_rotation(Quat::from_rotation_z(
                                rand.gen_range(0.0..std::f32::consts::PI * 2.),
                            )),
                        texture: asset_server.load("sprites/Fish.png"),
                        ..default()
                    },
                    Collider::ball(FISH_SIZE.x / 2.),
                    Sensor::default(),
                    Fish {},
                    Animator::new(get_fish_tween(Vec3::new(
                        rand_x, rand_y, 1.,
                    ))),
                ))
                .id(),
        );
    }

    // Save all stars in stars entity
    commands
        .spawn(fish)
        .insert(FishPack)
        .insert(Name::new("Fish_pack"))
        .push_children(&children_fish);
}

pub fn despawn_fish(
    mut commands: Commands,
    star_pack_query: Query<Entity, With<FishPack>>,
) {
    commands
        .entity(star_pack_query.single())
        .despawn_recursive();
}

pub fn tick_fish_spawn_timer(
    mut star_spawn_timer: ResMut<FishSpawnTimer>,
    time: Res<Time>,
) {
    star_spawn_timer.timer.tick(time.delta());
}

pub fn check_collision(
    mut collision_events: EventReader<CollisionEvent>,
    mut commands: Commands,
    entity_query: Query<(Entity, &Name), Or<(With<Player>, With<Enemy>)>>,
    mut fish_query: Query<(Entity, &Parent), With<Fish>>,
    audio: Res<Audio>,
    sample_pack: Res<SamplePack>,
    mut picked_event: EventWriter<FishWasPickedEvent>,
) {
    'outer: for event in collision_events.iter() {
        if let CollisionEvent::Started(entity1, entity2, _) = event {
            for (fish_entity, fish_pack) in fish_query.iter_mut() {
                if fish_entity == *entity1 {
                    for (entity, name) in entity_query.iter() {
                        if entity == *entity2 {
                            commands
                                .entity(fish_pack.get())
                                .remove_children(&[fish_entity]);
                            audio.play(sample_pack.pick_star.clone_weak());
                            commands.entity(fish_entity).despawn();
                            picked_event
                                .send(FishWasPickedEvent(name.to_string()));
                            // Continue cycle if collision is resolved
                            continue 'outer;
                        }
                    }
                } else if fish_entity == *entity2 {
                    for (entity, name) in entity_query.iter() {
                        if entity == *entity1 {
                            commands
                                .entity(fish_pack.get())
                                .remove_children(&[fish_entity]);
                            audio.play(sample_pack.pick_star.clone_weak());
                            commands.entity(fish_entity).despawn();
                            picked_event
                                .send(FishWasPickedEvent(name.to_string()));
                            // Continue cycle if collision is resolved
                            continue 'outer;
                        }
                    }
                }
            }
        }
    }
}

pub fn spawn_fish_over_time(
    mut commands: Commands,
    stars_pack_query: Query<Entity, With<FishPack>>,
    asset_server: Res<AssetServer>,
    star_spawn_timer: Res<FishSpawnTimer>,
) {
    if star_spawn_timer.timer.finished() {
        let mut rand = thread_rng();
        let rand_x = rand.gen_range(WORLD_MIN_EDGE..WORLD_MAX_EDGE);
        let rand_y = rand.gen_range(WORLD_MIN_EDGE..WORLD_MAX_EDGE);

        let stars_pack = stars_pack_query.single();
        let child = commands
            .spawn((
                SpriteBundle {
                    sprite: Sprite {
                        custom_size: Some(FISH_SIZE),
                        ..default()
                    },
                    transform: Transform::from_xyz(rand_x, rand_y, 1.)
                        .with_rotation(Quat::from_rotation_z(
                            rand.gen_range(0.0..std::f32::consts::PI * 2.),
                        )),
                    texture: asset_server.load("sprites/Fish.png"),
                    ..default()
                },
                Collider::ball(FISH_SIZE.x / 2.),
                Sensor::default(),
                Fish {},
                Animator::new(get_fish_tween(Vec3::new(rand_x, rand_y, 1.))),
            ))
            .id();
        commands.entity(stars_pack).add_child(child);
    }
}

fn get_fish_tween(start: Vec3) -> Tween<Transform> {
    let mut rand = rand::thread_rng();
    let rand_x = rand.gen_range(3.0..15.0);
    let rand_y = rand.gen_range(3.0..15.0);
    let rand_time = rand.gen_range(2000..3500);
    let tween = Tween::new(
        EaseFunction::QuadraticInOut,
        std::time::Duration::from_millis(rand_time),
        TransformPositionLens {
            start,
            end: start + Vec3::new(rand_x, rand_y, 0.),
        },
    )
    .with_repeat_count(RepeatCount::Infinite)
    .with_repeat_strategy(RepeatStrategy::MirroredRepeat);
    tween
}
