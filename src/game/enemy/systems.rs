use bevy::{prelude::*, window::PrimaryWindow};
use bevy_rapier2d::prelude::*;
use rand::prelude::*;
use std::ops::Range;

// ───── Current Crate Imports ────────────────────────────────────────────── //

use super::{components::Enemy, *};
use crate::{
    audio_system::resources::SamplePack,
    game::{player::BALL_SIZE, score::resources::Score},
    helper_functions::*,
};

// ───── Body ─────────────────────────────────────────────────────────────── //

pub fn spawn_enemies(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let window = window_query.get_single().unwrap();

    let mut rand = thread_rng();
    for _ in 0..NUMBER_OF_ENEMIES_ON_START {
        let random_x = rand.gen::<f32>() * window.width() * 2.;
        let random_y = rand.gen::<f32>() * window.height() * 2.;

        commands.spawn((
            SpriteBundle {
                transform: Transform::from_xyz(random_x, random_y, 0.),
                texture: asset_server.load("sprites/ball_red_large.png"),
                ..default()
            },
            RigidBody::Dynamic,
            Collider::ball(BALL_SIZE),
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
    mut enemy_query: Query<(&mut Velocity, &Enemy)>,
    time: Res<Time>,
) {
    for (mut velocity, enemy) in enemy_query.iter_mut() {
        let direction = enemy.direction.extend(0.);
        velocity.linvel =
            direction.truncate() * ENEMY_SPEED * time.delta_seconds();
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
    mut score: ResMut<Score>,
) {
    if score.value % 5 == 0 && score.value > score.last_value {
        let window = window_query.get_single().unwrap();
        let mut rand = thread_rng();
        let random_x = rand.gen::<f32>() * window.width() * 2.;
        let random_y = rand.gen::<f32>() * window.height() * 2.;

        commands.spawn((
            SpriteBundle {
                transform: Transform::from_xyz(random_x, random_y, 0.),
                texture: asset_server.load("sprites/ball_red_large.png"),
                ..default()
            },
            RigidBody::Dynamic,
            Collider::ball(BALL_SIZE),
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
        ));

        score.last_value += 1;
    }
}
