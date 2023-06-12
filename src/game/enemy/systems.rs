use bevy::{prelude::*, sprite::Anchor, utils::HashMap, window::PrimaryWindow};
use bevy_rapier2d::prelude::*;
use rand::prelude::*;
use std::ops::Range;

// ───── Current Crate Imports ────────────────────────────────────────────── //

use super::{
    components::{Enemy, EnemyIsArrivingEvent, PatchOfLight},
    *,
};
use crate::game::{player::SPACESHIP_SIZE, score::resources::Score};
use crate::helper_functions::*;
use crate::{
    audio_system::resources::SamplePack,
    game::fish::components::FishWasPickedEvent,
};

// ───── Body ─────────────────────────────────────────────────────────────── //

//
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
            direction_changed = true;

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
                    enemy
                        .direction
                        .reflect(contact_pair.manifold(0).unwrap().normal());
                    // Write entity2 to flags
                    entities_flags = entities_flags | 0b10;
                }
            }
            // Break if got actual collision
            if entities_flags != 0b00 {
                break;
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
    score: ResMut<Score>,
    mut names_list: Local<HashMap<String, String>>,
    mut picked_event: EventReader<FishWasPickedEvent>,
    mut arriving_event: EventWriter<EnemyIsArrivingEvent>,
) {
    // let score = score.get_score("Kitty").unwrap();
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

        let mut name = generate_rand_name();
        while names_list.contains_key(&name) {
            name = generate_rand_name();
        }
        names_list.insert(name.clone(), String::new());

        commands
            .spawn((
                SpriteBundle {
                    sprite: Sprite {
                        custom_size: Some(Vec2::splat(SPACESHIP_SIZE)),
                        ..default()
                    },
                    transform: Transform::from_xyz(random_x, random_y, 10.),
                    texture: asset_server.load("sprites/Dog.png"),
                    ..default()
                },
                RigidBody::Dynamic,
                Collider::ball(SPACESHIP_SIZE / 2.),
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
                Name::new(name),
            ))
            .with_children(|parent| {
                parent
                    .spawn(SpriteBundle {
                        sprite: Sprite {
                            custom_size: Some(Vec2::splat(65.)),
                            ..default()
                        },
                        texture: asset_server
                            .load("sprites/Dog's spacesuit.png"),
                        transform: Transform::from_xyz(-12.8, 13.2, 1.),
                        ..default()
                    })
                    .with_children(|parent| {
                        parent.spawn((
                            SpriteBundle {
                                sprite: Sprite {
                                    custom_size: Some(
                                        Vec2::new(104., 101.) / 5.,
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
            });
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

const FIRST_NAMES: [&str; 15] = [
    "Arnold", "Misha", "Adam", "Huan", "Ivan", "Toby", "Fox", "Robert",
    "Antonio", "Rabbit", "Buddy", "Wolf", "El Dog", "Shepard", "Slime",
];

const LAST_NAMES: [&str; 6] =
    ["Little", "Big", "Daring", "Cowardly", "Kind", "Peaceful"];

const NICKNAMES: [&str; 3] = ["Dark", "Light", "Sniffing"];

fn generate_rand_name() -> String {
    let mut rand = rand::thread_rng();
    let last_name_possibility = rand.gen::<bool>();
    let nickname_possibility = rand.gen::<bool>() && last_name_possibility;

    let mut name = String::new();
    name.push_str(FIRST_NAMES[rand.gen_range(0..FIRST_NAMES.len())]);

    if last_name_possibility {
        name = LAST_NAMES[rand.gen_range(0..LAST_NAMES.len())].to_string()
            + " "
            + &name;
    }
    if nickname_possibility {
        name = name
            + " of "
            + &NICKNAMES[rand.gen_range(0..NICKNAMES.len())].to_string();
    }

    name
}
