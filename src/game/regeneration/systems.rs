use bevy::{prelude::*, window::PrimaryWindow};
use bevy_rapier2d::prelude::*;
use rand::Rng;

// ───── Current Crate Imports ────────────────────────────────────────────── //

use super::components::FlyingMilk;
use super::resources::FlyingMilkResource;
use super::{RegeneratePlayerEvent, MAX_SPAWN_TIME, MILK_SPEED};
use crate::game::player::components::Player;
use crate::helper_functions::VectorUtilities;

// ───── Body ─────────────────────────────────────────────────────────────── //

pub fn spawn_milk_cup(
    mut commands: Commands,
    player_query: Query<&Player>,
    mut milk: ResMut<FlyingMilkResource>,
    time: Res<Time>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    if let Ok(player) = player_query.get_single() {
        if player.health < 3 && milk.timer.is_none() {
            let mut rng = rand::thread_rng();
            let rand_time = rng.gen_range(0.0..MAX_SPAWN_TIME);
            let timer = Timer::from_seconds(rand_time, TimerMode::Once);
            milk.timer = Some(timer);
        } else if player.health < 3 {
            if milk
                .timer
                .as_mut()
                .unwrap()
                .tick(time.delta())
                .just_finished()
            {
                let window = window_query.get_single().unwrap();
                let center =
                    Vec2::new(window.width() / 2., window.height() / 2.);

                let mut rand_point = Vec2::new_rand();
                rand_point *= 1000.;
                rand_point += center;

                let direction = (center - rand_point).normalize();

                let mut rng = rand::thread_rng();
                let mut rotation =
                    if rng.gen_range(0..50) > 1 { 0.3 } else { 7. };
                rotation *= if rng.gen::<bool>() { 1. } else { -1. };

                commands.spawn((
                    SpriteBundle {
                        sprite: Sprite {
                            custom_size: Some(Vec2::new(210., 280.) / 7.),
                            ..default()
                        },
                        texture: asset_server
                            .load("sprites/A glass of milk.png"),
                        transform: Transform::from_xyz(
                            rand_point.x,
                            rand_point.y,
                            10.,
                        ),
                        ..default()
                    },
                    Sensor,
                    Collider::ball(145. / 7.),
                    FlyingMilk {
                        direction,
                        rotation,
                    },
                ));
            }
        } else if milk.timer.is_some() {
            milk.timer = None;
        }
    }
}

pub fn cup_of_milk_movement(
    mut milk_query: Query<(&mut Transform, &FlyingMilk)>,
    time: Res<Time>,
) {
    for (mut transform, milk) in milk_query.iter_mut() {
        let x = milk.direction.x * time.delta_seconds() * MILK_SPEED;
        let y = milk.direction.y * time.delta_seconds() * MILK_SPEED;

        transform.translation.x += x;
        transform.translation.y += y;

        transform.rotate_z(milk.rotation * time.delta_seconds());
    }
}

pub fn check_collision(
    mut commands: Commands,
    rapier_contexrt: Res<RapierContext>,
    milk_query: Query<Entity, With<FlyingMilk>>,
    player_query: Query<(Entity, &Player)>,
    mut event_writer: EventWriter<RegeneratePlayerEvent>,
) {
    if let Ok(milk) = milk_query.get_single() {
        if let Ok((entity, player)) = player_query.get_single() {
            if rapier_contexrt
                .intersection_pair(milk, entity)
                .is_some_and(|v| v)
            {
                commands.entity(milk).despawn();
                event_writer.send(RegeneratePlayerEvent {
                    new_health: player.health + 1,
                });
            }
        }
    }
}

pub fn despawn_milk(mut commands: Commands, milk_query: Query<(&Transform, Entity), With<FlyingMilk>>) {
    if let Some((transform, milk)) = milk_query.get_single() {
        if 
    }
}
