use std::time::Duration;

use bevy::{
    prelude::*,
    window::{PrimaryWindow, WindowResized},
};
use bevy_rapier2d::prelude::*;
use kira::{
    clock::{ClockSpeed, ClockTime},
    sound::static_sound::StaticSoundSettings,
};

// ───── Current Crate Imports ────────────────────────────────────────────── //

use crate::{
    audio::{
        assets::AudioSource,
        resources::{KiraManager, SamplePack, SoundHandleResource},
    },
    helper_functions::get_camera_borders,
};

use super::{components::Wall, enemy::DoggyTheme, SimulationState};

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
    camera_query: Query<(&Transform, &OrthographicProjection), With<Camera2d>>,
) {
    let window = window_query.single();

    let (cam_transform, cam_projection) = camera_query.single();

    let (left, right, top, bottom) =
        get_camera_borders(cam_transform, cam_projection.area);

    let horizontal = Collider::cuboid(window.width(), 2.);
    let vertical = Collider::cuboid(2., window.height());

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

pub fn despawn_borders(
    mut commands: Commands,
    borders_query: Query<Entity, With<Wall>>,
) {
    for border in borders_query.iter() {
        commands.entity(border).despawn();
    }
}

pub fn play_main_theme(
    mut kira_manager: NonSendMut<KiraManager>,
    audio_assets: Res<Assets<AudioSource>>,
    sample_pack: Res<SamplePack>,
    mut sound_handle: ResMut<SoundHandleResource>,
) {
    const TEMPO: f64 = 115.;
    let clock = kira_manager
        .add_clock(ClockSpeed::TicksPerMinute(TEMPO))
        .unwrap();
    let mut handle = kira_manager
        .play(
            audio_assets
                .get(&sample_pack.main_theme)
                .unwrap()
                .get()
                .with_settings(
                    StaticSoundSettings::new()
                        .volume(0.7)
                        .start_time(clock.time()),
                ),
        )
        .unwrap();
    handle.set_loop_region(..).unwrap();
    clock.start().unwrap();
    sound_handle.main_theme = Some(handle);
    sound_handle.main_theme_clock = Some(clock);
}

pub fn system_check_main_theme_clock(
    sound_handle: Res<SoundHandleResource>,
    mut local_counter: Local<u64>,
    mut doggy_theme_events: EventWriter<DoggyTheme>,
) {
    if let Some(ref clock) = sound_handle.main_theme_clock {
        let tick = clock.time().ticks;
        if (144..=176).contains(&tick) && *local_counter != tick {
            println!("Tick: {}", tick);
            *local_counter = tick;
            doggy_theme_events.send(DoggyTheme);
        }
    }
}

pub fn stop_main_theme(mut sound_handle: ResMut<SoundHandleResource>) {
    if let Some(ref mut handle) = sound_handle.main_theme {
        handle
            .stop(kira::tween::Tween {
                duration: Duration::from_millis(200),
                ..default()
            })
            .unwrap();

        if let Some(ref clock) = sound_handle.main_theme_clock {
            clock.stop().unwrap();
        }
    }
}
