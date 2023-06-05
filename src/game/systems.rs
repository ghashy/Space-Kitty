use bevy::{prelude::*, window::PrimaryWindow};
use bevy_rapier2d::prelude::*;

// ───── Current Crate Imports ────────────────────────────────────────────── //

use super::{components::Wall, SimulationState};

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

pub fn spawn_world_borders(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window_query.get_single().unwrap();

    let horizontal = Collider::cuboid(window.width(), 2.);
    let vertical = Collider::cuboid(2., window.height());

    // Top border
    commands.spawn((
        SpatialBundle {
            transform: Transform {
                translation: Vec3::new(window.width(), window.height(), 0.),
                ..default()
            },
            ..default()
        },
        horizontal.clone(),
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
        horizontal,
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
                    window.height(),
                    0.,
                ),
                ..default()
            },
            ..default()
        },
        vertical.clone(),
        Wall,
    ));

    // Left border
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
        vertical,
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
