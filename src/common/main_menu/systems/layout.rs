use bevy::{prelude::*, window::PrimaryWindow};
use bevy_tweening::{lens::TransformPositionLens, *};

// ───── Current Crate Imports ────────────────────────────────────────────── //

use crate::common::{
    game::player::components::Player,
    main_menu::{components::*, styles::*},
};

// ───── Body ─────────────────────────────────────────────────────────────── //

// Layers hint:
// 0 - Background
// 1 - Background stars
// 3 - Planets
// 4 - Dust
// 5 - Gui

pub fn spawn_main_menu(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    build_planets_layer(&mut commands, &asset_server);
    build_main_menu(&mut commands, &asset_server);
    spawn_player(&mut commands, &asset_server, &window_query);
}

pub fn despawn_main_menu(
    mut commands: Commands,
    main_menu_query: Query<Entity, With<MainMenu>>,
    planets_layer_query: Query<Entity, With<PlanetsLayer>>,
    player_query: Query<Entity, With<Player>>,
) {
    if let Ok(main_menu_entity) = main_menu_query.get_single() {
        commands.entity(main_menu_entity).despawn_recursive();
    }
    if let Ok(planets_layer) = planets_layer_query.get_single() {
        commands.entity(planets_layer).despawn_recursive();
    }
    if let Ok(player) = player_query.get_single() {
        commands.entity(player).despawn_recursive();
    }
}

pub fn build_planets_layer(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
) -> Entity {
    let planets_layer = commands
        .spawn((
            NodeBundle {
                style: PLANETS_NODE,
                transform: Transform::from_xyz(0., 0., 3.),
                ..default()
            },
            PlanetsLayer,
        ))
        .with_children(|parent| {
            // Big planet
            parent.spawn(ImageBundle {
                style: PLANET_BIG,
                image: UiImage::new(
                    asset_server.load("sprites/Planet Big.png"),
                ),
                background_color: BackgroundColor::from(Color::rgba(
                    1., 1., 1., 1.,
                )),
                ..default()
            });
            // Main planet
            parent.spawn(ImageBundle {
                style: PLANET_MAIN,
                image: UiImage::new(
                    asset_server.load("sprites/Planet Main.png"),
                ),
                background_color: BackgroundColor::from(Color::rgba(
                    1., 1., 1., 1.,
                )),
                ..default()
            });
            // Small planet
            parent
                .spawn(ImageBundle {
                    style: PLANET_ATMOSPHERE,
                    image: UiImage::new(
                        asset_server.load("sprites/Planet atmosphere.png"),
                    ),
                    background_color: BackgroundColor::from(Color::rgba(
                        1., 1., 1., 0.3,
                    )),
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn(ImageBundle {
                        style: PLANET_SMALL,
                        image: UiImage::new(
                            asset_server.load("sprites/Planet Small.png"),
                        ),
                        background_color: BackgroundColor::from(Color::rgba(
                            1., 1., 1., 1.,
                        )),
                        ..default()
                    });
                });
        })
        .id();
    planets_layer
}

pub fn build_main_menu(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
) -> Entity {
    let main_menu_entity = commands
        .spawn((
            NodeBundle {
                style: MAIN_CONTAINER,
                transform: Transform::from_xyz(0., 0., 5.),
                ..default()
            },
            MainMenu {},
        ))
        .with_children(|parent| {
            // Left half of screen
            parent
                .spawn(NodeBundle {
                    style: HALF_SCREEN,
                    ..default()
                })
                .with_children(|parent| {
                    // Top part:
                    parent
                        .spawn(NodeBundle {
                            style: TOP_PART,
                            ..default()
                        })
                        .with_children(|parent| {
                            spawn_title(parent, asset_server);
                        });
                    // Bottom part
                    parent
                        .spawn(NodeBundle {
                            style: BOTTOM_PART,
                            ..default()
                        })
                        .with_children(|parent| {
                            spawn_play_button(parent, asset_server);

                            #[cfg(not(target_arch = "wasm32"))]
                            spawn_quit_button(parent, asset_server);
                        });
                });
            // Right half of screen
            parent.spawn(NodeBundle {
                style: HALF_SCREEN,
                ..default()
            });
        })
        .id();

    main_menu_entity
}

