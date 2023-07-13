use std::time::Duration;

use bevy::prelude::*;
use bevy::utils::HashMap;
use bevy_tweening::lens::UiPositionLens;
use bevy_tweening::{Animator, EaseFunction, Tween, TweenCompleted};

// ───── Current Crate Import ─────────────────────────────────────────────── //

use super::animation::animate_heart_out;
use super::{components::*, styles::*, HIT_EVENTS_OFFSET, REGEN_EVENTS_OFFSET};
use crate::common::game::enemy::EnemyIsArrivingEvent;
use crate::common::game::player::LIVES_COUNT;
use crate::common::game::regeneration::{
    MilkEscapedEvent, RegeneratePlayerEvent,
};
use crate::common::game::score::resources::Chart;
use crate::common::game::score::{ScoreEventType, ScoreUpdateEvent};
use crate::common::{events::PlayerHit, game::player::components::Player};

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
                    style: LEFT_SIDE_HUD_CONTAINER,
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn((
                        NodeBundle {
                            style: CHART,
                            ..default()
                        },
                        ChartBlock {
                            entities: HashMap::new(),
                        },
                    ));
                });
            // RIGHT SCREEN SIDE
            parent
                .spawn(NodeBundle {
                    style: RIGHT_SIDE_HUD_CONTAINER,
                    ..default()
                })
                .with_children(|parent| {
                    parent
                        .spawn(NodeBundle {
                            style: HEARTS_ROW,
                            ..default()
                        })
                        .with_children(|parent| {
                            for mut id in 1..=LIVES_COUNT {
                                id += HIT_EVENTS_OFFSET;
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
                            style: MESSAGES_BAR,
                            ..default()
                        },
                        MessagesList,
                    ));
                });
        });
}

fn spawn_row(
    parent: &mut ChildBuilder,
    asset_server: &Res<AssetServer>,
    text: &str,
    texture: Handle<Image>,
    content_style: Style,
    pos: usize,
) -> Entity {
    parent
        .spawn((
            NodeBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    position: UiRect::top(Val::Px(pos as f32 * 75.)),
                    display: Display::Flex,
                    flex_direction: FlexDirection::Row,
                    justify_content: JustifyContent::SpaceBetween,
                    align_items: AlignItems::Center,
                    ..Style::DEFAULT
                },
                ..default()
            },
            ChartRow {
                idx: pos,
                entity: None,
            },
        ))
        .with_children(|parent| {
            parent.spawn((ImageBundle {
                style: content_style,
                image: UiImage {
                    texture,
                    ..default()
                },
                ..default()
            },));
            parent.spawn((TextBundle {
                style: ITEM_TEXT,
                text: Text::from_section(
                    text,
                    TextStyle {
                        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                        font_size: 20.,
                        color: Color::WHITE,
                    },
                ),
                ..default()
            },));
        })
        .id()
}

/// `drawn_block` contains pairs: K: Entity (of character), V: Entity (of UI)
pub fn spawn_rows_from_backend(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    chart: Res<Chart>,
    mut chart_block: Query<(Entity, &mut ChartBlock)>,
    mut chart_rows: Query<(&mut ChartRow, &Children)>,
    mut texts: Query<&mut Text>,
) {
    if chart.is_changed() {
        let (block_entity, mut drawn_block) = chart_block.single_mut();
        let mut removed_keys = Vec::new();

        // Figure out which entities are not exist now, despawn their rows
        for (character, ui) in drawn_block.entities.iter() {
            if let None = chart.get_pos(*character) {
                commands.entity(*ui).despawn_recursive();
                removed_keys.push(*character);
            } else if let Some(pos) = chart.get_pos(*character) {
                if pos > 2 {
                    commands.entity(*ui).despawn_recursive();
                    removed_keys.push(*character);
                }
            }
        }

        // Remove unexistent entities from `drawn_block`
        removed_keys.iter().for_each(|key| {
            drawn_block.entities.remove(key);
        });

        // Iterate chart from backend, and spawn if not spawned rows from chart
        for (idx, item) in chart.lines.iter().enumerate() {
            if !drawn_block.entities.contains_key(&item.entity) && idx < 3 {
                commands.entity(block_entity).with_children(|parent| {
                    let row_id = spawn_row(
                        parent,
                        &asset_server,
                        &(item.name.to_string()
                            + ": "
                            + &item.score.to_string()),
                        item.image.clone(),
                        AVATAR,
                        idx,
                    );
                    // Push this entity to list of drawn entities
                    drawn_block.entities.insert(item.entity, row_id);
                });
            }
        }

        // Correct rows positions, update scores
        for (character, ui) in drawn_block.entities.iter() {
            if let Some(idx) = chart.get_pos(*character) {
                if let Ok((mut chart_row, child)) = chart_rows.get_mut(*ui) {
                    // Update scores
                    let scoreline = chart
                        .get_line(idx)
                        .expect("Error get ScoreLine from chart");

                    let mut text = texts
                        .get_mut(*child.iter().take(2).last().unwrap())
                        .unwrap();

                    text.sections = vec![TextSection {
                        value: scoreline.name.to_string()
                            + ": "
                            + &scoreline.score.to_string(),
                        style: TextStyle {
                            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                            font_size: 20.,
                            color: Color::WHITE,
                        },
                    }];

                    // Correct positions
                    if chart_row.idx != idx {
                        let tween = Tween::new(
                            EaseFunction::QuadraticInOut,
                            Duration::from_millis(700),
                            UiPositionLens {
                                start: UiRect::top(Val::Px(
                                    chart_row.idx as f32 * 75.,
                                )),
                                end: UiRect::top(Val::Px(idx as f32 * 75.)),
                            },
                        );
                        chart_row.idx = idx;
                        commands.entity(*ui).insert(Animator::new(tween));
                    }
                }
            }
        }
    }
}

