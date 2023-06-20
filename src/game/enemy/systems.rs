use bevy::{prelude::*, sprite::Anchor, utils::HashSet, window::PrimaryWindow};
use bevy_rapier2d::prelude::*;
use rand::prelude::*;
use std::ops::Range;

// ───── Current Crate Imports ────────────────────────────────────────────── //

use super::{
    components::{Enemy, EnemyIsArrivingEvent, PatchOfLight},
    *,
};
use crate::game::{player::DOG_SIZE, score::resources::Score};
use crate::helper_functions::*;
use crate::{
    audio_system::resources::SamplePack,
    game::fish::components::FishWasPickedEvent,
};

// ───── Body ─────────────────────────────────────────────────────────────── //

pub fn load_resources(mut commands: Commands, asset_server: Res<AssetServer>) {
    let names: Handle<DogNames> =
        asset_server.load("json_data/dogs_names.json");

    let mut images: Vec<(String, Handle<Image>)> = Vec::new();

    for i in 1..12 {
        let name = format!("Face{}", i);
        images.push((
            name.clone(),
            asset_server.load(format!("sprites/Dogs/{}.png", name)),
        ));
    }
    let name = String::from("FaceHarry");
    images.push((
        name.clone(),
        asset_server.load(format!("sprites/Dogs/{}.png", name)),
    ));

    images.shuffle(&mut rand::thread_rng());

    commands.insert_resource(DogResource {
        json_data: names,
        images,
    })
}

pub fn despawn_enemies(
    mut commands: Commands,
    enemy_query: Query<Entity, With<Enemy>>,
) {
    for enemy in enemy_query.iter() {
        commands.entity(enemy).despawn_recursive();
    }
}

pub fn enemy_movement(
    mut enemy_query: Query<(Option<&mut Velocity>, &Enemy)>,
    time: Res<Time>,
) {
    for (velocity, enemy) in enemy_query.iter_mut() {
        let direction = enemy.direction.extend(0.);
        if let Some(mut velocity) = velocity {
            velocity.linvel =
                direction.truncate() * ENEMY_SPEED * time.delta_seconds();
        }
    }
}

pub fn update_enemy_direction(
    mut enemy_query: Query<(Entity, &mut Enemy), With<Enemy>>,
    mut collision_event: EventReader<CollisionEvent>,
    rapier_context: Res<RapierContext>,
    audio: Res<Audio>,
    sample_pack: Res<SamplePack>,
) {
    let mut direction_changed = false;

    // 0b00 - empty
    // 0b01 - entity1
    // 0b10 - entity2
    // 0b11 - both
    let mut entities_flags = 0b00;

    for event in collision_event.iter() {
        if let CollisionEvent::Started(entity1, entity2, _) = event {
            // For playing audio

            // If we found a collision
            if let Some(contact_pair) =
                rapier_context.contact_pair(*entity1, *entity2)
            {
                // Is collided entity1 actually enemy?
                if let Some(mut enemy) =
                    enemy_query.iter_mut().find_map(|enemy| {
                        if enemy.0 == *entity1 {
                            Some(enemy.1)
                        } else {
                            None
                        }
                    })
                {
                    direction_changed = true;
                    enemy
                        .direction
                        .reflect(contact_pair.manifold(0).unwrap().normal());
                    // Write entity1 to flags
                    entities_flags = entities_flags | 0b01;
                }

                // Is collided entity2 actually enemy?
                if let Some(mut enemy) =
                    enemy_query.iter_mut().find_map(|enemy| {
                        if enemy.0 == *entity2 {
                            Some(enemy.1)
                        } else {
                            None
                        }
                    })
                {
                    direction_changed = true;
                    enemy
                        .direction
                        .reflect(contact_pair.manifold(0).unwrap().normal());
                    // Write entity2 to flags
                    entities_flags = entities_flags | 0b10;
                }
            }
            // TODO: profile this break disabled
            // Break if got actual collision
            if entities_flags != 0b00 {
                continue;
            }
        }
    }
    // Randomly play one of sound effects
    let sound_effect: &Handle<AudioSource>;
    let mut rng = thread_rng();
    if direction_changed {
        if entities_flags != 0b11 {
            sound_effect = match rng.gen::<bool>() {
                true => &sample_pack.imp_med_0,
                false => &sample_pack.imp_med_1,
            }
        } else {
            sound_effect = match rng.gen_range::<u16, Range<u16>>(0..5) {
                0 => &sample_pack.imp_light_0,
                1 => &sample_pack.imp_light_1,
                2 => &sample_pack.imp_light_2,
                3 => &sample_pack.imp_light_3,
                4 => &sample_pack.imp_light_4,
                _ => &sample_pack.exp,
            }
        }

        audio.play(sound_effect.clone());
    }
}

