use bevy::prelude::*;
use bevy_tweening::TweenCompleted;
use std::println;

// ───── Current Crate Import ─────────────────────────────────────────────── //

use super::{components::*, styles::*, HudState};
use crate::game::gui::animation::*;
use crate::{events::PlayerHit, game::player::components::Player};

// ───── Body ─────────────────────────────────────────────────────────────── //

pub fn spawn_hud(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    assets: Res<Assets<Image>>,
) {
    let img1 = asset_server.load("sprites/State=Life Background 341 x 286.png");
    let img2 = asset_server.load("sprites/State=Life 308.2 x 258.74.png");
    // let img3 = asset_server.load("sprites/State=No Life 341 x 286.png");

    let hud_entity = commands
        .spawn((
            NodeBundle {
                style: HUD_CONTAINER,
                ..default()
            },
            HUD,
        ))
        .with_children(|parent| {
            // === Hearts Row === //
            parent
                .spawn(NodeBundle {
                    style: HEARS_ROW,
                    ..default()
                })
                .with_children(|parent| {
                    for id in 1..4 {
                        parent
                            .spawn((
                                ImageBundle {
                                    style: HEART_BACKGROUND,
                                    image: img1.clone().into(),
                                    ..default()
                                },
                                HeartImage::Back(id, img1.clone()),
                            ))
                            .with_children(|parent| {
                                parent.spawn((
                                    ImageBundle {
                                        style: HEART_CONTENT,
                                        image: img2.clone().into(),
                                        ..default()
                                    },
                                    HeartImage::Content(id, img2.clone()),
                                ));
                            });
                    }
                });
        });
}

pub fn despawn_hud(
    mut commands: Commands,
    hud_query: Query<Entity, With<HUD>>,
) {
    commands
        .entity(hud_query.get_single().unwrap())
        .despawn_recursive();
}

pub fn listen_events(
    mut event_reader: EventReader<PlayerHit>,
    mut hud_state: ResMut<NextState<HudState>>,
    assets: Res<Assets<Image>>,
) {
    for event in event_reader.iter() {
        println!("Player was hit! health: {}", event.remaining_health);
        hud_state.set(HudState::Update);
    }
}

pub fn update_hud(
    // Commands
    mut commands: Commands,
    // Events
    mut tween_event: EventReader<TweenCompleted>,
    // State
    mut hud_state: ResMut<NextState<HudState>>,
    // Local
    mut started_tweens: Local<(bool, bool)>,
    // Queries
    player_query: Query<&Player>,
    mut heart_img: Query<(Entity, &mut UiImage, &HeartImage)>,
    // Resources
    asset_server: Res<AssetServer>,
) {
    let health = match player_query.get_single() {
        Ok(player) => player.health,
        Err(_) => 0,
    };

    // Animation phase 1
    if !started_tweens.0 {
        for (entity, _, heart_image_type) in heart_img.iter() {
            match heart_image_type {
                HeartImage::Content(id, _) if *id == health + 1 => {
                    animate_heart_down(&mut commands, entity, *id);
                    started_tweens.0 = true;
                }
                _ => {}
            }
        }
    }

    // Animation phase 1
    // Check if tween is completed
    for event in tween_event.iter() {
        let ev_id = event.user_data as u8;

        for (entity, mut image, heart_image_type) in heart_img.iter_mut() {
            match heart_image_type {
                HeartImage::Content(id, texture) if ev_id == *id => {
                    // If all tweens finished
                    if started_tweens.1 {
                        started_tweens.0 = false;
                        started_tweens.1 = false;
                        hud_state.set(HudState::Idle);
                        return;
                    }

                    // Change red texture to grey
                    image.texture = asset_server
                        .load("sprites/State=No Life 341 x 286.png");
                    animate_heart_up(&mut commands, entity, *id);
                    started_tweens.1 = true;
                }
                _ => {}
            }
        }
        break;
    }
}
