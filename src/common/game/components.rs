use bevy::prelude::*;

// #[derive(Component)] // pub struct Wall;

#[derive(Component)]
pub enum Wall {
    Top,
    Bottom,
    Left,
    Right,
}

#[derive(Component)]
pub struct BackgroundTexture;

#[derive(Component)]
pub struct ControlsSheet;
