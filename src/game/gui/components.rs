use bevy::prelude::*;

// ───── Body ─────────────────────────────────────────────────────────────── //

#[derive(Component)]
pub struct Hud;

#[derive(Component)]
pub struct MessagesList;

#[derive(Component)]
pub struct Message(pub Timer);

#[derive(Component, Debug)]
pub struct HeartImage(pub u64, pub Handle<Image>, pub Handle<Image>);

#[derive(Component)]
pub struct ChartBlock;

#[derive(Component)]
pub struct TopImageMarker;

#[derive(Component)]
pub struct TopTextMarker;
