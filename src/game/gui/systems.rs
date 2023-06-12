use bevy::prelude::*;
use bevy_tweening::TweenCompleted;
use std::println;

// ───── Current Crate Import ─────────────────────────────────────────────── //

use super::{components::*, styles::*, HudLivesState, LIVES_ID_OFFSET};
use crate::game::fish::components::FishWasPickedEvent;
use crate::game::gui::animation::*;
use crate::game::player::LIVES_COUNT;
use crate::{events::PlayerHit, game::player::components::Player};

// ───── Body ─────────────────────────────────────────────────────────────── //

pub fn spawn_hud(mut commands: Commands, asset_server: Res<AssetServer>) {
    let img1 = asset_server.load("sprites/Cat's state Life.png");
    let img2 = asset_server.load("sprites/Cat's state No life.png");

    commands
        .spawn((
            NodeBundle {
                style: HUD_CONTAINER,
                // background_color: BackgroundColor(Color::RED),
                ..default()
            },
            Hud,
        ))
        .with_children(|parent| {
            parent
                .spawn(NodeBundle {
                    // background_color: BackgroundColor(Color::GREEN),
                    style: RIGHT_SIDE_BLOCK,
                    ..default()
                })
                .with_children(|parent| {
                    parent
                        .spawn(NodeBundle {
                            style: HEARTS_ROW,
                            // background_color: BackgroundColor(Color::PINK),
                            ..default()
                        })
                        .with_children(|parent| {
                            for mut id in 1..=LIVES_COUNT {
                                id += LIVES_ID_OFFSET as u64;
                                parent.spawn((
                                    ImageBundle {
                                        style: CAT_FACE,
                                        image: img1.clone().into(),
                                        ..default()
                                    },
                                    HeartImage(id, img1.clone(), img2.clone()),
                                ));
                            }
                        });
                    parent.spawn((
                        NodeBundle {
                            // background_color: BackgroundColor(
                            //     Color::ORANGE_RED,
                            // ),
                            style: MESSAGES_BAR,
                            ..default()
                        },
                        MessagesList,
                    ));
                });
        });
}

pub fn despawn_hud(
    mut commands: Commands,
    hud_query: Query<Entity, With<Hud>>,
) {
    commands
        .entity(hud_query.get_single().unwrap())
        .despawn_recursive();
}

pub fn listen_hit_events(
    mut player_hit_events: EventReader<PlayerHit>,
    mut hud_state: ResMut<NextState<HudLivesState>>,
) {
    for event in player_hit_events.iter() {
        println!("Player was hit! health: {}", event.remaining_health);
        hud_state.set(HudLivesState::Update);
    }
}

pub fn update_messages(
    mut commands: Commands,
    mut list: Query<Entity, With<MessagesList>>,
    asset_server: Res<AssetServer>,
    mut picked_fish_events: EventReader<FishWasPickedEvent>,
) {
    for event in picked_fish_events.iter() {
        let label = TextBundle::from_section(
            format!("{} got another one!", &event.0),
            TextStyle {
                font: asset_server.load("fonts/Abaddon Bold.ttf"),
                font_size: 25.,
                color: Color::WHITE,
            },
        )
        .with_style(Style::default());
        let id = commands.spawn(label).id();
        commands.entity(list.single()).push_children(&[id]);
    }
}

pub fn remove_message() {}

pub fn update_lives(
    mut commands: Commands,
    mut tween_event: EventReader<TweenCompleted>,
    // State
    mut hud_state: ResMut<NextState<HudLivesState>>,
    // Local
    mut started_tweens: Local<(bool, bool)>,
    // Queries
    player_query: Query<&Player>,
    mut heart_images: Query<(Entity, &mut UiImage, &HeartImage)>,
) {
    // Animation phase 1
    if !started_tweens.0 {
        let health = match player_query.get_single() {
            Ok(player) => player.health,
            Err(_) => 0,
        };

        for (entity, _, &HeartImage(id, _, _)) in heart_images.iter() {
            if id == ((health + 1) as u64) + LIVES_ID_OFFSET {
                animate_heart_down(&mut commands, entity, id);
                started_tweens.0 = true;
                return;
            }
        }
    }

    // Animation phase 2
    // Check all tween events
    for event in tween_event.iter() {
        let event_id = event.user_data;
        let lives_range = LIVES_ID_OFFSET..=LIVES_ID_OFFSET + LIVES_COUNT;
        // We should to process only lives animation events here
        if !(lives_range).contains(&event_id) {
            continue;
        }

        // Filter all other images except our current animing image
        let mut our_image = heart_images
            .iter_mut()
            .filter(|(_, _, heart_image)| heart_image.0 == event_id);

        if let Some((entity, mut image, heart_image)) = our_image.next() {
            // End of animation
            if started_tweens.1 {
                started_tweens.0 = false;
                started_tweens.1 = false;
                hud_state.set(HudLivesState::Idle);
                return;
            }

            // Change texture
            image.texture = heart_image.2.clone_weak();
            animate_heart_up(&mut commands, entity, heart_image.0);
            started_tweens.1 = true;
            return;
        }
    }
}
