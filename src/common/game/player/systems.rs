use bevy::render::view::window;
use bevy::sprite::Anchor;
use bevy::{prelude::*, window::PrimaryWindow};
use bevy_rapier2d::prelude::*;
use kira::sound::static_sound::{StaticSoundHandle, StaticSoundSettings};
use rand::Rng;
use std::time::Duration;

// ───── Current Crate Imports ────────────────────────────────────────────── //

use super::{components::*, PLAYER_SPEED};
use super::{PlayerState, SPACESHIP_SIZE};
use crate::common::audio::assets::AudioSource;
use crate::common::audio::resources::{KiraManager, SamplePack};
use crate::common::events::{GameOver, PlayerHit};
use crate::common::game::components::Wall;
use crate::common::game::enemy::components::*;
use crate::common::game::fish::FISH_SIZE;
use crate::common::game::gui::components::Avatar;
use crate::common::game::regeneration::RegeneratePlayerEvent;
use crate::common::game::score::resources::Score;
use crate::common::game::score::ScoreUpdateEvent;
use crate::common::helper_functions::*;
use crate::common::resources::TextureStorage;

// ───── Body ─────────────────────────────────────────────────────────────── //

pub fn spawn_player_without_gpu_particles(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    // Assume that there can be only one entity of PrimaryWindow at the time
    let window = window_query.get_single().unwrap();

    commands
        .spawn((
            SpriteBundle {
                sprite: Sprite {
                    custom_size: Some(Vec2::splat(SPACESHIP_SIZE)),
                    ..default()
                },
                transform: Transform::from_xyz(
                    window.width() / 2.,
                    window.height() / 2.,
                    10.,
                ),
                texture: asset_server.load("sprites/Cat's starship.png"),
                ..default()
            },
            RigidBody::Dynamic,
            Collider::ball(SPACESHIP_SIZE / 2.),
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
            Avatar(asset_server.load("sprites/Avatars/Frame Kitty.png")),
            Name::new("Kitty"),
        ))
        .with_children(|parent| {
            parent.spawn((
                SpriteBundle {
                    sprite: Sprite {
                        custom_size: Some(Vec2::new(53., 29.) / 1.5),
                        anchor: Anchor::Custom(Vec2::new(0., 1.6)),
                        ..default()
                    },
                    transform: Transform::from_xyz(0., -1.5, -1.),
                    texture: asset_server
                        .load("sprites/Rocket engine.png")
                        .into(),

                    ..default()
                },
                RocketEngineSprite,
            ));
        });
}

pub fn despawn_player_on_exit_game_state(
    mut commands: Commands,
    player_query: Query<Entity, With<Player>>,
    mut player_state: ResMut<NextState<PlayerState>>,
) {
    if let Ok(player) = player_query.get_single() {
        despawn_player(&mut commands, player);
        player_state.set(PlayerState::Vulnerable);
    }
}

pub fn despawn_player(commands: &mut Commands, player: Entity) {
    commands.entity(player).despawn_recursive();
}

