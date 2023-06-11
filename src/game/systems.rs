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

pub fn update_borders(
    mut commands: Commands,
    mut window_resize_event: EventReader<WindowResized>,
    mut walls_query: Query<(&mut Transform, Entity, &Wall)>,
    camera_query: Query<
        (&Transform, &OrthographicProjection),
        (With<Camera2d>, Without<Wall>),
    >,
    mut local_timer: Local<(Timer, f32, f32)>,
    mut false_on_start: Local<bool>,
    time: Res<Time>,
) {
    if !*false_on_start {
        *false_on_start = true;
        local_timer.0.pause();
    }

    if let Some(event) = window_resize_event.iter().last() {
        // Start timer
        local_timer.1 = event.width;
        local_timer.2 = event.height;
        local_timer.0.set_mode(TimerMode::Once);
        local_timer
            .0
            .set_duration(std::time::Duration::from_millis(1000));
        local_timer.0.reset();
        local_timer.0.tick(time.delta());
        local_timer.0.unpause();
    }

    if !local_timer.0.finished() && !local_timer.0.paused() {
        local_timer.0.tick(time.delta());

        let horizontal = Collider::cuboid(local_timer.1, 2.);
        let vertical = Collider::cuboid(2., local_timer.2);
        let (cam_transform, cam_projection) = camera_query.single();
        let (left, right, top, bottom) =
            get_camera_borders(cam_transform, cam_projection.area);

        for (mut wall_transform, entity, wall) in walls_query.iter_mut() {
            match wall {
                Wall::Top => {
                    wall_transform.translation = top;
                    commands.entity(entity).insert(horizontal.clone());
                }
                Wall::Bottom => {
                    wall_transform.translation = bottom;
                    commands.entity(entity).insert(horizontal.clone());
                }
                Wall::Left => {
                    wall_transform.translation = left;
                    commands.entity(entity).insert(vertical.clone());
                }
                Wall::Right => {
                    wall_transform.translation = right;
                    commands.entity(entity).insert(vertical.clone());
                }
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
