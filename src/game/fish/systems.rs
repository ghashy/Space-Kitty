use bevy::{prelude::*, window::PrimaryWindow};
use bevy_rapier2d::prelude::*;
use bevy_tweening::{lens::*, *};
use rand::prelude::*;

// ───── Current Crate Imports ────────────────────────────────────────────── //

use crate::audio::assets::AudioSource;
use crate::audio::resources::{KiraManager, SamplePack};
use crate::game::{enemy::components::Enemy, player::components::Player};

use super::{
    components::*, resources::FishSpawnTimer, FISH_SIZE, NUMBER_OF_FISH,
};

// ───── Body ─────────────────────────────────────────────────────────────── //

pub fn spawn_fish(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let window = window_query.get_single().unwrap();

    // Entity for storing stars
    let fish = SpatialBundle::default();
    let mut children_fish = vec![];

    let mut rand = thread_rng();
    for _ in 0..NUMBER_OF_FISH {
        let rand_x = rand.gen::<f32>() * window.width();
        let rand_y = rand.gen::<f32>() * window.height();

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
    mut kira_manager: NonSendMut<KiraManager>,
    audio_assets: Res<Assets<AudioSource>>,
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

                            let handle =
                                get_random_pick_fish_sample(&sample_pack);
                            // Play audio
                            kira_manager
                                .play(audio_assets.get(handle).unwrap().get())
                                .unwrap()
                                .set_volume(0.3, kira::tween::Tween::default())
                                .unwrap();

                            commands.entity(fish_entity).despawn();
                            picked_event.send(FishWasPickedEvent(
                                name.to_string(),
                                entity,
                            ));
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
                            // Play audio
                            kira_manager
                                .play(
                                    audio_assets
                                        .get(&sample_pack.pick_fish1)
                                        .unwrap()
                                        .get(),
                                )
                                .unwrap();
                            commands.entity(fish_entity).despawn();
                            picked_event.send(FishWasPickedEvent(
                                name.to_string(),
                                entity,
                            ));
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
    window_query: Query<&Window, With<PrimaryWindow>>,
    stars_pack_query: Query<Entity, With<FishPack>>,
    asset_server: Res<AssetServer>,
    star_spawn_timer: Res<FishSpawnTimer>,
) {
    if star_spawn_timer.timer.finished() {
        let window = window_query.get_single().unwrap();
        let mut rand = thread_rng();
        let rand_x = rand.gen::<f32>() * window.width();
        let rand_y = rand.gen::<f32>() * window.height();

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

fn get_random_pick_fish_sample<'a>(
    sample_pack: &'a Res<SamplePack>,
) -> &'a Handle<AudioSource> {
    match rand::thread_rng().gen_range(0..21) {
        0 => &sample_pack.pick_fish1,
        1 => &sample_pack.pick_fish2,
        2 => &sample_pack.pick_fish3,
        3 => &sample_pack.pick_fish4,
        4 => &sample_pack.pick_fish5,
        5 => &sample_pack.pick_fish6,
        6 => &sample_pack.pick_fish7,
        7 => &sample_pack.pick_fish8,
        8 => &sample_pack.pick_fish9,
        9 => &sample_pack.pick_fish10,
        10 => &sample_pack.pick_fish11,
        11 => &sample_pack.pick_fish12,
        12 => &sample_pack.pick_fish13,
        13 => &sample_pack.pick_fish14,
        14 => &sample_pack.pick_fish15,
        15 => &sample_pack.pick_fish16,
        16 => &sample_pack.pick_fish17,
        17 => &sample_pack.pick_fish18,
        18 => &sample_pack.pick_fish19,
        19 => &sample_pack.pick_fish20,
        20 => &sample_pack.pick_fish21,
        _ => unreachable!(),
    }
}
