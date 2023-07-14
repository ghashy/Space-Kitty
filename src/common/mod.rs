// #![deny(
//     warnings,
//     missing_copy_implementations,
//     trivial_casts,
//     trivial_numeric_casts,
//     unsafe_code,
//     unstable_features,
//     unused_import_braces,
//     unused_qualifications,
//     missing_docs
// )]

use asset_loader::JsonAssetLoader;
use bevy::{
    prelude::*,
    render::{
        settings::{WgpuFeatures, WgpuSettings},
        RenderPlugin,
    },
    window::{WindowMode, WindowResolution},
};
use bevy_rapier2d::prelude::*;
use bevy_tweening::TweenCompleted;
use bevy_tweening::TweeningPlugin;

// ───── Current Crate Imports ────────────────────────────────────────────── //

use audio::AudioPlugin;
use components::*;
use file_logger_plugin::FileLoggerPlugin;
use game::GamePlugin;
use gameover::GameoverPlugin;
use main_menu::MainMenuPlugin;
use resources::{CometTimer, TextureStorage};
use systems::*;
use transition::TransitionPlugin;

#[cfg(debug_assertions)]
use debug::DebugPlugin;

// ───── Submodules ───────────────────────────────────────────────────────── //

// Modules in folders
pub mod asset_loader;
pub mod audio;
pub mod game;
pub mod gameover;
pub mod main_menu;

// Top-level modules
mod animation;
mod components;
pub mod events;
mod file_logger_plugin;
pub mod helper_functions;
mod resources;
pub mod systems;
mod transition;

#[cfg(debug_assertions)]
mod debug;

// ───── Constants ────────────────────────────────────────────────────────── //

const RAND_STAR_ANIMATION_TIME_RANGE: std::ops::Range<f32> = 5_f32..100_f32;
const COMET_SPEED: f32 = 500.;

// ───── Body ─────────────────────────────────────────────────────────────── //

pub fn start() {
    let mut app = App::new();
    // DefaultPlugins
    let group = DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            resizable: false,
            mode: WindowMode::BorderlessFullscreen,
            title: String::from("Space Kitty"),
            ..default()
        }),
        ..default()
    });

    // Logger
    {
        #[cfg(feature = "file_logger")]
        let group = group.disable::<bevy::log::LogPlugin>();

        app.add_plugins(group);

        #[cfg(feature = "file_logger")]
        app.add_plugins(FileLoggerPlugin);
    }

    app
        // Asset loaders
        .init_asset_loader::<JsonAssetLoader>()
        // Resources
        .init_resource::<CometTimer>()
        .init_resource::<TextureStorage>()
        // Startup Systems
        .add_systems(Startup, setup)
        .add_systems(Startup, spawn_camera)
        .add_systems(Startup, spawn_background_stars)
        .add_systems(Startup, spawn_background_texture)
        .add_systems(Startup, setup_audio_assets)
        // States
        .add_state::<AppState>()
        // Events
        .add_event::<DarkenScreenEvent>()
        // Plugins
        // + 2 percents on cpu
        .add_plugins(AudioPlugin)
        // +1.1 percent on cpu
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.))
        .add_plugins(GamePlugin)
        .add_plugins(GameoverPlugin)
        .add_plugins(TweeningPlugin)
        .add_plugins(MainMenuPlugin)
        .add_plugins(TransitionPlugin)
        // Audio loading system
        .add_systems(
            Update,
            update_app_state_after_audio_loaded
                .run_if(in_state(AppState::AudioLoading)),
        )
        // Gui Update Systems
        .add_systems(
            Update,
            (
                update_background_stars,
                animate_background_stars,
                spawn_periodical_comet,
                move_comets,
                despawn_outer_comets,
            ),
        )
        // Systems
        .add_systems(
            Update,
            finalize_transition_to_game.run_if(in_state(AppState::MainMenu)),
        )
        .add_systems(Update, handle_game_over.run_if(in_state(AppState::Game)))
        .add_systems(
            Update,
            finalize_transition_to_gameover.run_if(in_state(AppState::Game)),
        );

    #[cfg(debug_assertions)]
    app.add_plugins(DebugPlugin);

    app.init_resource::<self::resources::DustTimer>()
        .add_systems(Update, spawn_dust_wasm)
        .add_systems(Update, poll_and_despawn_dust_particles);

    #[cfg(target_os = "android")]
    app.insert_resource(Msaa::Off);

    app.run();
}

#[derive(States, Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum AppState {
    #[default]
    AudioLoading,
    MainMenu,
    Game,
    GameOver,
}

// TweenEvent Codes:
// 0..250 - background stars events.
// 300 - Dark transition phase1: screen is black, transition from menu to game.
// 301 - Dark transition phase1: screen is black, transition from game go
// gameover.
// 302 - Dark transition phase1: screen is black, transition from splash go
// game.
// 310 - Dark transition phase2: screen is transparent.
// 400..450 - gui lives id animation, hit events.
// 450..500 - gui lives id animation, regeneration events.
