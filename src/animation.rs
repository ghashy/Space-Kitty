use bevy::prelude::*;
use bevy_tweening::{lens::*, *};

// ───── Body ─────────────────────────────────────────────────────────────── //

pub fn animate_star(commands: &mut Commands, entity: Entity, index: u16) {
    let scale_tween = Tween::new(
        EaseFunction::CubicIn,
        std::time::Duration::from_millis(500),
        TransformScaleLens {
            start: Vec3::splat(1.),
            end: Vec3::splat(3.),
        },
    )
    .then(
        Tween::new(
            EaseFunction::QuadraticIn,
            std::time::Duration::from_millis(300),
            TransformScaleLens {
                start: Vec3::splat(3.),
                end: Vec3::splat(1.),
            },
        )
        .with_completed_event(index as u64),
    );

    let color_tween = Tween::new(
        EaseFunction::CubicIn,
        std::time::Duration::from_millis(500),
        SpriteColorLens {
            start: Color::hsl(0., 0., 1.0),
            end: Color::hsl(0., 0., 3.9),
        },
    )
    .then(Tween::new(
        EaseFunction::QuadraticIn,
        std::time::Duration::from_millis(300),
        SpriteColorLens {
            start: Color::hsl(0., 0., 3.9),
            end: Color::hsl(0., 0., 1.0),
        },
    ));

    commands
        .entity(entity)
        .insert((Animator::new(scale_tween), Animator::new(color_tween)));
}
