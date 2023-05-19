use bevy::{prelude::*, window::PrimaryWindow};
use bevy_rapier2d::prelude::*;

// ----- Modules ------------------------------------------------------------ //

use super::{components::Wall, SimulationState};
use crate::game::components::BackgroundTexture;

// ----- Body --------------------------------------------------------------- //

pub fn pause_simulation(
    mut simulation_state_next_state: ResMut<NextState<SimulationState>>,
) {
    simulation_state_next_state.set(SimulationState::Paused);
}

pub fn resume_simulation(
    mut simulation_state_next_state: ResMut<NextState<SimulationState>>,
) {
    simulation_state_next_state.set(SimulationState::Running);
}

pub fn toggle_simulation_on_input_event(
    mut commands: Commands,
    keyboard_input: Res<Input<KeyCode>>,
    simulation_state: Res<State<SimulationState>>,
) {
    if keyboard_input.just_pressed(KeyCode::Space) {
        if simulation_state.0 == SimulationState::Running {
            commands.insert_resource(NextState(Some(SimulationState::Paused)));
            println!("Simulation paused");
        }

        if simulation_state.0 == SimulationState::Paused {
            commands.insert_resource(NextState(Some(SimulationState::Running)));
            println!("Simulation running");
        }
    }
}

pub fn spawn_world_borders(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window_query.get_single().unwrap();

    let width = Collider::cuboid(window.width() * 2., 2.);
    let height = Collider::cuboid(2., window.height() * 2.);

    // Top border
    commands.spawn((
        SpatialBundle {
            transform: Transform {
                translation: Vec3::new(
                    // X axis
                    window.width(),
                    // Y axis
                    window.height() * 2.,
                    0.,
                ),
                ..default()
            },
            ..default()
        },
        width.clone(),
        Wall,
    ));

    // Bottom border
    commands.spawn((
        SpatialBundle {
            transform: Transform {
                translation: Vec3::new(
                    // X axis
                    window.width(),
                    // Y axis
                    0.,
                    0.,
                ),
                ..default()
            },
            ..default()
        },
        width,
        Wall,
    ));

    // Left border
    commands.spawn((
        SpatialBundle {
            transform: Transform {
                translation: Vec3::new(
                    // X axis
                    0.,
                    // Y axis
                    window.height() * 2.,
                    0.,
                ),
                ..default()
            },
            ..default()
        },
        height.clone(),
        Wall,
    ));

    // Left border
    commands.spawn((
        SpatialBundle {
            transform: Transform {
                translation: Vec3::new(
                    // X axis
                    window.width() * 2.,
                    // Y axis
                    0.,
                    0.,
                ),
                ..default()
            },
            ..default()
        },
        height,
        Wall,
    ));
}

pub fn despawn_borders(
    mut commands: Commands,
    borders_query: Query<Entity, With<Wall>>,
) {
    for border in borders_query.iter() {
        commands.entity(border).despawn();
    }
}

pub fn spawn_background(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window_query.get_single().unwrap();
    let image: Handle<Image> =
        asset_server.load("sprites/background_green.png");

    let width = window.width() / 128. * 2.;
    let height = window.height() / 128. * 2. + 1.;

    let mut background_sprites = vec![];

    for row in 0..width as u32 {
        for column in 0..height as u32 {
            let sprite = commands
                .spawn(SpriteBundle {
                    texture: image.clone(),
                    transform: Transform::from_translation(Vec3::new(
                        64. + 128. * row as f32,
                        54. + 128. * column as f32,
                        -100.,
                    )),
                    sprite: Sprite {
                        color: Color::rgba(0.3, 0.3, 0.4, 1.),
                        ..default()
                    },
                    ..default()
                })
                .id();
            background_sprites.push(sprite);
        }
    }

    commands
        .spawn(SpatialBundle::default())
        .insert(BackgroundTexture)
        .insert(Name::new("BackgroundTexture"))
        .push_children(&background_sprites);
}

pub fn despawn_background(
    mut commands: Commands,
    background_sprite_query: Query<Entity, With<BackgroundTexture>>,
) {
    commands
        .entity(background_sprite_query.single())
        .despawn_recursive();
}
