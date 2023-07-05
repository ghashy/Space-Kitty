use std::time::Duration;

use bevy::{prelude::*, window::PrimaryWindow};
use bevy_rapier2d::prelude::*;
use kira::{clock::ClockSpeed, sound::static_sound::StaticSoundSettings};

// ───── Current Crate Imports ────────────────────────────────────────────── //

use crate::{
    audio::{
        assets::AudioSource,
        resources::{KiraManager, SamplePack, SoundHandleResource},
    },
    helper_functions::get_camera_borders,
    AppState,
};

use super::{
    components::{ControlsSheet, Wall},
    enemy::DoggyTheme,
    resources::GameData,
    SimulationState,
};

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

pub fn spawn_controls_sheet(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    camera_query: Query<&Transform, With<Camera2d>>,
    game_res: Res<GameData>,
) {
    if game_res.sheet_was_shown {
        return;
    }

    let mut position = camera_query.single().translation;
    position.y -= camera_query.single().translation.y / 2.;
    commands.spawn((
        SpriteBundle {
            transform: Transform::from_translation(position),
            sprite: Sprite {
                custom_size: Some(Vec2::new(1532., 208.) * 0.5),
                ..default()
            },
            texture: asset_server.load("sprites/Prompt.png"),
            ..default()
        },
        ControlsSheet,
    ));
}

pub fn detect_input(
    input: Res<Input<KeyCode>>,
    mut game_res: ResMut<GameData>,
) {
    // If we got any input, sheet now was shown
    if let Some(_) = input.get_just_pressed().next() {
        game_res.sheet_was_shown = true;
    }
}

pub fn despawn_controls_sheet(
    mut commands: Commands,
    mut controls_sheet_query: Query<(Entity, &mut Sprite), With<ControlsSheet>>,
    game_res: Res<GameData>,
    time: Res<Time>,
    next_state: Res<NextState<AppState>>,
) {
    if let Ok((entity, mut sprite)) = controls_sheet_query.get_single_mut() {
        // Despawn sheet if it was shown
        if game_res.sheet_was_shown {
            let alpha = sprite.color.a() - time.delta_seconds();
            if alpha < 0. {
                commands.entity(entity).despawn();
                println!("Despawned");
                return;
            }
            sprite.color.set_a(alpha);
            // Despawn sheet on exit from game state
        } else if next_state.0.is_some_and(|state| state != AppState::Game) {
            commands.entity(entity).despawn();
        }
    }
}

pub fn system_play_main_theme(
    kira_manager: NonSendMut<KiraManager>,
    audio_assets: Res<Assets<AudioSource>>,
    sample_pack: Res<SamplePack>,
    sound_handle: ResMut<SoundHandleResource>,
) {
    play_main_theme(kira_manager, audio_assets, sample_pack, sound_handle);
}

pub fn system_restart_clock(
    kira_manager: NonSendMut<KiraManager>,
    audio_assets: Res<Assets<AudioSource>>,
    sample_pack: Res<SamplePack>,
    sound_handle: ResMut<SoundHandleResource>,
) {
    if let Some(ref handle) = sound_handle.main_theme {
        if handle.state() == kira::sound::PlaybackState::Stopped {
            play_main_theme(
                kira_manager,
                audio_assets,
                sample_pack,
                sound_handle,
            );
        }
    }
}

fn play_main_theme(
    mut kira_manager: NonSendMut<KiraManager>,
    audio_assets: Res<Assets<AudioSource>>,
    sample_pack: Res<SamplePack>,
    mut sound_handle: ResMut<SoundHandleResource>,
) {
    const TEMPO: f64 = 115.;
    let clock = kira_manager
        .add_clock(ClockSpeed::TicksPerMinute(TEMPO))
        .unwrap();
    let sound_data = audio_assets
        .get(&sample_pack.main_theme)
        .unwrap()
        .get()
        .with_settings(
            StaticSoundSettings::new()
                .volume(0.7)
                .start_time(clock.time())
                .output_destination(kira_manager.get_master()),
        );
    let handle = kira_manager.play(sound_data).unwrap();
    // handle.set_loop_region(..).unwrap();
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
