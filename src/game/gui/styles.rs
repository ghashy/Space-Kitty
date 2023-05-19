use bevy::prelude::*;

// ----- Constants ---------------------------------------------------------- //

pub const HUD_CONTAINER: Style = Style {
    display: Display::Flex,
    size: Size::new(Val::Percent(100.), Val::Percent(100.)),
    flex_direction: FlexDirection::Column,
    align_items: AlignItems::FlexEnd,
    ..Style::DEFAULT
};

pub const HEARS_ROW: Style = Style {
    display: Display::Flex,
    margin: UiRect::all(Val::Px(20.)),
    ..Style::DEFAULT
};

pub const HEART_BACKGROUND: Style = Style {
    size: Size::new(Val::Px(80.), Val::Px(70.)),
    margin: UiRect::all(Val::Px(10.)),
    justify_content: JustifyContent::Center,
    align_items: AlignItems::Center,
    ..Style::DEFAULT
};

pub const HEART_CONTENT: Style = Style {
    size: Size::new(Val::Px(80.), Val::Px(70.)),
    margin: UiRect::all(Val::Px(30.)),
    ..Style::DEFAULT
};

// ----- Body --------------------------------------------------------------- //
