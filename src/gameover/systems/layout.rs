use bevy::{
    a11y::{
        accesskit::{NodeBuilder, Role},
        AccessibilityNode,
    },
    input::mouse::MouseWheel,
    prelude::*,
    window::PrimaryWindow,
};

use crate::{
    game::score::resources::HighScores,
    gameover::{
        components::{GameoverComponent, ScrollView},
        styles::{
            BAG, BOARD_FILL, BOARD_FRAME, CAT_FACE, DOG_FACE, EMITTING_FILL,
            EMITTING_FRAME, MAIN_CONTAINER, ROW, SCROLL_PARENT, SCROLL_VIEW,
            SPACESHIP,
        },
    },
};

// ───── Body ─────────────────────────────────────────────────────────────── //

#[rustfmt::skip]
pub fn spawn_gameover_layout(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    highscore: Res<HighScores>,
) {
    commands
        .spawn((NodeBundle {
            style: MAIN_CONTAINER,
            ..default()
        }, 
        GameoverComponent
        ))
        .with_children(|parent| {
            parent
                .spawn(ImageBundle {
                    style: SPACESHIP,
                    image: UiImage::new(
                        asset_server
                            .load("sprites/Gameover/Starship upper part.png"),
                    ),
                    ..default()
                })
                .with_children(|parent| {
                    parent
                        .spawn(ImageBundle {
                            style: EMITTING_FRAME,
                            image: UiImage::new(
                                asset_server.load(
                                    "sprites/Gameover/Emitting frame.png",
                                ),
                            ),
                            ..default()
                        })
                        .with_children(|parent| {
                            parent
                                .spawn(ImageBundle {
                                    style: EMITTING_FILL,
                                    background_color: BackgroundColor(
                                        Color::WHITE.with_a(0.35),
                                    ),
                                    image: UiImage::new(asset_server.load(
                                        "sprites/Gameover/Emitting fill.png",
                                    )),
                                    ..default()
                                })
                                .with_children(|parent| {
                                    parent
                                        .spawn(ImageBundle {
                                            style: BOARD_FRAME,
                                            background_color: BackgroundColor(
                                                Color::WHITE.with_a(0.8),
                                            ),
                                            image: UiImage::new(asset_server.load(
                                                "sprites/Gameover/Board frame.png",
                                            )),
                                            ..default()
                                        })
                                        .with_children(|parent| {
                                            parent
                                                .spawn(ImageBundle {
                                                    style: BOARD_FILL,
                                                    background_color: BackgroundColor(
                                                        Color::WHITE.with_a(0.3),
                                                    ),
                                                    image: UiImage::new(asset_server.load(
                                                        "sprites/Gameover/Board fill.png",
                                                    )),
                                                    ..default()
                                                })
                                                .with_children(|parent| {
                                                    // Gameover label
                                                    parent
                                                        .spawn(TextBundle {
                                                            style: Style::DEFAULT,
                                                            text: Text::from_section("Game over",
                                                                TextStyle {
                                                                    font: asset_server.load("fonts/NicoMoji-Regular.ttf"),
                                                                    font_size: 41.,
                                                                    color: Color::hex("23CED1").unwrap(),
                                                                    ..default()
                                                                }),
                                                            ..default()
                                                    });

                                                    // Scroll view
                                                    parent.
                                                        spawn(
                                                            NodeBundle {
                                                                style: SCROLL_PARENT,
                                                                ..default()
                                                            },
                                                        ).with_children(|parent| {
                                                            parent.
                                                                spawn((
                                                                    NodeBundle {
                                                                        style: SCROLL_VIEW,
                                                                        ..default()
                                                                    },
                                                                    ScrollView { position: 0. },
                                                                    AccessibilityNode(NodeBuilder::new(Role::List)),
                                                                )).with_children(|parent| {
                                                                       for (idx, (score, name, image)) in highscore.sorted().enumerate() {
                                                                            spawn_row(idx + 1, parent, name, image, score, &asset_server);
                                                                       }             
                                                            
                                                            });
                                                    });
                                                });
                                        });
                                });
                        });
                });
        });
}

