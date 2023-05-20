use bevy::prelude::*;
use bevy_tweening::{lens::TransformScaleLens, *};
use std::time::Duration;

// ───── Body ─────────────────────────────────────────────────────────────── //

pub fn animate_heart_down(commands: &mut Commands, entity: Entity, id: u8) {
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
        .with_completed_event(id as u64),
    );

    commands.entity(entity).insert(Animator::new(tween));
}

pub fn animate_heart_up(commands: &mut Commands, entity: Entity, id: u8) {
    let tween = Tween::new(
        EaseFunction::CubicIn,
        Duration::from_millis(300),
        TransformScaleLens {
            start: Vec3::ZERO,
            end: Vec3::splat(1.),
        },
    )
    .with_repeat_count(RepeatCount::Finite(1))
    .with_completed_event(id as u64);

    commands.entity(entity).insert(Animator::new(tween));
}
