use bevy::{prelude::*, window::PrimaryWindow};
use bevy_rapier2d::prelude::*;

// ----- Crate -------------------------------------------------------------- //

use crate::audio_system::resources::SamplePack;
use crate::game::enemy::components::*;
use crate::game::score::resources::Score;
use crate::game::star::components::Star;
use crate::game::star::STAR_SIZE;
use crate::{events::GameOver, helper_functions::*};

// ----- Module ------------------------------------------------------------- //

use super::BALL_SIZE;
use super::{components::Player, PLAYER_SPEED};

// ----- Body --------------------------------------------------------------- //

pub fn spawn_player(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    // Assume that there can be only one entity of PrimaryWindow at the time
    let window = window_query.get_single().unwrap();

    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(
                window.width() / 2.,
                window.height() / 2.,
                0.,
            ),
            texture: asset_server.load("sprites/ball_blue_large.png"),
            ..default()
        },
        RigidBody::Dynamic,
        Collider::ball(BALL_SIZE),
        ExternalForce {
            force: Vec2::ZERO,
            torque: 0.,
        },
        Damping {
            linear_damping: 0.6,
            angular_damping: 5.,
        },
        ActiveCollisionTypes::all(),
        ActiveEvents::COLLISION_EVENTS,
        Restitution::coefficient(1.),
        Player {},
    ));
}

pub fn despawn_player(
    mut commands: Commands,
    player_query: Query<Entity, With<Player>>,
) {
    if let Ok(player) = player_query.get_single() {
        commands.entity(player).despawn();
    }
}

pub fn player_movement(
    keyboard_input: Res<Input<KeyCode>>,
    mut player_query: Query<&mut ExternalForce, With<Player>>,
    time: Res<Time>,
) {
    if let Ok(mut player) = player_query.get_single_mut() {
        let mut direction = Vec3::ZERO;

        let top = KeyCode::W;
        let down = KeyCode::S;
        let left = KeyCode::A;
        let right = KeyCode::D;

        if keyboard_input.pressed(left) {
            direction += Vec3::new(-1., 0., 0.);
        }
        if keyboard_input.pressed(right) {
            direction += Vec3::new(1., 0., 0.);
        }
        if keyboard_input.pressed(top) {
            direction += Vec3::new(0., 1., 0.);
        }
        if keyboard_input.pressed(down) {
            direction += Vec3::new(0., -1., 0.);
        }

        if direction.length() > 0.0 {
            direction = direction.normalize();
        }

        player.force =
            direction.truncate() * PLAYER_SPEED * time.delta_seconds();
    }
}

pub fn enemy_hit_player(
    mut commands: Commands,
    mut game_over_event_writer: EventWriter<GameOver>,
    mut player_query: Query<(Entity, &Transform), With<Player>>,
    enemy_query: Query<&Transform, With<Enemy>>,
    audio: Res<Audio>,
    sample_pack: Res<SamplePack>,
    score: Res<Score>,
) {
    if let Ok((player_entity, player_transform)) = player_query.get_single_mut()
    {
        for enemy_transform in enemy_query.iter() {
            let distance = player_transform
                .translation
                .distance(enemy_transform.translation);
            let ball_radius = BALL_SIZE;

            if distance < ball_radius + ball_radius {
                println!("Game over!");
                audio.play(sample_pack.exp.clone());
                commands.entity(player_entity).despawn();

                game_over_event_writer.send(GameOver {
                    final_score: score.value,
                })
            }
        }
    }
}

pub fn player_hit_star(
    mut commands: Commands,
    player_query: Query<&Transform, With<Player>>,
    star_query: Query<(Entity, &Transform), With<Star>>,
    audio: Res<Audio>,
    sample_pack: Res<SamplePack>,
    mut score: ResMut<Score>,
) {
    if let Ok(player_transform) = player_query.get_single() {
        for (star_entity, star_transform) in star_query.iter() {
            let distance = player_transform
                .translation
                .distance(star_transform.translation);

            let ball_radius = BALL_SIZE / 2.;
            let star_radius = STAR_SIZE / 2.;

            if distance < ball_radius + star_radius {
                audio.play(sample_pack.pick_star.clone());
                commands.entity(star_entity).despawn();
                score.last_value = score.value;
                score.value += 1;
            }
        }
    }
}