pub fn despawn_gameover_layout(
    mut commands: Commands,
    gameover_component: Query<Entity, With<GameoverComponent>>,
) {
    commands
        .entity(gameover_component.single())
        .despawn_recursive();
}

fn spawn_row(
    idx: usize,
    parent: &mut ChildBuilder,
    name: Name,
    mut image: Handle<Image>,
    score: u32,
    asset_server: &Res<AssetServer>,
) {
    // Trim name from end if it contains `AKA`
    // if name.contains("aka").
    let mut entity_style = DOG_FACE;
    // Row
    parent
        .spawn((
            NodeBundle {
                style: ROW,
                ..default()
            },
            AccessibilityNode(NodeBuilder::new(Role::List)),
        ))
        .with_children(|parent| {
            // Chart index
            parent.spawn(TextBundle {
                text: Text::from_section(
                    idx.to_string() + ".",
                    TextStyle {
                        font: asset_server.load("fonts/NicoMoji-Regular.ttf"),
                        font_size: 25.,
                        color: Color::hex("23CED1").unwrap(),
                        ..default()
                    },
                ),
                style: Style {
                    margin: UiRect::horizontal(Val::Px(10.)),
                    ..Style::DEFAULT
                },
                ..default()
            });
            // Handle Kitty's image case
            if name.to_string() == "Kitty" {
                image = asset_server.load("sprites/Cat's face.png");
                entity_style = CAT_FACE;
            }
            // Entity's image
            parent.spawn(ImageBundle {
                image: UiImage::new(image),
                background_color: BackgroundColor(Color::rgb(0., 0.93, 1.)),
                style: entity_style,
                ..default()
            });
            // Entity's name
            parent.spawn(TextBundle {
                text: Text::from_section(
                    name.to_string(),
                    TextStyle {
                        font: asset_server.load("fonts/NicoMoji-Regular.ttf"),
                        font_size: 21.,
                        color: Color::hex("23CED1").unwrap(),
                        ..default()
                    },
                ),
                style: Style {
                    margin: UiRect {
                        left: Val::Px(25.),
                        right: Val::Px(50.),
                        ..default()
                    },
                    size: Size::width(Val::Px(100.)),
                    ..Style::DEFAULT
                },
                ..default()
            });
            // A bag
            parent
                .spawn(ImageBundle {
                    image: UiImage::new(
                        asset_server.load("sprites/Gameover/A bag.png"),
                    ),
                    style: BAG,
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn(TextBundle {
                        text: Text::from_section(
                            score.to_string(),
                            TextStyle {
                                font: asset_server
                                    .load("fonts/FiraSans-Bold.ttf"),
                                color: Color::hex("23CED1").unwrap(),
                                font_size: 22.,
                                ..default()
                            },
                        ),
                        style: Style { ..default() },
                        ..default()
                    });
                });
        });
}

// Temporary implementation
pub fn scroll_list(
    mut mouse_wheel_events: EventReader<MouseWheel>,
    mut query_list: Query<(&mut ScrollView, &mut Style, &Parent, &Node)>,
    query_node: Query<&Node>,
) {
    use bevy::input::mouse::MouseScrollUnit;

    for (mut scrolling_list, mut style, parent, node) in &mut query_list {
        let items_height = node.size().y;
        let contaier_height = query_node.get(parent.get()).unwrap().size().y;

        let max_scroll = (items_height - contaier_height).max(0.);
        for mouse_wheel_event in mouse_wheel_events.iter() {
            let dy = match mouse_wheel_event.unit {
                MouseScrollUnit::Line => mouse_wheel_event.y * 20.,
                MouseScrollUnit::Pixel => mouse_wheel_event.y,
            };
            scrolling_list.position += dy;
            scrolling_list.position =
                scrolling_list.position.clamp(-max_scroll, 0.);
            style.position.top = Val::Px(scrolling_list.position);
        }
    }
}
