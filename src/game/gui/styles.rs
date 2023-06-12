use bevy::prelude::*;

// ───── Constants ────────────────────────────────────────────────────────── //

pub const HUD_CONTAINER: Style = Style {
    display: Display::Flex,
    size: Size::new(Val::Percent(100.), Val::Percent(100.)),
    flex_direction: FlexDirection::Column,
    align_items: AlignItems::FlexEnd,
    ..Style::DEFAULT
};

pub const RIGHT_SIDE_BLOCK: Style = Style {
    display: Display::Flex,
    flex_direction: FlexDirection::Column,
    margin: UiRect::all(Val::Px(20.)),
    ..Style::DEFAULT
};

pub const HEARTS_ROW: Style = Style {
    display: Display::Flex,
    ..Style::DEFAULT
};

pub const CAT_FACE: Style = Style {
    size: Size::new(Val::Px(80.), Val::Px(70.)),
    margin: UiRect::all(Val::Px(10.)),
    justify_content: JustifyContent::Center,
    align_items: AlignItems::Center,
    ..Style::DEFAULT
};

pub const MESSAGES_BAR: Style = Style {
    size: Size::new(Val::Percent(100.), Val::Px(300.)),
    flex_direction: FlexDirection::Column,
    justify_content: JustifyContent::Start,
    align_items: AlignItems::End,
    ..Style::DEFAULT
};

// ───── Body ─────────────────────────────────────────────────────────────── //
