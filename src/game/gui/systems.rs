use bevy::prelude::*;

// ----- Modules ------------------------------------------------------------ //

use crate::{events::PlayerHit, game::player::components::Player};

use super::{components::*, styles::*, HudState};

// ───── Body ─────────────────────────────────────────────────────────────── //

pub fn spawn_hud(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    assets: Res<Assets<Image>>,
) {
    // let img1: Handle<Image> =
    //     asset_server.load("sprites/State=Life 308.2 x 258.74.png");
    // let img2: Handle<Image> =
    //     asset_server.load("sprites/State=Life Background 341 x 286.png");
    // let img3: Handle<Image> =
    //     asset_server.load("sprites/State=No Life 341 x 286.png");

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
                                    image: asset_server
                                        .load("sprites/State=Life Background 341 x 286.png")
                                        .into(),
                                    ..default()
                                },
                                HeartImage::Back(id),
                            ))
                            .with_children(|parent| {
                                parent.spawn((
                                    ImageBundle {
                                        style: HEART_CONTENT,
                                        image: asset_server
                                            .load("sprites/State=Life 308.2 x 258.74.png")
                                            .into(),
                                        ..default()
                                    },
                                    HeartImage::Content(id),
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
    mut commands: Commands,
    player_query: Query<&Player>,
    mut heart_img: Query<(&Parent, Entity, &mut UiImage, &HeartImage)>,
    mut hud_state: ResMut<NextState<HudState>>,
    asset_server: Res<AssetServer>,
    assets: Res<Assets<Image>>,
) {
    if let Ok(player) = player_query.get_single() {
        for (parent, entity, mut image, heart_image_type) in heart_img.iter() {
            match heart_image_type {
                HeartImage::Content(id) if *id > player.health => {
                    dbg!(parent, entity, image, heart_image_type, &heart_img);
                    commands.entity(parent.get()).remove_children(&[entity]);
                    commands.entity(entity).despawn();
                }
                HeartImage::Back(id) => {
                    println!("HeartImage::Back(id) => {}", id);
                }
                HeartImage::Empty(id) => {
                    println!("HeartImage::Back(id) => {}", id);
                }
                _ => {
                    println!("EMPTY!")
                }
            }
        }
    }

    println!("-------------------Updating hud-------------------");
    hud_state.set(HudState::Idle);
}
