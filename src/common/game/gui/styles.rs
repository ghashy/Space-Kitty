use bevy::prelude::*;

// ───── Constants ────────────────────────────────────────────────────────── //

pub fn hud_container() -> Style {
    Style {
        display: Display::Flex,
        width: Val::Percent(100.),
        height: Val::Percent(100.),
        align_content: AlignContent::SpaceAround,
        ..Style::DEFAULT
    }
}

pub fn left_side_hud_container() -> Style {
    Style {
        display: Display::Flex,
        width: Val::Percent(50.),
        height: Val::Percent(100.),
        flex_direction: FlexDirection::Column,
        align_items: AlignItems::FlexStart,
        ..Style::DEFAULT
    }
}

pub fn chart() -> Style {
    Style {
        display: Display::Flex,
        flex_direction: FlexDirection::Column,
        margin: UiRect {
            left: Val::Px(10.),
            top: Val::Px(10.),
            ..UiRect::DEFAULT
        },
        ..Style::DEFAULT
    }
}

pub fn avatar() -> Style {
    Style {
        width: Val::Px(240. / 3.7),
        height: Val::Px(241. / 3.7),
        ..Style::DEFAULT
    }
}

pub fn item_text() -> Style {
    Style {
        margin: UiRect {
            left: Val::Px(10.),
            right: Val::Px(10.),
            ..UiRect::DEFAULT
        },
        ..Style::DEFAULT
    }
}

pub fn right_side_hud_container() -> Style {
    Style {
        display: Display::Flex,
        width: Val::Percent(50.),
        height: Val::Percent(100.),
        flex_direction: FlexDirection::Column,
        align_items: AlignItems::FlexEnd,
        ..Style::DEFAULT
    }
}

pub fn hearts_row() -> Style {
    Style {
        display: Display::Flex,
        ..Style::DEFAULT
    }
}

pub fn starship_life() -> Style {
    Style {
        width: Val::Px(294. / 5.),
        height: Val::Px(297. / 5.),
        margin: UiRect::all(Val::Px(10.)),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        ..Style::DEFAULT
    }
}

pub fn messages_bar() -> Style {
    Style {
        width: Val::Percent(100.),
        height: Val::Px(300.),
        flex_direction: FlexDirection::Column,
        justify_content: JustifyContent::Start,
        align_items: AlignItems::End,
        margin: UiRect::all(Val::Percent(1.)),
        ..Style::DEFAULT
    }
}
