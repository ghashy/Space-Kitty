use bevy::{prelude::*, utils::HashMap};

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
pub struct ChartBlock {
    pub entities: HashMap<Entity, Entity>,
}

#[derive(Component)]
pub struct ChartRow {
    pub idx: usize,
    pub entity: Option<Entity>,
}

#[derive(Component)]
pub struct Avatar(pub Handle<Image>);
