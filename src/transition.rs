use bevy::prelude::*;
use bevy_tweening::*;

// ───── Current Crate Imports ────────────────────────────────────────────── //

use crate::{
    components::{DarkScreenOverlap, DarkenScreenEvent},
    AppState,
};

// ───── Body ─────────────────────────────────────────────────────────────── //

pub struct TransitionPlugin;

impl Plugin for TransitionPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(component_animator_system::<BackgroundColor>)
            .add_system(
                spawn_overlap_on_transition
                    .in_set(OnUpdate(AppState::MainMenu)),
            )
            .add_system(despawn_overlap_after_transition);
    }
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
    app_state: Res<State<AppState>>,
) {
    for _ in event_reader.iter() {
        if app_state.0 == AppState::MainMenu {
            let sequence = Tween::new(
                EaseFunction::CubicIn,
                std::time::Duration::from_millis(500),
                UiColorLens {
                    start: Color::rgba(0., 0., 0., 0.),
                    end: Color::rgba(0., 0., 0., 1.),
                },
            )
            .with_completed_event(300)
            .then(
                Tween::new(
                    EaseFunction::CubicIn,
                    std::time::Duration::from_millis(500),
                    UiColorLens {
                        start: Color::rgba(0., 0., 0., 1.),
                        end: Color::rgba(0., 0., 0., 0.),
                    },
                )
                .with_completed_event(301),
            );

            let node = NodeBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    size: Size::new(Val::Percent(100.), Val::Percent(100.)),
                    ..default()
                },
                background_color: BackgroundColor(Color::rgba(0., 0., 0., 0.)),
                transform: Transform::from_xyz(0., 0., 100.),
                ..default()
            };

            commands.spawn((node, Animator::new(sequence), DarkScreenOverlap));
        }
        break;
    }
}

fn despawn_overlap_after_transition(
    mut commands: Commands,
    screen_overlap_query: Query<Entity, With<DarkScreenOverlap>>,
    mut tween_event: EventReader<TweenCompleted>,
) {
    for event in tween_event.iter() {
        // Is phase2 finished? user data shoud be eq 1
        if event.user_data == 1 {
            if let Ok(overlap) = screen_overlap_query.get_single() {
                commands.entity(overlap).despawn_recursive();
            }
        }
    }
}
