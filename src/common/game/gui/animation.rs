use bevy::prelude::*;
use bevy_tweening::{lens::TransformScaleLens, *};
use std::time::Duration;

// ───── Body ─────────────────────────────────────────────────────────────── //

pub fn animate_heart_out(commands: &mut Commands, entity: Entity, id: u64) {
    // Old image scale down
    let tween = Tween::new(
        EaseFunction::CubicIn,
        Duration::from_millis(500),
        TransformScaleLens {
            start: Vec3::splat(1.),
            end: Vec3::splat(1.3),
        },
    )
    .with_repeat_count(RepeatCount::Finite(1))
    .then(
        Tween::new(
            EaseFunction::QuarticIn,
            Duration::from_millis(300),
            TransformScaleLens {
                start: Vec3::splat(1.3),
                end: Vec3::ZERO,
            },
        )
        .with_completed_event(id)
        // New image scale up
        .then(
            Tween::new(
                EaseFunction::CubicIn,
                Duration::from_millis(300),
                TransformScaleLens {
                    start: Vec3::ZERO,
                    end: Vec3::splat(1.),
                },
            )
            .with_repeat_count(RepeatCount::Finite(1)),
        ),
    );

    commands.entity(entity).insert(Animator::new(tween));
}
