use bevy::prelude::*;

// ----- Module ------------------------------------------------------------- //

pub mod components;
pub mod resources;
pub mod systems;

use crate::AppState;

use self::{resources::*, systems::*};

use super::SimulationState;

// ----- Constants ---------------------------------------------------------- //

const NUMBER_OF_STARS: usize = 10;
pub const STAR_SIZE: f32 = 30.;

// ----- Body --------------------------------------------------------------- //

pub struct StarsPlugin;

impl Plugin for StarsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<StarSpawnTimer>()
            // Enter State Systems
            .add_system(spawn_stars.in_schedule(OnEnter(AppState::Game)))
            // Systems
            .add_systems(
                (tick_star_spawn_timer, spawn_stars_over_time)
                    .in_set(OnUpdate(AppState::Game))
                    .in_set(OnUpdate(SimulationState::Running)),
            )
            // Exit State Systems
            .add_system(despawn_stars.in_schedule(OnExit(AppState::Game)));
    }
}