fn spawn_title(parent: &mut ChildBuilder, asset_server: &Res<AssetServer>) {
    parent
        .spawn(NodeBundle {
            style: TITLE_STYLE,
            ..default()
        })
        .with_children(|parent| {
            // Image
            parent.spawn(ImageBundle {
                style: IMAGE_STYLE,
                image: asset_server.load("sprites/Title.png").into(),
                ..default()
            });
        });
}

fn spawn_play_button(
    parent: &mut ChildBuilder,
    asset_server: &Res<AssetServer>,
) {
    let default_texture: Handle<Image> =
        asset_server.load("sprites/Play_Default.png");
    let hovered_texture: Handle<Image> =
        asset_server.load("sprites/Play_Hovered.png");
    let clicked_texture: Handle<Image> =
        asset_server.load("sprites/Play_Clicked.png");
    parent.spawn((
        ButtonBundle {
            style: NORMAL_BUTTON_STYLE,
            background_color: NORMAL_BUTTON_COLOR.into(),
            image: default_texture.clone().into(),
            ..default()
        },
        PlayButton {
            default_handle: default_texture,
            hover_handle: hovered_texture,
            click_handle: clicked_texture,
        },
    ));
}

fn spawn_quit_button(
    parent: &mut ChildBuilder,
    asset_server: &Res<AssetServer>,
) {
    let default_texture: Handle<Image> =
        asset_server.load("sprites/Quit_Default.png");
    let hovered_texture: Handle<Image> =
        asset_server.load("sprites/Quit_Hovered.png");
    let clicked_texture: Handle<Image> =
        asset_server.load("sprites/Quit_Hovered.png");
    parent.spawn((
        ButtonBundle {
            style: NORMAL_BUTTON_STYLE,
            background_color: NORMAL_BUTTON_COLOR.into(),
            image: default_texture.clone().into(),
            ..default()
        },
        QuitButton {
            default_handle: default_texture,
            hover_handle: hovered_texture,
            click_handle: clicked_texture,
        },
    ));
}

fn spawn_player(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    window_query: &Query<&Window, With<PrimaryWindow>>,
) {
    let tween = Tween::new(
        EaseFunction::QuadraticInOut,
        std::time::Duration::from_millis(2000),
        TransformPositionLens {
            start: Vec3::new(0., -10., 0.),
            end: Vec3::new(0., 10., 0.),
        },
    )
    .with_repeat_count(RepeatCount::Infinite)
    .with_repeat_strategy(RepeatStrategy::MirroredRepeat);

    let window = window_query.single();
    let x = window.resolution.width() / 4.9;
    let y = window.resolution.height() / 3.9;
    commands
        .spawn((
            SpatialBundle {
                transform: Transform::from_xyz(x, y, 10.),
                ..default()
            },
            Name::new("PlayerBox"),
            Player { health: 1 },
        ))
        .with_children(|parent| {
            parent.spawn((
                SpriteBundle {
                    sprite: Sprite {
                        custom_size: Some(Vec2::new(520., 622.) / 2.),
                        flip_x: true,
                        ..default()
                    },
                    texture: asset_server.load("sprites/Cat's starship_HQ.png"),
                    ..default()
                },
                Animator::new(tween),
            ));
        });
}

// ───── Unit tests ───────────────────────────────────────────────────────── //

#[cfg(test)]
mod tests {
    use bevy::prelude::*;
    use bevy_tweening::{lens::*, *};

    #[test]
    fn animation_states() {
        let mut app = App::new();
        let tween = Tween::new(
            EaseFunction::CubicOut,
            std::time::Duration::from_millis(500),
            TransformScaleLens {
                start: Vec3::splat(1.),
                end: Vec3::splat(2.),
            },
        );
        let animator = Animator::new(tween);
        let entity = app.world.spawn((SpatialBundle::default(), animator)).id();
        app.update();
        let mut animation = app.world.entity_mut(entity);
        let mut animation = animation.get_mut::<Animator<Transform>>().unwrap();
        assert_eq!(animation.state, AnimatorState::Playing);
        animation.stop();
        assert_eq!(animation.state, AnimatorState::Paused);
    }
}
