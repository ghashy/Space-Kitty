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

use bevy::{
    prelude::*,
    render::{
        settings::{WgpuFeatures, WgpuSettings},
        RenderPlugin,
    },
    window::{WindowMode, WindowResolution},
    winit::WinitSettings,
};
use bevy_hanabi::HanabiPlugin;
use bevy_rapier2d::prelude::*;
use bevy_tweening::TweeningPlugin;

// ───── Current Crate Imports ────────────────────────────────────────────── //

use components::*;
use debug::DebugPlugin;
use game::GamePlugin;
use main_menu::MainMenuPlugin;
use resources::CometTimer;
use systems::*;
use transition::TransitionPlugin;

// ───── Submodules ───────────────────────────────────────────────────────── //

// Modules in folders
pub mod audio_system;
pub mod game;
pub mod main_menu;

// Top-level modules
mod animation;
mod components;
mod debug;
pub mod events;
pub mod helper_functions;
mod resources;
mod systems;
mod transition;

// ───── Constants ────────────────────────────────────────────────────────── //

const RAND_STAR_ANIMATION_TIME_RANGE: std::ops::Range<f32> = 5_f32..100_f32;
const COMET_SPEED: f32 = 500.;

// ───── Body ─────────────────────────────────────────────────────────────── //

fn main() {
    // Settings for bevy_hanabi
    let mut wgpu_settings = WgpuSettings::default();
    wgpu_settings
        .features
        .set(WgpuFeatures::VERTEX_WRITABLE_STORAGE, true);

    App::new()
        // Resources
        .init_resource::<CometTimer>()
        // Startup Systems
        .add_startup_system(setup)
        .add_startup_system(spawn_camera)
        .add_startup_system(spawn_background_stars)
        .add_startup_system(spawn_background_texture)
        .add_startup_system(spawn_dust)
        // States
        .add_state::<AppState>()
        // Events
        .add_event::<DarkenScreenEvent>()
        // Plugins
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        // IMPORTANT: Options for release build
                        // resolution: WindowResolution::new(1280., 720.)
                        //     .with_scale_factor_override(1.5),
                        // mode: WindowMode::SizedFullscreen,
                        title: String::from("Space Kitty"),
                        resizable: true,
                        ..default()
                    }),
                    ..default()
                })
                .set(RenderPlugin { wgpu_settings }),
        )
        // + 2 percents on cpu
        .add_plugin(HanabiPlugin)
        // +1.1 percent on cpu
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.))
        .add_plugin(GamePlugin)
        .add_plugin(TweeningPlugin)
        .add_plugin(MainMenuPlugin)
        .add_plugin(DebugPlugin)
        .add_plugin(TransitionPlugin)
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
        .add_system(handle_game_over)
        .add_system(exit_game)
        .run();
}

#[derive(States, Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum AppState {
    #[default]
    MainMenu,
    Game,
    GameOver,
}

// TweenEvent Codes:
// 0..250 - background stars events.
// 300 - MainMenu animation phase1 is finished.
// 301 - MainMenu animation phase2 is finished, moving to next state: Game.
// 400..450 - gui lives id animation.
