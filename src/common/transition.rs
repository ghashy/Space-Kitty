use bevy::prelude::*;
use bevy_tweening::*;

// ───── Current Crate Imports ────────────────────────────────────────────── //

use crate::common::components::{DarkScreenOverlap, DarkenScreenEvent};

// ───── Body ─────────────────────────────────────────────────────────────── //

pub struct TransitionPlugin;

impl Plugin for TransitionPlugin {
    fn build(&self, app: &mut App) {
        app
            // States
            .add_state::<TransitionState>()
            .add_systems(Update, component_animator_system::<BackgroundColor>)
            .add_systems(Update, spawn_overlap_on_transition)
            .add_systems(Update, despawn_overlap_after_transition);
    }
}

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum TransitionState {
    #[default]
    NoTransition,
    Transition,
}

#[derive(Clone, Copy)]
pub enum TransitionRoute {
    MenuToGame = 300,
    GameToGameover = 301,

    #[cfg(target_arch = "wasm32")]
    SplashToMenu = 302,
}

pub struct UiColorLens {
    start: Color,
    end: Color,
}

impl Lens<BackgroundColor> for UiColorLens {
    fn lerp(&mut self, target: &mut BackgroundColor, ratio: f32) {
        let start = Vec4::from(self.start);
        let end = Vec4::from(self.end);
        target.0 = (start + (end - start) * ratio).into();
    }
}

pub fn spawn_overlap_on_transition(
    mut commands: Commands,
    mut event_reader: EventReader<DarkenScreenEvent>,
    app_state: Res<State<TransitionState>>,
    mut next_state: ResMut<NextState<TransitionState>>,
) {
    if let Some(event) = event_reader.iter().next() {
        if *app_state.get() == TransitionState::NoTransition {
            let sequence = Tween::new(
                EaseFunction::CubicIn,
                std::time::Duration::from_millis(500),
                UiColorLens {
                    start: Color::rgba(0., 0., 0., 0.),
                    end: Color::rgba(0., 0., 0., 1.),
                },
            )
            .with_completed_event(event.0 as u64)
            .then(
                Tween::new(
                    EaseFunction::CubicIn,
                    std::time::Duration::from_millis(500),
                    UiColorLens {
                        start: Color::rgba(0., 0., 0., 1.),
                        end: Color::rgba(0., 0., 0., 0.),
                    },
                )
                .with_completed_event(310),
            );

            let node = NodeBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    width: Val::Percent(100.),
                    height: Val::Percent(100.),
                    ..default()
                },
                background_color: BackgroundColor(Color::rgba(0., 0., 0., 0.)),
                transform: Transform::from_xyz(0., 0., 100.),
                ..default()
            };

            commands.spawn((node, Animator::new(sequence), DarkScreenOverlap));
            next_state.set(TransitionState::Transition);
        }
    }
}

fn despawn_overlap_after_transition(
    mut commands: Commands,
    screen_overlap_query: Query<Entity, With<DarkScreenOverlap>>,
    mut tween_event: EventReader<TweenCompleted>,
    mut next_state: ResMut<NextState<TransitionState>>,
) {
    for event in tween_event.iter() {
        // Is phase2 finished? user data shoud be eq 1
        if event.user_data == 310 {
            if let Ok(overlap) = screen_overlap_query.get_single() {
                commands.entity(overlap).despawn_recursive();
                next_state.set(TransitionState::NoTransition);
            }
        }
    }
}
