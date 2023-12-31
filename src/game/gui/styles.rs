use bevy::prelude::*;

// ───── Constants ────────────────────────────────────────────────────────── //

pub const HUD_CONTAINER: Style = Style {
    display: Display::Flex,
    size: Size::new(Val::Percent(100.), Val::Percent(100.)),
    align_content: AlignContent::SpaceAround,
    ..Style::DEFAULT
};

pub const LEFT_SIDE_HUD_CONTAINER: Style = Style {
    display: Display::Flex,
    size: Size::new(Val::Percent(50.), Val::Percent(100.)),
    flex_direction: FlexDirection::Column,
    align_items: AlignItems::FlexStart,
    ..Style::DEFAULT
};

pub const CHART: Style = Style {
    display: Display::Flex,
    flex_direction: FlexDirection::Column,
    margin: UiRect {
        left: Val::Px(10.),
        top: Val::Px(10.),
        ..UiRect::DEFAULT
    },
    ..Style::DEFAULT
};

pub const AVATAR: Style = Style {
    size: Size::new(Val::Px(240. / 3.7), Val::Px(241. / 3.7)),
    ..Style::DEFAULT
};

pub const ITEM_TEXT: Style = Style {
    margin: UiRect {
        left: Val::Px(10.),
        right: Val::Px(10.),
        ..UiRect::DEFAULT
    },
    ..Style::DEFAULT
};

pub const RIGHT_SIDE_HUD_CONTAINER: Style = Style {
    display: Display::Flex,
    size: Size::new(Val::Percent(50.), Val::Percent(100.)),
    flex_direction: FlexDirection::Column,
    align_items: AlignItems::FlexEnd,
    ..Style::DEFAULT
};

pub const HEARTS_ROW: Style = Style {
    display: Display::Flex,
    ..Style::DEFAULT
};

pub const STARSHIP_LIFE: Style = Style {
    size: Size::new(Val::Px(294. / 5.), Val::Px(297. / 5.)),
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
    margin: UiRect::all(Val::Percent(1.)),
    ..Style::DEFAULT
};
