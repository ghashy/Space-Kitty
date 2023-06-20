use bevy::sprite::Anchor;
use bevy::{prelude::*, window::PrimaryWindow};
use bevy_hanabi::*;
use bevy_rapier2d::prelude::*;

// ───── Current Crate Imports ────────────────────────────────────────────── //

use super::{components::*, PLAYER_SPEED};
use super::{PlayerState, SPACESHIP_SIZE};
use crate::audio_system::resources::SamplePack;
use crate::events::{GameOver, PlayerHit};
use crate::game::enemy::components::*;
use crate::game::score::resources::Score;
use crate::helper_functions::*;

// ───── Body ─────────────────────────────────────────────────────────────── //

pub fn spawn_player(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
    mut effects: ResMut<Assets<EffectAsset>>,
) {
    // Assume that there can be only one entity of PrimaryWindow at the time
    let window = window_query.get_single().unwrap();

    let mut color_gradient = Gradient::new();
    color_gradient.add_key(0.0, Vec4::new(0., 0.07, 0.06, 0.0));
    color_gradient.add_key(0.2, Vec4::new(0.06, 0.02, 0.10, 0.5));
    color_gradient.add_key(1.0, Vec4::new(0., 0., 0., 0.));

    let effect = effects.add(
        EffectAsset {
            name: "RocketFlame".to_string(),
            capacity: 1000,
            spawner: Spawner::rate(70.0.into()).with_starts_active(false),
            ..default()
        }
        .init(InitLifetimeModifier {
            lifetime: 3_f32.into(),
        })
        .init(InitPositionCircleModifier {
            radius: 11.,
            ..default()
        })
        .init(InitVelocityCircleModifier {
            axis: Vec3::Z,
            speed: 30.0.into(),
            ..default()
        })
        .init(InitVelocityTangentModifier {
            speed: Value::Uniform((-20., 20.)),
            axis: Vec3::Z,
            ..default()
        })
        .render(ParticleTextureModifier {
            texture: asset_server.load("sprites/Smoke.png"),
        })
        .render(SizeOverLifetimeModifier {
            gradient: Gradient::constant(Vec2::splat(25.0)),
        })
        .render(ColorOverLifetimeModifier {
            gradient: color_gradient,
        }),
    );

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
            Name::new("Kitty"),
        ))
        .with_children(|parent| {
            parent
                .spawn((
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
                ))
                .with_children(|parent| {
                    parent.spawn((
                        RocketEngineParticles,
                        ParticleEffectBundle {
                            transform: Transform::from_translation(
                                (Vec2::NEG_Y * 33.).extend(0.),
                            ),
                            effect: ParticleEffect::new(effect)
                                .with_z_layer_2d(Some(0.)),
                            ..default()
                        },
                        Name::new("RocketEngineParticles"),
                    ));
                });
        });
}

pub fn despawn_player_on_exit_game_state(
    mut commands: Commands,
    player_query: Query<Entity, With<Player>>,
) {
    if let Ok(player) = player_query.get_single() {
        despawn_player(&mut commands, player);
    }
}

pub fn despawn_player(commands: &mut Commands, player: Entity) {
    commands.entity(player).despawn_recursive();
}

pub fn player_movement(
    keyboard_input: Res<Input<KeyCode>>,
    mut player_query: Query<(&mut ExternalForce, &Transform), With<Player>>,
    mut spawner_query: Query<&mut EffectSpawner, With<RocketEngineParticles>>,
    time: Res<Time>,
    mut rocket_transform_query: Query<
        &mut Transform,
        (With<RocketEngineSprite>, Without<Player>),
    >,
) {
    if let Ok((mut player, player_transform)) = player_query.get_single_mut() {
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

        // If there are some input
        if direction.length() > 0.0 {
            direction = direction.normalize();
            // Animate engine rotation
            rotate_transform_with_parent_calibration(
                &player_transform.rotation,
                &mut rocket_transform_query.single_mut(),
                direction.truncate() * -1.,
                // Our sprite was drawn in this axis
                Vec2::NEG_Y,
                Some(&time),
            );
        }

        player.force =
            direction.truncate() * PLAYER_SPEED * time.delta_seconds();

        if let Ok(mut spawner) = spawner_query.get_single_mut() {
            spawner.set_active(direction.length() > 0.0);
        }
    }
}

pub fn enemy_hit_player(
    mut commands: Commands,
    mut collision_events: EventReader<CollisionEvent>,
    enemies: Query<Entity, With<Enemy>>,
    mut player_query: Query<(Entity, &mut Player), Without<Enemy>>,
    mut player_state: ResMut<NextState<PlayerState>>,
    audio: Res<Audio>,
    sample_pack: Res<SamplePack>,
    _game_over_event_writer: EventWriter<GameOver>,
    mut event_writer: EventWriter<PlayerHit>,
    _score: Res<Score>,
) {
    for event in collision_events.iter() {
        if let Ok((player_entity, player)) = player_query.get_single_mut() {
            if let CollisionEvent::Started(entity1, entity2, _) = event {
                if player_entity == *entity1 {
                    // First entity is player
                    if handle_collision(
                        &enemies,
                        entity2,
                        player,
                        player_entity,
                        &mut player_state,
                        &mut commands,
                        &audio,
                        &sample_pack,
                        &mut event_writer,
                    ) {
                        break;
                    }
                } else if player_entity == *entity2 {
                    // Second entity is player
                    if handle_collision(
                        &enemies,
                        entity2,
                        player,
                        player_entity,
                        &mut player_state,
                        &mut commands,
                        &audio,
                        &sample_pack,
                        &mut event_writer,
                    ) {
                        break;
                    }
                }
            }
        }
    }
}

// TODO: add collision sound
fn handle_collision(
    enemies_query: &Query<'_, '_, Entity, With<Enemy>>,
    collided_with: &Entity,
    mut player: Mut<'_, Player>,
    player_entity: Entity,
    player_state: &mut ResMut<'_, NextState<PlayerState>>,
    commands: &mut Commands<'_, '_>,
    audio: &Res<'_, Audio>,
    sample_pack: &Res<'_, SamplePack>,
    event_writer: &mut EventWriter<'_, PlayerHit>,
) -> bool {
    if enemies_query.iter().any(|e| e == *collided_with) {
        // Collision
        if player.health > 1 {
            player.health -= 1;
            // Spawn Timer to Player entity
            commands
                .entity(player_entity)
                .insert(PlayerInvulnerableTimer(Timer::from_seconds(
                    3.,
                    TimerMode::Once,
                )));
            player_state.set(PlayerState::Invulnerable);
        } else {
            player.health -= 1;
            audio.play(sample_pack.exp.clone());
            despawn_player(commands, player_entity);

            // TODO: handle gameover event, add function to score to get highest score
            // game_over_event_writer.send(GameOver {
            //     final_score: score.value,
            // })
        }
        event_writer.send(PlayerHit {
            remaining_health: player.health,
        });
        return true;
    }
    false
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
