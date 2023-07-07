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
use bevy_tweening::TweeningPlugin;

#[cfg(not(target_arch = "wasm32"))]
use bevy_hanabi::HanabiPlugin;

// ───── Current Crate Imports ────────────────────────────────────────────── //

use audio::AudioPlugin;
use components::*;
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

fn main() {
    // Settings for bevy_hanabi
    let mut wgpu_settings = WgpuSettings::default();

    #[cfg(not(target_arch = "wasm32"))]
    {
        wgpu_settings
            .features
            .set(WgpuFeatures::VERTEX_WRITABLE_STORAGE, true);
    }

    let mut app = App::new();
    // DefaultPlugins
    if !cfg!(target_arch = "wasm32") {
        app.add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        resolution: WindowResolution::new(
                            1280. / 1.1,
                            720. / 1.1,
                        )
                        .with_scale_factor_override(2.),
                        mode: WindowMode::BorderlessFullscreen,
                        title: String::from("Space Kitty"),
                        ..default()
                    }),
                    ..default()
                })
                .set(RenderPlugin { wgpu_settings }),
        );
    } else {
        app.add_plugins(DefaultPlugins);
    }

    app
        // Asset loaders
        .init_asset_loader::<JsonAssetLoader>()
        // Resources
        .init_resource::<CometTimer>()
        .init_resource::<TextureStorage>()
        // Startup Systems
        .add_startup_system(setup)
        .add_startup_system(spawn_camera)
        .add_startup_system(spawn_background_stars)
        .add_startup_system(spawn_background_texture)
        .add_startup_system(setup_audio_assets)
        // States
        .add_state::<AppState>()
        // Events
        .add_event::<DarkenScreenEvent>()
        // Plugins
        // + 2 percents on cpu
        .add_plugin(AudioPlugin)
        // +1.1 percent on cpu
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.))
        .add_plugin(GamePlugin)
        .add_plugin(GameoverPlugin)
        .add_plugin(TweeningPlugin)
        .add_plugin(MainMenuPlugin)
        .add_plugin(TransitionPlugin)
        // Audio loading system
        .add_system(
            update_app_state_after_audio_loaded
                .in_set(OnUpdate(AppState::AudioLoading)),
        )
        // Gui Update Systems
        .add_systems(
            (
                update_background_stars,
                animate_background_stars,
                spawn_periodical_comet,
                move_comets,
                despawn_outer_comets,
            )
                .in_base_set(CoreSet::Update),
        )
        // Systems
        .add_system(
            finalize_transition_to_game.in_set(OnUpdate(AppState::MainMenu)),
        )
        .add_system(handle_pressing_g_key.in_set(OnUpdate(AppState::MainMenu)))
        .add_system(handle_pressing_m_key.in_set(OnUpdate(AppState::Game)))
        // Debug ScrollView
        .add_system(debug_pressing_o_key.in_set(OnUpdate(AppState::Game)))
        .add_system(handle_pressing_m_key.in_set(OnUpdate(AppState::GameOver)))
        .add_system(handle_game_over.in_set(OnUpdate(AppState::Game)))
        .add_system(
            finalize_transition_to_gameover.in_set(OnUpdate(AppState::Game)),
        )
        .add_system(exit_game);

    #[cfg(debug_assertions)]
    app.add_plugin(DebugPlugin);

    #[cfg(not(target_arch = "wasm32"))]
    app.add_plugin(HanabiPlugin).add_startup_system(spawn_dust);

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
// 310 - Dark transition phase2: screen is transparent.
// 400..450 - gui lives id animation, hit events.
// 450..500 - gui lives id animation, regeneration events.
