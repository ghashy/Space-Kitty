use bevy::prelude::*;

// ───── Constants ────────────────────────────────────────────────────────── //

// Colors
pub const NORMAL_BUTTON_COLOR: Color = Color::WHITE;

// Planets
pub const PLANETS_NODE: Style = Style {
    size: Size::new(Val::Percent(100.), Val::Percent(100.)),
    position_type: PositionType::Absolute,
    ..Style::DEFAULT
};
pub const PLANET_BIG: Style = Style {
    size: Size::new(Val::Px(700.), Val::Px(600.)),
    position: UiRect::new(
        Val::Px(-39.2),
        Val::Px(-302.2),
        Val::Px(-349.3),
        Val::Px(0.),
    ),
    position_type: PositionType::Absolute,
    ..Style::DEFAULT
};
pub const PLANET_MAIN: Style = Style {
    size: Size::new(Val::Px(595.6), Val::Px(376.5)),
    position: UiRect::new(
        Val::Percent(37.9),
        Val::Percent(0.),
        Val::Percent(54.1),
        Val::Percent(0.),
    ),
    position_type: PositionType::Absolute,
    ..Style::DEFAULT
};
pub const PLANET_ATMOSPHERE: Style = Style {
    size: Size::new(Val::Px(130.), Val::Px(130.)),
    justify_content: JustifyContent::Center,
    position: UiRect::new(
        Val::Percent(66.7),
        Val::Percent(0.),
        Val::Percent(27.3),
        Val::Percent(0.),
    ),
    position_type: PositionType::Absolute,
    ..Style::DEFAULT
};
pub const PLANET_SMALL: Style = Style {
    size: Size::new(Val::Px(100.), Val::Px(100.)),
    align_self: AlignSelf::Center,
    position_type: PositionType::Absolute,
    ..Style::DEFAULT
};

// Main Menu
pub const MAIN_CONTAINER: Style = Style {
    display: Display::Flex,
    flex_direction: FlexDirection::Row,
    size: Size::new(Val::Percent(100.), Val::Percent(100.)),
    gap: Size::new(Val::Px(8.), Val::Px(8.)),
    ..Style::DEFAULT
};
pub const HALF_SCREEN: Style = Style {
    flex_direction: FlexDirection::Column,
    size: Size::new(Val::Percent(50.), Val::Percent(100.)),
    flex_grow: 1.,
    ..Style::DEFAULT
};
pub const TOP_PART: Style = Style {
    flex_direction: FlexDirection::Column,
    justify_content: JustifyContent::FlexEnd,
    align_items: AlignItems::FlexEnd,
    flex_grow: 1.,
    ..Style::DEFAULT
};
pub const BOTTOM_PART: Style = Style {
    flex_direction: FlexDirection::Column,
    flex_grow: 1.,
    justify_content: JustifyContent::Center,
    align_items: AlignItems::FlexEnd,
    gap: Size::new(Val::Percent(0.), Val::Percent(16.)),
    ..Style::DEFAULT
};
pub const NORMAL_BUTTON_STYLE: Style = Style {
    size: Size::new(Val::Px(200.), Val::Px(80.)),
    ..Style::DEFAULT
};
pub const IMAGE_STYLE: Style = Style {
    max_size: Size::new(Val::Px(454.), Val::Px(124.)),
    margin: UiRect::new(Val::Px(8.), Val::Px(8.), Val::Px(8.), Val::Px(8.)),
    ..Style::DEFAULT
};
pub const TITLE_STYLE: Style = Style {
    flex_direction: FlexDirection::Row,
    justify_content: JustifyContent::Center,
    align_items: AlignItems::Center,
    size: Size::new(Val::Px(300.), Val::Px(120.)),
    ..Style::DEFAULT
};

// ───── Body ─────────────────────────────────────────────────────────────── //

#[allow(dead_code)]
pub fn get_button_text_style(asset_server: &Res<AssetServer>) -> TextStyle {
    TextStyle {
        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
        font_size: 32.,
        color: Color::WHITE,
    }
}

#[allow(dead_code)]
pub fn get_title_text_style(asset_server: &Res<AssetServer>) -> TextStyle {
    TextStyle {
        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
        font_size: 64.,
        color: Color::WHITE,
    }
}
