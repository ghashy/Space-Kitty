use bevy::prelude::*;

// ───── Body ─────────────────────────────────────────────────────────────── //

#[derive(Component)]
pub struct RestartButton {
    pub default_handle: Handle<Image>,
    pub hover_handle: Handle<Image>,
    pub click_handle: Handle<Image>,
}

#[derive(Component)]
pub struct ExitButton {
    pub default_handle: Handle<Image>,
    pub hover_handle: Handle<Image>,
    pub click_handle: Handle<Image>,
}

#[derive(Component)]
pub struct ScrollView {
    pub position: f32,
}
