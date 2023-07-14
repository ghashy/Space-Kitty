use bevy::{
    a11y::{
        accesskit::{NodeBuilder, Role},
        AccessibilityNode,
    },
    input::mouse::MouseWheel,
    prelude::*,
};

use crate::common::{
    game::score::resources::HighScores,
    gameover::{
        components::{GameoverComponent, ScrollView},
        styles::{
            bag, board_fill, board_frame, cat_face, dog_face, emitting_fill,
            emitting_frame, left_button, left_container, main_container,
            right_button, right_container, row, scroll_parent, scroll_view,
            spaceship,
        },
    },
    main_menu::components::{PlayButton, QuitButton},
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
            style: main_container(),
            ..default()
        }, 
        GameoverComponent
        ))
        // Central part with spaceship
        .with_children(|parent| {
            // Left container with button
            parent.spawn(NodeBundle {
                // background_color: BackgroundColor(Color::AZURE),
                style: left_container(),
                ..default()
            }).with_children(|parent| {
                // Left button
                #[cfg(not(target_arch = "wasm32"))]
                {
                    parent.spawn((ButtonBundle {
                        image: UiImage::new(asset_server.load("sprites/Gameover/Buttons/Quit default.png")),
                        // background_color: BackgroundColor(Color::GOLD),
                        style: left_button(),
                        ..default()
                    },
                    QuitButton {
                        default_handle: asset_server.load("sprites/Gameover/Buttons/Quit default.png"),
                        hover_handle: asset_server.load("sprites/Gameover/Buttons/Quit hovered.png"),
                        click_handle: asset_server.load("sprites/Gameover/Buttons/Quit clicked.png"),
                    },
                    ));
                }
            });
            parent
                .spawn(ImageBundle {
                    style: spaceship(),
                    image: UiImage::new(
                        asset_server
                            .load("sprites/Gameover/Starship upper part.png"),
                    ),
                    ..default()
                })
                .with_children(|parent| {
                    parent
                        .spawn(ImageBundle {
                            style: emitting_frame(),
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
                                    style: emitting_fill(),
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
                                            style: board_frame(),
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
                                                    style: board_fill(),
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
                                                                style: scroll_parent(),
                                                                ..default()
                                                            },
                                                        ).with_children(|parent| {
                                                            parent.
                                                                spawn((
                                                                    NodeBundle {
                                                                        style: scroll_view(),
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
            // Right container with button
            parent.spawn(NodeBundle {
                style: right_container(),
                ..default()
            }).with_children(|parent| {
                    // Right button
                    parent.spawn((ButtonBundle {
                        image: UiImage::new(asset_server.load("sprites/Gameover/Buttons/Retry default.png")),
                        style: right_button(),
                        ..default()
                    },
                    PlayButton {
                        default_handle: asset_server.load("sprites/Gameover/Buttons/Retry default.png"),
                        hover_handle: asset_server.load("sprites/Gameover/Buttons/Retry hovered.png"),
                        click_handle: asset_server.load("sprites/Gameover/Buttons/Retry clicked.png"),
                    },
                    ));
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
    // Row
    parent
        .spawn((
            NodeBundle {
                style: row(),
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
                    width: Val::Px(35.),
                    ..Style::DEFAULT
                },
                ..default()
            });
            // Entity's image
            // Handle Kitty's image case
            if name.to_string() == "Kitty" {
                image = asset_server.load("sprites/Cat's face blue.png");
                parent.spawn(ImageBundle {
                    image: UiImage::new(image),
                    background_color: BackgroundColor(Color::rgb(0., 0.93, 1.)),
                    style: cat_face(),
                    ..default()
                });
            } else {
                // Handle dogs
                parent
                    .spawn(ImageBundle {
                        image: UiImage::new(image),
                        background_color: BackgroundColor(Color::rgb(
                            0., 0.93, 1.,
                        )),
                        style: Style {
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            ..dog_face()
                        },
                        ..default()
                    })
                    // Spacesuit
                    .with_children(|parent| {
                        parent
                            .spawn(ImageBundle {
                                image: UiImage::new(
                                    asset_server
                                        .load("sprites/Dog's spacesuit.png"),
                                ),
                                background_color: BackgroundColor(Color::rgb(
                                    0., 0.93, 1.,
                                )),
                                style: Style {
                                    left: Val::Px(-20.6),
                                    top: Val::Px(-15.7),
                                    width: Val::Px(96.1),
                                    height: Val::Px(95.8),
                                    ..Style::DEFAULT
                                },
                                ..default()
                            })
                            .with_children(|parent| {
                                parent.spawn(ImageBundle {
                                    image: UiImage::new(
                                        asset_server.load(
                                            "sprites/Light reflection.png",
                                        ),
                                    ),
                                    background_color: BackgroundColor(
                                        Color::rgb(0., 0.93, 1.),
                                    ),
                                    style: Style {
                                        left: Val::Px(5.9),
                                        top: Val::Px(7.5),
                                        width: Val::Px(44.3),
                                        height: Val::Px(50.2),
                                        ..Style::DEFAULT
                                    },
                                    ..default()
                                });
                            });
                    });
            }
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
                    width: Val::Px(100.),
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
                    style: bag(),
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
            style.top = Val::Px(scrolling_list.position);
        }
    }
}
