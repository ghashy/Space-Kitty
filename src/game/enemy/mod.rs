use bevy::prelude::*;

// ───── Current Crate Imports ────────────────────────────────────────────── //

use self::{components::EnemyIsArrivingEvent, systems::*};
use super::SimulationState;
use crate::AppState;

// ───── Submodules ───────────────────────────────────────────────────────── //

pub mod components;
pub mod resources;
pub mod systems;

// ───── Constants ────────────────────────────────────────────────────────── //

const ENEMY_SPEED: f32 = 10000.;

// ───── Body ─────────────────────────────────────────────────────────────── //

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app
            // Events
            .add_event::<EnemyIsArrivingEvent>()
            // Systems
            .add_systems(
                (
                    update_enemy_direction,
                    enemy_movement,
                    spawn_enemy_on_game_progress,
                    rotate_patch_of_light,
                    // enemy_hit_fish,
                )
                    .in_set(OnUpdate(AppState::Game))
                    .in_set(OnUpdate(SimulationState::Running)),
            )
            // Exit State Systems
            .add_system(despawn_enemies.in_schedule(OnExit(AppState::Game)));
    }
}