pub fn player_movement_without_gpu_particles(
    touches: Res<Touches>,
    keyboard_input: Res<Input<KeyCode>>,
    mut player_query: Query<(&mut ExternalForce, &Transform), With<Player>>,
    time: Res<Time>,
    mut rocket_transform_query: Query<
        &mut Transform,
        (With<RocketEngineSprite>, Without<Player>),
    >,
    mut kira_manager: NonSendMut<KiraManager>,
    audio_assets: Res<Assets<AudioSource>>,
    sample_pack: Res<SamplePack>,
    mut local_is_playing: Local<bool>,
    mut local_engine_handle: Local<Option<StaticSoundHandle>>,
    asset_server: Res<AssetServer>,
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    if let Ok((mut player, player_transform)) = player_query.get_single_mut() {
        let mut direction = Vec2::ZERO;

        let up = KeyCode::W;
        let down = KeyCode::S;
        let left = KeyCode::A;
        let right = KeyCode::D;

        let up_opt = KeyCode::Up;
        let down_opt = KeyCode::Down;
        let left_opt = KeyCode::Left;
        let right_opt = KeyCode::Right;

        if keyboard_input.pressed(left) || keyboard_input.pressed(left_opt) {
            direction += Vec2::new(-1., 0.);
        }
        if keyboard_input.pressed(right) || keyboard_input.pressed(right_opt) {
            direction += Vec2::new(1., 0.);
        }
        if keyboard_input.pressed(up) || keyboard_input.pressed(up_opt) {
            direction += Vec2::new(0., 1.);
        }
        if keyboard_input.pressed(down) || keyboard_input.pressed(down_opt) {
            direction += Vec2::new(0., -1.);
        }

        let window = window_query.get_single().unwrap();

        for touch in touches.iter() {
            println!("Player: {}", player_transform.translation.truncate());
            println!("Touch: {}", touch.position());
            // let pos =
            //     touch.position() + Vec2::new(window.width(), window.height());
            let pos = Vec2::new(
                touch.position().x,
                window.height() - touch.position().y,
            );
            direction += pos - player_transform.translation.truncate();
        }

        // If there are some input
        if direction.length() > 0.0 {
            direction = direction.normalize();
            // Animate engine rotation
            rotate_transform_with_parent_calibration(
                &player_transform.rotation,
                &mut rocket_transform_query.single_mut(),
                direction * -1.,
                // Our sprite was drawn in this axis
                Vec2::NEG_Y,
                Some(&time),
            );

            // Play engine audio
            // Button was just pressed
            if !*local_is_playing {
                let rand_pos = rand::thread_rng().gen_range(0.0..3.0);
                let sample = audio_assets
                    .get(&sample_pack.engine)
                    .unwrap()
                    .get()
                    .with_settings(
                        StaticSoundSettings::new()
                            .volume(0.01)
                            .output_destination(kira_manager.get_master()),
                    );
                sample
                    .settings
                    .output_destination(kira_manager.get_master());
                let mut handle = kira_manager.play(sample).unwrap();
                // For playing from rand position
                handle.seek_to(rand_pos).unwrap();
                handle
                    .set_volume(
                        0.21,
                        kira::tween::Tween {
                            duration: Duration::from_millis(100),
                            ..default()
                        },
                    )
                    .unwrap();
                handle.set_loop_region(..).unwrap();
                *local_is_playing = true;

                *local_engine_handle = Some(handle);
            }
        } else {
            // Stop only if already playing
            if *local_is_playing {
                if let Some(ref mut handle) = *local_engine_handle {
                    if let Err(e) = handle.stop(kira::tween::Tween {
                        duration: Duration::from_secs(1),
                        easing: kira::tween::Easing::OutPowf(1.),
                        ..default()
                    }) {
                        println!("Error engine sound stopping: {}", e);
                    }
                    *local_is_playing = false;
                }
            }
        }

        player.force = direction * PLAYER_SPEED * time.delta_seconds();

        // Engine particles
        if direction.length() > 0.0 {
            let mut rng = rand::thread_rng();

            let direction = direction
                .rotated(std::f32::consts::PI)
                .rotated(rng.gen_range(-0.3..0.3));
            let velocity = rng.gen_range(100.0..200.0);
            let timer = Timer::from_seconds(1.9, TimerMode::Once);
            let position = (player_transform.translation.truncate()
                + direction * 30.
                + Vec2::new(rng.gen(), rng.gen()).normalize() * 20.)
                .extend(player_transform.translation.z - 2.);

            commands.spawn((
                SpriteBundle {
                    sprite: Sprite {
                        custom_size: Some(FISH_SIZE),
                        color: Color::rgba(0.2, 0.16, 0.29, 0.),
                        ..default()
                    },
                    transform: Transform::from_translation(position)
                        .with_rotation(Quat::from_rotation_z(rng.gen())),
                    texture: asset_server.load("sprites/Smoke.png"),
                    ..default()
                },
                SmokeParticle {
                    direction,
                    velocity,
                    timer,
                },
            ));
        }
    } else {
        if *local_is_playing {
            if let Some(ref mut handle) = *local_engine_handle {
                if let Err(e) = handle.stop(kira::tween::Tween {
                    duration: Duration::from_secs(1),
                    easing: kira::tween::Easing::OutPowf(1.),
                    ..default()
                }) {
                    println!("Error engine sound stopping: {}", e);
                }
                *local_is_playing = false;
            }
        }
    }
}

