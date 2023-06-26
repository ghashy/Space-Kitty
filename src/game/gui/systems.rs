use bevy::prelude::*;
use bevy_tweening::TweenCompleted;

// ───── Current Crate Import ─────────────────────────────────────────────── //

use super::animation::animate_heart_out;
use super::{components::*, styles::*, CHART_SIZE, LIVES_ID_OFFSET};
use crate::game::enemy::EnemyIsArrivingEvent;
use crate::game::player::LIVES_COUNT;
use crate::game::score::resources::Chart;
use crate::game::score::ScoreUpdateEvent;
use crate::{events::PlayerHit, game::player::components::Player};

// ───── Body ─────────────────────────────────────────────────────────────── //

pub fn spawn_hud(mut commands: Commands, asset_server: Res<AssetServer>) {
    let img1 = asset_server.load("sprites/Starship - life.png");
    let img2 = asset_server.load("sprites/Starship - no life.png");

    commands
        .spawn((
            NodeBundle {
                style: HUD_CONTAINER,
                ..default()
            },
            Hud,
        ))
        .with_children(|parent| {
            // LEFT SCREEN SIDE
            parent
                .spawn(NodeBundle {
                    // background_color: BackgroundColor(Color::CYAN.with_a(0.5)),
                    style: LEFT_SIDE_HUD_CONTAINER,
                    ..default()
                })
                .with_children(|parent| {
                    parent
                        .spawn((
                            NodeBundle {
                                style: CHART,
                                ..default()
                            },
                            ChartBlock,
                        ))
                        .with_children(|parent| {
                            // Items
                            for _ in 0..CHART_SIZE {
                                spawn_row(parent, &asset_server);
                            }
                        });
                });
            // RIGHT SCREEN SIDE
            parent
                .spawn(NodeBundle {
                    style: RIGHT_SIDE_HUD_CONTAINER,
                    // background_color: BackgroundColor(Color::RED),
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
                                id += LIVES_ID_OFFSET;
                                parent.spawn((
                                    ImageBundle {
                                        style: STARSHIP_LIFE,
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

fn spawn_row(parent: &mut ChildBuilder, asset_server: &Res<AssetServer>) {
    parent
        .spawn(NodeBundle {
            style: ITEM, // row
            ..default()
        })
        .with_children(|parent| {
            parent
                .spawn(ImageBundle {
                    image: UiImage {
                        texture: asset_server.load("sprites/Frame.png"),
                        ..default()
                    },
                    style: ITEM_IMAGE_BACK,
                    ..default()
                })
                .with_children(|parent| {
                    parent
                        .spawn((
                            ImageBundle {
                                style: ITEM_IMAGE_CONTENT,
                                image: UiImage {
                                    texture: asset_server
                                        .load("sprites/Frame.png"),
                                    ..default()
                                },
                                ..default()
                            },
                            TopImageMarker,
                        ))
                        .with_children(|parent| {
                            parent.spawn(ImageBundle {
                                image: UiImage {
                                    texture: asset_server
                                        .load("sprites/Frame outline.png"),
                                    ..default()
                                },
                                style: ITEM_IMAGE_OUTLINE,
                                ..default()
                            });
                        });
                });
            parent.spawn((
                TextBundle {
                    style: ITEM_TEXT,
                    text: Text::from_section(
                        "Kitty",
                        TextStyle {
                            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                            font_size: 24.,
                            color: Color::WHITE,
                        },
                    ),
                    ..default()
                },
                TopTextMarker,
            ));
            parent.spawn(ImageBundle {
                image: UiImage {
                    texture: asset_server.load("sprites/Fish for score.png"),
                    ..default()
                },
                style: ITEM_FISH_IMAGE,
                ..default()
            });
        });
}

pub fn update_chart(mut commands: Commands, chart: Res<Chart>) {
    if chart.is_changed() {
        println!("Chart is updated")
    }
}

pub fn update_messages(
    mut commands: Commands,
    list: Query<Entity, With<MessagesList>>,
    asset_server: Res<AssetServer>,
    mut score_update_event: EventReader<ScoreUpdateEvent>,
    mut arriving_events: EventReader<EnemyIsArrivingEvent>,
) {
    for event in score_update_event.iter() {
        let suffix = match event.event_type.get_score() % 10 {
            1 => "st",
            2 => "nd",
            3 => "d",
            _ => "th",
        };
        let label = (
            TextBundle::from_sections([
                TextSection::new(
                    format!("{}", &event.name),
                    TextStyle {
                        font: asset_server.load("fonts/Abaddon Bold.ttf"),
                        font_size: 25.,
                        color: Color::GREEN,
                    },
                ),
                TextSection::new(
                    format!(
                        " got his {}{} cracker!",
                        event.event_type.get_score(),
                        suffix
                    ),
                    TextStyle {
                        font: asset_server.load("fonts/Abaddon Bold.ttf"),
                        font_size: 25.,
                        color: Color::WHITE,
                    },
                ),
            ]),
            Message(Timer::new(
                std::time::Duration::from_secs(3),
                TimerMode::Once,
            )),
        );
        let id = commands.spawn(label).id();
        commands.entity(list.single()).push_children(&[id]);
    }
    for event in arriving_events.iter() {
        let label = (
            TextBundle::from_sections([
                TextSection::new(
                    format!("{}", &event.0),
                    TextStyle {
                        font: asset_server.load("fonts/Abaddon Bold.ttf"),
                        font_size: 25.,
                        color: Color::GREEN,
                    },
                ),
                TextSection::new(
                    " is ",
                    TextStyle {
                        font: asset_server.load("fonts/Abaddon Bold.ttf"),
                        font_size: 25.,
                        color: Color::WHITE,
                    },
                ),
                TextSection::new(
                    "arriving!",
                    TextStyle {
                        font: asset_server.load("fonts/Abaddon Bold.ttf"),
                        font_size: 25.,
                        color: Color::ORANGE,
                    },
                ),
            ]),
            Message(Timer::new(
                std::time::Duration::from_secs(3),
                TimerMode::Once,
            )),
        );
        let id = commands.spawn(label).id();
        commands.entity(list.single()).push_children(&[id]);
    }
}

pub fn remove_message_on_timeout(
    mut commands: Commands,
    mut labels_query: Query<(Entity, &mut Message, &Parent)>,
    time: Res<Time>,
) {
    for (entity, mut message, parent) in labels_query.iter_mut() {
        if message.0.tick(time.delta()).just_finished() {
            commands.entity(parent.get()).remove_children(&[entity]);
            commands.entity(entity).despawn();
        }
    }
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
    mut commands: Commands,
    mut player_hit_events: EventReader<PlayerHit>,
    mut animation_events: EventReader<TweenCompleted>,
    player_query: Query<&Player>,
    mut heart_images: Query<(Entity, &mut UiImage, &HeartImage)>,
) {
    if let Some(_) = player_hit_events.iter().next() {
        if let Ok(player) = player_query.get_single() {
            let id = player.health as u64 + LIVES_ID_OFFSET + 1;
            let entity = heart_images
                .iter_mut()
                .filter(|(_, _, heart_image)| heart_image.0 == id)
                .next()
                .unwrap()
                .0;
            animate_heart_out(&mut commands, entity, id);
        } else {
            let id = LIVES_ID_OFFSET + 1;
            let entity = heart_images
                .iter_mut()
                .filter(|(_, _, heart_image)| heart_image.0 == id)
                .next()
                .unwrap()
                .0;
            animate_heart_out(&mut commands, entity, id);
        }
    }

    for event in animation_events.iter() {
        if (400..500).contains(&event.user_data) {
            let mut our_image = heart_images
                .iter_mut()
                .filter(|(_, _, heart_image)| heart_image.0 == event.user_data)
                .next()
                .unwrap();
            our_image.1.texture = (our_image.2).2.clone_weak();
            return;
        }
    }
}
