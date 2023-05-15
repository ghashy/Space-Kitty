use bevy::{app::AppExit, prelude::*, window::PrimaryWindow};

// ----- Crate -------------------------------------------------------------- //

use crate::{
    audio_system::resources::SamplePack, events::*, game::SimulationState,
    AppState,
};

// ----- Body --------------------------------------------------------------- //

pub fn setup(mut commands: Commands, asset_server: ResMut<AssetServer>) {
    let sound_effect_1: Handle<AudioSource> =
        asset_server.load("audio/pluck_001.ogg");
    let sound_effect_2: Handle<AudioSource> =
        asset_server.load("audio/pluck_002.ogg");
    let exp = asset_server.load("audio/explosionCrunch_000.ogg");
    let pick_star = asset_server.load("audio/laserLarge_000.ogg");

    commands.insert_resource(SamplePack {
        pluck1: sound_effect_1,
        pluck2: sound_effect_2,
        exp,
        pick_star,
    });
}

pub fn spawn_camera(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window_query.get_single().unwrap();

    commands.spawn(Camera2dBundle {
        transform: Transform::from_xyz(
            window.width() / 2.,
            window.height() / 2.,
            0.,
        ),
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
) {
    for event in game_over_event_reader.iter() {
        println!("FinalScore: {}", event.final_score);
        commands.insert_resource(NextState(Some(AppState::GameOver)));
        commands.insert_resource(NextState(Some(SimulationState::Paused)));
    }
}