pub fn spawn_particles_on_collision_with_enemy(
    mut commands: Commands,
    mut hit_events: EventReader<PlayerHit>,
    asset_server: Res<AssetServer>,
) {
    if let Some(event) = hit_events.iter().next() {
        let mut rng = rand::thread_rng();

        for _ in 0..event.drop_count {
            let direction = event
                .hit_normal
                .rotated(std::f32::consts::PI)
                .rotated(rng.gen_range(-0.9..0.9));
            let velocity = rng.gen_range(200.0..300.0);
            let timer = Timer::from_seconds(3., TimerMode::Once);
            let position = (event.position.truncate()
                + Vec2::new(rng.gen(), rng.gen()).normalize() * 20.)
                .extend(event.position.z);

            commands.spawn((
                SpriteBundle {
                    sprite: Sprite {
                        custom_size: Some(FISH_SIZE),
                        ..default()
                    },
                    transform: Transform::from_translation(position)
                        .with_rotation(Quat::from_rotation_z(rng.gen())),
                    texture: asset_server.load("sprites/Fish.png"),
                    ..default()
                },
                DropFishParticle {
                    direction,
                    velocity,
                    timer,
                },
            ));
        }
    }
}

pub fn poll_and_despawn_collision_particles(
    mut commands: Commands,
    mut particles_query: Query<(
        Entity,
        &mut Sprite,
        &mut Transform,
        &mut DropFishParticle,
    )>,
    time: Res<Time>,
) {
    for (entity, mut sprite, mut transform, mut particle) in
        particles_query.iter_mut()
    {
        if particle.timer.tick(time.delta()).finished() {
            commands.entity(entity).despawn();
        } else {
            let x =
                particle.direction.x * time.delta_seconds() * particle.velocity;
            let y =
                particle.direction.y * time.delta_seconds() * particle.velocity;

            transform.translation.x += x;
            transform.translation.y += y;
            sprite.color.set_a(particle.timer.percent_left());
            particle.velocity = particle.velocity - time.delta_seconds() * 87.;
        }
    }
}

pub fn poll_and_despawn_smoke_particles(
    mut commands: Commands,
    mut particles_query: Query<(
        Entity,
        &mut Sprite,
        &mut Transform,
        &mut SmokeParticle,
    )>,
    time: Res<Time>,
) {
    for (entity, mut sprite, mut transform, mut particle) in
        particles_query.iter_mut()
    {
        if particle.timer.tick(time.delta()).finished() {
            commands.entity(entity).despawn();
        } else {
            let x =
                particle.direction.x * time.delta_seconds() * particle.velocity;
            let y =
                particle.direction.y * time.delta_seconds() * particle.velocity;

            transform.translation.x += x;
            transform.translation.y += y;
            let diff = 1.0 - particle.timer.percent_left();
            let alpha = if diff < 0.5 {
                diff
            } else {
                particle.timer.percent_left()
            };
            sprite.color.set_a(alpha);
            sprite.color.set_r(particle.timer.percent_left() * 0.5);
            sprite.color.set_g(particle.timer.percent_left() * 0.1);
            particle.velocity = particle.velocity - time.delta_seconds() * 87.;
        }
    }
}

pub fn despawn_smoke_particles_on_exit_state(
    mut commands: Commands,
    particles_query: Query<Entity, With<SmokeParticle>>,
) {
    for entity in particles_query.iter() {
        commands.entity(entity).despawn();
    }
}

pub fn despawn_collision_particles(
    mut commands: Commands,
    particles_query: Query<Entity, With<DropFishParticle>>,
) {
    for particle in particles_query.iter() {
        commands.entity(particle).despawn();
    }
}