pub fn update_messages(
    mut commands: Commands,
    list: Query<Entity, With<MessagesList>>,
    asset_server: Res<AssetServer>,
    mut score_update_event: EventReader<ScoreUpdateEvent>,
    mut arriving_events: EventReader<EnemyIsArrivingEvent>,
    mut escaped_milk_events: EventReader<MilkEscapedEvent>,
    mut regen_events: EventReader<RegeneratePlayerEvent>,
) {
    for event in score_update_event.iter() {
        let suffix = match event.event_type.get_score() % 10 {
            1 => "st",
            2 => "nd",
            3 => "d",
            _ => "th",
        };
        let message = match event.event_type {
            ScoreEventType::ScoreDrop(how_much) => {
                let ending = if how_much == 1 { "" } else { "s" };
                format!(" dropped {} cracker{}!", how_much, ending)
            }
            _ => {
                format!(
                    " got his {}{} cracker!",
                    event.event_type.get_score(),
                    suffix
                )
            }
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
                    message,
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
    for _ in escaped_milk_events.iter() {
        let label = (
            TextBundle::from_sections([TextSection::new(
                "The milk escaped!",
                TextStyle {
                    font: asset_server.load("fonts/Abaddon Bold.ttf"),
                    font_size: 25.,
                    color: Color::RED,
                },
            )]),
            Message(Timer::new(
                std::time::Duration::from_secs(3),
                TimerMode::Once,
            )),
        );
        let id = commands.spawn(label).id();
        commands.entity(list.single()).push_children(&[id]);
    }
    for _ in regen_events.iter() {
        let label = (
            TextBundle::from_sections([TextSection::new(
                "The milk'd been drinked!",
                TextStyle {
                    font: asset_server.load("fonts/Abaddon Bold.ttf"),
                    font_size: 25.,
                    color: Color::GREEN,
                },
            )]),
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
            let id = player.health as u64 + HIT_EVENTS_OFFSET + 1;
            let entity = heart_images
                .iter_mut()
                .filter(|(_, _, heart_image)| heart_image.0 == id)
                .next()
                .unwrap()
                .0;
            animate_heart_out(&mut commands, entity, id);
        } else {
            let id = HIT_EVENTS_OFFSET + 1;
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
        if (HIT_EVENTS_OFFSET..REGEN_EVENTS_OFFSET).contains(&event.user_data) {
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

// FIXME
// Some very bad decisions here, but I can't figure out how
// to organize work with tweening and events for now.
pub fn listen_regeneration_events(
    mut commands: Commands,
    mut regen_events: EventReader<RegeneratePlayerEvent>,
    mut animation_events: EventReader<TweenCompleted>,
    player_query: Query<&Player>,
    mut heart_images: Query<(Entity, &mut UiImage, &HeartImage)>,
) {
    if let Some(event) = regen_events.iter().next() {
        if let Ok(_) = player_query.get_single() {
            // Some bad desicion, use `HIT_EVENTS_OFFSET` here
            let id1 = event.new_health as u64 + HIT_EVENTS_OFFSET;
            let entity = heart_images
                .iter_mut()
                .filter(|(_, _, heart_image)| heart_image.0 == id1)
                .next()
                .unwrap()
                .0;

            let id2 = REGEN_EVENTS_OFFSET + event.new_health as u64;
            animate_heart_out(&mut commands, entity, id2);
        }
    }

    for event in animation_events.iter() {
        if (REGEN_EVENTS_OFFSET..500).contains(&event.user_data) {
            let mut our_image = heart_images
                .iter_mut()
                .filter(|(_, _, heart_image)| {
                    heart_image.0 == event.user_data - 50
                })
                .next()
                .unwrap();
            our_image.1.texture = (our_image.2).1.clone_weak();
            return;
        }
    }
}
