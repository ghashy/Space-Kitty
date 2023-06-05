use bevy::prelude::*;

// ───── Current Crate Imports ────────────────────────────────────────────── //

use crate::game::player::BALL_SIZE;

// ───── Body ─────────────────────────────────────────────────────────────── //

pub trait Vec2Util {
    fn reflect(&mut self, _rhs: Vec2) {}
}

impl Vec2Util for Vec2 {
    fn reflect(&mut self, rhs: Vec2) {
        let scalar = 2. * self.dot(rhs) / rhs.dot(rhs);

        self.x = self.x - (rhs.x * scalar);
        self.y = self.y - (rhs.y * scalar);
    }
}

pub fn clamp_translation(
    mut translation: Vec3,
    x_min: f32,
    x_max: f32,
    y_min: f32,
    y_max: f32,
) -> Vec3 {
    // Bound the player x position
    if translation.x < x_min {
        translation.x = x_min;
    } else if translation.x > x_max {
        translation.x = x_max;
    }

    // Bound the players y position
    if translation.y < y_min {
        translation.y = y_min;
    } else if translation.y > y_max {
        translation.y = y_max;
    }

    translation
}

pub fn get_window_borders(window: &Window) -> (f32, f32, f32, f32) {
    let half_enemy_size = BALL_SIZE / 2.;
    let x_min = 0. + half_enemy_size;
    let x_max = window.width() - half_enemy_size;
    let y_min = 0. + half_enemy_size;
    let y_max = window.height() - half_enemy_size;
    (x_min, x_max, y_min, y_max)
}
