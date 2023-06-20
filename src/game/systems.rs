use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

// ───── Current Crate Imports ────────────────────────────────────────────── //

use crate::{components::BackgroundImage, WORLD_MAX_EDGE, WORLD_MIN_EDGE};

use super::{components::Wall, player::components::Player, SimulationState};

// ───── Body ─────────────────────────────────────────────────────────────── //

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

pub fn spawn_world_borders(mut commands: Commands) {
    let left = Vec3::new(WORLD_MIN_EDGE, 0., 0.);
    let right = Vec3::new(WORLD_MAX_EDGE, 0., 0.);
    let top = Vec3::new(0., WORLD_MAX_EDGE, 0.);
    let bottom = Vec3::new(0., WORLD_MIN_EDGE, 0.);

    let horizontal = Collider::cuboid(WORLD_MAX_EDGE, 2.);
    let vertical = Collider::cuboid(2., WORLD_MAX_EDGE);

    // Top border
    commands.spawn((
        SpatialBundle {
            transform: Transform {
                translation: top,
                ..default()
            },
            ..default()
        },
        horizontal.clone(),
        Wall::Top,
    ));

    // Bottom border
    commands.spawn((
        SpatialBundle {
            transform: Transform {
                translation: bottom,
                ..default()
            },
            ..default()
        },
        horizontal,
        Wall::Bottom,
    ));

    // Left border
    commands.spawn((
        SpatialBundle {
            transform: Transform {
                translation: left,
                ..default()
            },
            ..default()
        },
        vertical.clone(),
        Wall::Left,
    ));

    // Right border
    commands.spawn((
        SpatialBundle {
            transform: Transform {
                translation: right,
                ..default()
            },
            ..default()
        },
        vertical,
        Wall::Right,
    ));
}

pub fn system_camera_follows_player(
    mut background_image_query: Query<&mut Transform, With<BackgroundImage>>,
    mut camera_query: Query<
        &mut Transform,
        (With<Camera2d>, Without<BackgroundImage>),
    >,
    player_query: Query<
        &Transform,
        (With<Player>, Without<Camera2d>, Without<BackgroundImage>),
    >,
    time: Res<Time>,
) {
    if let Ok(player) = player_query.get_single() {
        if let Ok(mut camera_transform) = camera_query.get_single_mut() {
            camera_transform.translation.x += (player.translation.x
                - camera_transform.translation.x)
                * time.delta_seconds()
                * 5.;
            camera_transform.translation.y += (player.translation.y
                - camera_transform.translation.y)
                * time.delta_seconds()
                * 5.;

            // camera_transform.translation.x = player.translation.x;
            // camera_transform.translation.y = player.translation.y;
            // camera_transform.translation = camera_transform
            //     .translation
            //     .truncate()
            //     .lerp(player.translation.truncate(), 0.1)
            //     .extend(camera_transform.translation.z);
            if let Ok(mut background_transform) =
                background_image_query.get_single_mut()
            {
                background_transform.translation.x =
                    camera_transform.translation.x;
                background_transform.translation.y =
                    camera_transform.translation.y;
            }
        }
    }
}

pub fn despawn_borders(
    mut commands: Commands,
    borders_query: Query<Entity, With<Wall>>,
) {
    for border in borders_query.iter() {
        commands.entity(border).despawn();
    }
}
