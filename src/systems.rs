use bevy::{
    app::AppExit, prelude::*, render::camera::ScalingMode,
    window::PrimaryWindow,
};
use bevy_rapier2d::prelude::*;

// ----- Crate -------------------------------------------------------------- //

use crate::{
    audio_system::resources::SamplePack, events::*, game::SimulationState,
    AppState,
};

// ----- Body --------------------------------------------------------------- //

pub fn setup(
    mut commands: Commands,
    mut rapier_config: ResMut<RapierConfiguration>,
    asset_server: ResMut<AssetServer>,
) {
    // Setup physics gravity
    rapier_config.gravity = Vec2::ZERO;

    commands.insert_resource(SamplePack {
        imp_light_0: asset_server.load("audio/impact/Light_00.ogg"),
        imp_light_1: asset_server.load("audio/impact/Light_01.ogg"),
        imp_light_2: asset_server.load("audio/impact/Light_02.ogg"),
        imp_light_3: asset_server.load("audio/impact/Light_03.ogg"),
        imp_light_4: asset_server.load("audio/impact/Light_04.ogg"),
        imp_med_0: asset_server.load("audio/impact/Medium_00.ogg"),
        imp_med_1: asset_server.load("audio/impact/Medium_01.ogg"),
        exp: asset_server.load("audio/explosionCrunch_000.ogg"),
        pick_star: asset_server.load("audio/laserLarge_000.ogg"),
    });
}

pub fn spawn_camera(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window_query.get_single().unwrap();

    commands.spawn(Camera2dBundle {
        transform: Transform::from_xyz(window.width(), window.height(), 0.),
        projection: OrthographicProjection {
            scale: 2.,
            scaling_mode: ScalingMode::Fixed {
                width: window.width(),
                height: window.height(),
            },
            ..default()
        },
        ..default()
    });
}

pub fn transition_to_game_state(
    keyboard_input: Res<Input<KeyCode>>,
    app_state: Res<State<AppState>>,
    mut next_app_state: ResMut<NextState<AppState>>,
) {
    if keyboard_input.just_pressed(KeyCode::G) {
        if app_state.0 != AppState::Game {
            next_app_state.set(AppState::Game);
            println!("Entered AppState::Game");
        }
    }
}

pub fn transition_to_main_menu_state(
    keyboard_input: Res<Input<KeyCode>>,
    app_state: Res<State<AppState>>,
    mut next_app_state: ResMut<NextState<AppState>>,
) {
    if keyboard_input.just_pressed(KeyCode::M) {
        if app_state.0 != AppState::MainMenu {
            next_app_state.set(AppState::MainMenu);
            println!("Entered AppState::MainMenu");
        }
    }
}

pub fn exit_game(
    keyboard_input: Res<Input<KeyCode>>,
    mut event_writer: EventWriter<AppExit>,
) {
    if keyboard_input.pressed(KeyCode::Escape) {
        event_writer.send(AppExit);
    }
}

pub fn handle_game_over(
    mut commands: Commands,
    mut game_over_event_reader: EventReader<GameOver>,
    entities: Query<Entity>,
) {
    for event in game_over_event_reader.iter() {
        println!("FinalScore: {}", event.final_score);
        commands.insert_resource(NextState(Some(AppState::GameOver)));
        commands.insert_resource(NextState(Some(SimulationState::Paused)));
        break;
    }
}