pub fn spawn_enemy_on_game_progress(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
    mut dogs_resource: ResMut<DogResource>,
    names_assets: Res<Assets<DogNames>>,
    score: ResMut<Score>,
    mut already_spawned_data: Local<(HashSet<String>, u8)>,
    mut picked_event: EventReader<FishWasPickedEvent>,
    mut arriving_event: EventWriter<EnemyIsArrivingEvent>,
) {
    let name = picked_event
        .iter()
        .map(|event| event.0.clone())
        .find(|name| name == "Kitty");

    // If we didn't found any Kitty in these events return
    if name == None {
        return;
    }

    let name = name.unwrap();

    // If there are no score for such name
    if let Err(_) = score.get_score(&name) {
        return;
    }

    let score = score.get_score(&name).unwrap();

    if score % 5 == 0 {
        let window = window_query.get_single().unwrap();
        let mut rand = thread_rng();
        let random_x = rand.gen::<f32>() * window.width();
        let random_y = rand.gen::<f32>() * window.height();

        let (name, texture, scale_modifier) = generate_dog(
            &mut dogs_resource,
            names_assets,
            &mut already_spawned_data,
        );

        let entity = commands
            .spawn((
                SpriteBundle {
                    sprite: Sprite {
                        custom_size: Some(DOG_SIZE * scale_modifier),
                        ..default()
                    },
                    transform: Transform::from_xyz(random_x, random_y, 10.),
                    texture,
                    ..default()
                },
                RigidBody::Dynamic,
                Collider::ball(DOG_SIZE.x * scale_modifier * 0.47),
                Velocity {
                    linvel: Vec2::new(random_x, random_y),
                    angvel: 0.3,
                },
                Sleeping::disabled(),
                ActiveCollisionTypes::all(),
                ActiveEvents::COLLISION_EVENTS,
                Enemy {
                    direction: Vec2::new(rand.gen::<f32>(), rand.gen::<f32>())
                        .normalize(),
                },
                Name::new(name.clone()),
            ))
            .with_children(|parent| {
                parent
                    .spawn(SpriteBundle {
                        sprite: Sprite {
                            custom_size: Some(
                                Vec2::splat(125.) * scale_modifier,
                            ),
                            ..default()
                        },
                        texture: asset_server
                            .load("sprites/Dog's spacesuit.png"),
                        transform: Transform::from_xyz(
                            -12.8 * 2. * scale_modifier,
                            13.2 * 2. * scale_modifier,
                            1.,
                        ),
                        ..default()
                    })
                    .with_children(|parent| {
                        parent.spawn((
                            SpriteBundle {
                                sprite: Sprite {
                                    custom_size: Some(
                                        Vec2::new(
                                            104. * 2. * scale_modifier,
                                            101. * 2. * scale_modifier,
                                        ) / 5.,
                                    ),
                                    anchor: Anchor::Custom(Vec2::new(
                                        0.55, -0.55,
                                    )),
                                    ..default()
                                },
                                texture: asset_server
                                    .load("sprites/Light reflection.png"),
                                transform: Transform::from_xyz(0., 0., 1.),
                                ..default()
                            },
                            PatchOfLight,
                        ));
                    });
            })
            .id();
        if name == "Doggy Potter" {
            let wand = commands
                .spawn((
                    Collider::cuboid(10., 130.),
                    ActiveEvents::COLLISION_EVENTS,
                    SpriteBundle {
                        transform: Transform {
                            translation: Vec3::new(-49.2, -51.6, -0.5),
                            rotation: Quat::from_rotation_z(-4.2),
                            scale: Vec3::new(0.3, 0.3, 0.),
                            ..default()
                        },
                        texture: asset_server.load("sprites/Magic wand.png"),
                        ..default()
                    },
                ))
                .id();
            commands.entity(entity).push_children(&[wand]);
        }
    }
    arriving_event.send(EnemyIsArrivingEvent(name));
}

pub fn rotate_patch_of_light(
    mut patch_query: Query<(Entity, &mut Transform), With<PatchOfLight>>,
    parents_query: Query<&Parent>,
    transforms_query: Query<&Transform, Without<PatchOfLight>>,
) {
    for (entity, mut transform) in patch_query.iter_mut() {
        let mut common_rotation = Quat::IDENTITY;
        // Collect all parents rotation
        for parent in parents_query.iter_ancestors(entity) {
            if let Ok(transform) = transforms_query.get(parent) {
                common_rotation = common_rotation.mul_quat(transform.rotation);
            }
        }
        // Apply rotation to patch
        rotate_transform_with_parent_calibration(
            &common_rotation,
            &mut transform,
            Vec2::NEG_X,
            Vec2::NEG_X,
            None,
        );
    }
}

fn generate_dog(
    dogs_resource: &mut ResMut<DogResource>,
    names_assets: Res<Assets<DogNames>>,
    already_spawned_data: &mut Local<(HashSet<String>, u8)>,
) -> (String, Handle<Image>, f32) {
    // Rand
    let mut rand = rand::thread_rng();
    let last_name_possibility = rand.gen::<bool>();
    let nickname_possibility = rand.gen::<bool>() && last_name_possibility;

    // Get handles
    let dogs_names = names_assets.get(&dogs_resource.json_data).unwrap();
    let filename = dogs_resource.images[already_spawned_data.1 as usize]
        .0
        .clone();
    let image = dogs_resource.images[already_spawned_data.1 as usize]
        .1
        .clone();

    {
        if already_spawned_data.1 < 11 {
            already_spawned_data.1 += 1;
        } else {
            already_spawned_data.1 = 0;
            // Shuffle sprites on next round of spawning
            dogs_resource.images.shuffle(&mut rand);
        }
    }
    // let mut default_scale = rand.gen_range(0.3..1.2);
    let mut default_scale = 0.5;

    // Handle Doggy Potter case
    if &filename == "FaceHarry" {
        return (String::from("Doggy Potter"), image, default_scale);
    }

    // Handle Big Kid case
    if &filename == "Face8" {
        default_scale = 1.0;
    }

    // Generate name
    let mut name = String::new();
    loop {
        name.push_str(
            &dogs_names.first_names
                [rand.gen_range(0..dogs_names.first_names.len())],
        );
        if last_name_possibility {
            name = dogs_names.last_names
                [rand.gen_range(0..dogs_names.last_names.len())]
            .to_string()
                + " "
                + &name;
        }
        if nickname_possibility {
            name = name
                + " of "
                + &dogs_names.nicknames
                    [rand.gen_range(0..dogs_names.nicknames.len())]
                .to_string();
        }
        if !already_spawned_data.0.contains(&name) {
            break;
        }
    }

    (name, image, default_scale)
}
