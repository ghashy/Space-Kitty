use bevy::{prelude::*, window::PrimaryWindow};
use bevy_rapier2d::prelude::*;

// ───── Current Crate Imports ────────────────────────────────────────────── //

use super::{components::*, PLAYER_SPEED};
use super::{PlayerState, BALL_SIZE};
use crate::audio_system::resources::SamplePack;
use crate::events::{GameOver, PlayerHit};
use crate::game::enemy::components::*;
use crate::game::score::resources::Score;
use crate::game::star::components::{Star, StarsPack};
use crate::game::star::STAR_SIZE;

// ───── Body ─────────────────────────────────────────────────────────────── //

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
        Player { health: 3 },
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
    // Events
    mut game_over_event_writer: EventWriter<GameOver>,
    mut event_writer: EventWriter<PlayerHit>,
    // Queries
    mut player_query: Query<(Entity, &Transform, &mut Player)>,
    enemy_query: Query<&Transform, With<Enemy>>,
    // State
    mut player_state: ResMut<NextState<PlayerState>>,
    // Audio
    audio: Res<Audio>,
    sample_pack: Res<SamplePack>,
    // Assistants
    score: Res<Score>,
) {
    if let Ok((player_entity, player_transform, mut player)) =
        player_query.get_single_mut()
    {
        for enemy_transform in enemy_query.iter() {
            let distance = player_transform
                .translation
                .distance(enemy_transform.translation);
            let ball_radius = BALL_SIZE;

            if distance < ball_radius + ball_radius {
                if player.health > 1 {
                    player.health -= 1;
                    // Spawn Timer to Player entity
                    commands.entity(player_entity).insert(
                        PlayerInvulnerableTimer(Timer::from_seconds(
                            3.,
                            TimerMode::Once,
                        )),
                    );
                    player_state.set(PlayerState::Invulnerable);
                } else {
                    player.health -= 1;
                    println!("Game over!");
                    audio.play(sample_pack.exp.clone());
                    commands.entity(player_entity).despawn();
                    println!("DESPAWNED");

                    game_over_event_writer.send(GameOver {
                        final_score: score.value,
                    })
                }
                event_writer.send(PlayerHit {
                    remaining_health: player.health,
                });
            }
        }
    }
}

pub fn player_hit_star(
    mut commands: Commands,
    player_query: Query<&Transform, With<Player>>,
    star_query: Query<(Entity, &Transform), With<Star>>,
    star_pack: Query<Entity, With<StarsPack>>,
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
                commands
                    .entity(star_pack.single())
                    .remove_children(&[star_entity]);

                audio.play(sample_pack.pick_star.clone());
                commands.entity(star_entity).despawn();
                score.last_value = score.value;
                score.value += 1;
            }
        }
    }
}

pub fn count_player_invulnerability_timer(
    mut commands: Commands,
    time: Res<Time>,
    mut player_state: ResMut<NextState<PlayerState>>,
    mut player_query: Query<
        (Entity, &mut PlayerInvulnerableTimer),
        With<Player>,
    >,
) {
    if let Ok((entity, mut timer)) = player_query.get_single_mut() {
        if timer.0.tick(time.delta()).finished() {
            player_state.set(PlayerState::Vulnerable);
            commands.entity(entity).remove::<PlayerInvulnerableTimer>();
            println!("VULNERABILITY!");
        }
    }
}

pub fn blink_player(
    mut player_query: Query<
        (&mut Sprite, &PlayerInvulnerableTimer),
        With<Player>,
    >,
) {
    if let Ok((mut sprite, timer)) = player_query.get_single_mut() {
        if timer.0.percent() >= 0.9 {
            sprite.color.set_a(1.);
        } else {
            sprite
                .color
                .set_a((timer.0.elapsed_secs() * 8.).sin().abs());
        }
    }
}