pub fn handle_player_collision(
    state: Res<State<PlayerState>>,
    mut commands: Commands,
    mut collision_events: EventReader<CollisionEvent>,
    enemies: Query<(Entity, &GlobalTransform), With<Enemy>>,
    walls: Query<Entity, With<Wall>>,
    mut player_query: Query<
        (Entity, &GlobalTransform, &mut Player),
        Without<Enemy>,
    >,
    mut player_state: ResMut<NextState<PlayerState>>,
    mut kira_manager: NonSendMut<KiraManager>,
    audio_assets: Res<Assets<AudioSource>>,
    sample_pack: Res<SamplePack>,
    mut game_over_event_writer: EventWriter<GameOver>,
    mut event_writer: EventWriter<PlayerHit>,
    mut score_events: EventWriter<ScoreUpdateEvent>,
    mut score: ResMut<Score>,
) {
    for event in collision_events.iter() {
        if let Ok((player_entity, global_transform, mut player)) =
            player_query.get_single_mut()
        {
            if let CollisionEvent::Started(entity1, entity2, _) = event {
                let collided_with;

                if player_entity == *entity1 {
                    collided_with = *entity2;
                } else if player_entity == *entity2 {
                    collided_with = *entity1;
                } else {
                    continue;
                }

                if let Some((_, enemy_global_transform)) =
                    enemies.iter().find(|(e, _)| {
                        *e == collided_with
                        // A little bit strange, but whatever
                            && *state.get() == PlayerState::Vulnerable
                    })
                {
                    // Collision
                    if player.health > 1 {
                        player.health -= 1;

                        // Play alarm sound
                        let sound_data = audio_assets
                            .get(&sample_pack.alarm)
                            .unwrap()
                            .get()
                            .with_settings(
                                StaticSoundSettings::new()
                                    .volume(0.5)
                                    .output_destination(
                                        kira_manager.get_master(),
                                    ),
                            );
                        kira_manager.play(sound_data).unwrap();

                        // Play meow sound
                        let sound_data = audio_assets
                            .get(get_random_meow(&sample_pack))
                            .unwrap()
                            .get()
                            .with_settings(
                                StaticSoundSettings::new()
                                    .volume(0.8)
                                    .output_destination(
                                        kira_manager.get_master(),
                                    ),
                            );
                        kira_manager.play(sound_data).unwrap();

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

                        despawn_player(&mut commands, player_entity);

                        game_over_event_writer.send(GameOver);
                    }
                    // Remove 25% from kitty's score
                    let drop_count = match score.drop_score(player_entity, 0.1)
                    {
                        Ok(score) => {
                            score_events.send(ScoreUpdateEvent {
                            name: Name::from("Kitty"),
                            event_type:
                                crate::common::game::score::ScoreEventType::ScoreDrop(
                                    score,
                                ),
                        }
                        );
                            score
                        }
                        Err(_) => panic!("No score for Kitty!"),
                    };
                    // Write event with collision data
                    let hit_normal =
                        (enemy_global_transform.translation().truncate()
                            - global_transform.translation().truncate())
                        .normalize();
                    let position = global_transform.translation();
                    event_writer.send(PlayerHit {
                        remaining_health: player.health,
                        position,
                        hit_normal,
                        drop_count,
                    });
                } else if walls.iter().any(|e| collided_with == e) {
                    // Play meow sound
                    let sound_data = audio_assets
                        .get(&sample_pack.meow5)
                        .unwrap()
                        .get()
                        .with_settings(
                            StaticSoundSettings::new()
                                .volume(0.23)
                                .output_destination(kira_manager.get_master()),
                        );

                    kira_manager.play(sound_data).unwrap();
                }
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
        }
    }
}

// TODO: We should make blinking in shader, for group transparecy animation.
pub fn blink_player(
    mut player_query: Query<
        (&mut Sprite, &PlayerInvulnerableTimer),
        With<Player>,
    >,
    mut engine_query: Query<
        &mut Sprite,
        (With<RocketEngineSprite>, Without<Player>),
    >,
) {
    if let Ok((mut sprite, timer)) = player_query.get_single_mut() {
        if timer.0.percent() >= 0.9 {
            sprite.color.set_a(1.);
            if let Ok(mut engine) = engine_query.get_single_mut() {
                engine.color.set_a(1.);
            }
        } else {
            sprite
                .color
                .set_a((timer.0.elapsed_secs() * 8.).sin().abs());
            if let Ok(mut engine) = engine_query.get_single_mut() {
                engine
                    .color
                    .set_a((timer.0.elapsed_secs() * 8.).sin().abs());
            }
        }
    }
}

pub fn regenerate_player(
    mut player_query: Query<&mut Player>,
    mut regen_events: EventReader<RegeneratePlayerEvent>,
) {
    if let Some(event) = regen_events.iter().next() {
        if let Ok(mut player) = player_query.get_single_mut() {
            player.health = event.new_health;
        }
    }
}

fn get_random_meow<'a>(
    sample_pack: &'a Res<SamplePack>,
) -> &'a Handle<AudioSource> {
    match rand::thread_rng().gen_range(0..8) {
        0 => &sample_pack.meow1,
        1 => &sample_pack.meow2,
        2 => &sample_pack.meow3,
        3 => &sample_pack.meow4,
        4 => &sample_pack.meow5,
        5 => &sample_pack.meow6,
        6 => &sample_pack.meow7,
        7 => &sample_pack.meowroar,
        _ => unreachable!(),
    }
}
