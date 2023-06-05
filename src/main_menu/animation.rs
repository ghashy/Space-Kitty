use bevy::prelude::*;

// ───── Current Crate Imports ────────────────────────────────────────────── //

use super::components::ButtonType;

// ───── Body ─────────────────────────────────────────────────────────────── //

pub fn animate_button_hover(image: &mut UiImage, button: impl ButtonType) {
    *image = button.get_hover_handle().into();
}

pub fn animate_button_click(image: &mut UiImage, button: impl ButtonType) {
    *image = button.get_click_handle().into();
}
pub fn animate_button_none(image: &mut UiImage, button: impl ButtonType) {
    *image = button.get_default_handle().into();
}
