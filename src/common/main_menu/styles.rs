use bevy::prelude::*;

// ───── Constants ────────────────────────────────────────────────────────── //

// Colors
pub const NORMAL_BUTTON_COLOR: Color = Color::WHITE;

// Planets
pub fn planets_node() -> Style {
    Style {
        width: Val::Percent(100.),
        height: Val::Percent(100.),
        position_type: PositionType::Absolute,
        ..Style::DEFAULT
    }
}
pub fn planet_big() -> Style {
    Style {
        width: Val::Px(700.),
        height: Val::Px(600.),
        left: Val::Px(-39.2),
        right: Val::Px(-302.2),
        top: Val::Px(-349.3),
        bottom: Val::Px(0.),
        position_type: PositionType::Absolute,
        ..Style::DEFAULT
    }
}
pub fn planet_main() -> Style {
    Style {
        width: Val::Px(595.6),
        height: Val::Px(376.5),
        left: Val::Percent(37.9),
        right: Val::Percent(0.),
        top: Val::Percent(54.1),
        bottom: Val::Percent(0.),
        position_type: PositionType::Absolute,
        ..Style::DEFAULT
    }
}
pub fn planet_atmosphere() -> Style {
    Style {
        width: Val::Px(130.),
        height: Val::Px(130.),
        justify_content: JustifyContent::Center,
        left: Val::Percent(66.7),
        right: Val::Percent(0.),
        top: Val::Percent(27.3),
        bottom: Val::Percent(0.),
        position_type: PositionType::Absolute,
        ..Style::DEFAULT
    }
}
pub fn planet_small() -> Style {
    Style {
        width: Val::Px(100.),
        height: Val::Px(100.),
        align_self: AlignSelf::Center,
        position_type: PositionType::Absolute,
        ..Style::DEFAULT
    }
}

// Main Menu
pub fn main_container() -> Style {
    Style {
        display: Display::Flex,
        flex_direction: FlexDirection::Row,
        width: Val::Percent(100.),
        height: Val::Percent(100.),
        row_gap: Val::Px(8.),
        column_gap: Val::Px(8.),
        ..Style::DEFAULT
    }
}
pub fn half_screen() -> Style {
    Style {
        flex_direction: FlexDirection::Column,
        width: Val::Percent(50.),
        height: Val::Percent(100.),
        flex_grow: 1.,
        ..Style::DEFAULT
    }
}
pub fn top_part() -> Style {
    Style {
        flex_direction: FlexDirection::Column,
        justify_content: JustifyContent::FlexEnd,
        align_items: AlignItems::FlexEnd,
        flex_grow: 1.,
        ..Style::DEFAULT
    }
}
pub fn bottom_part() -> Style {
    Style {
        flex_direction: FlexDirection::Column,
        flex_grow: 1.,
        justify_content: JustifyContent::Center,
        align_items: AlignItems::FlexEnd,
        row_gap: Val::Percent(0.),
        column_gap: Val::Percent(16.),
        ..Style::DEFAULT
    }
}
pub fn normal_button_style() -> Style {
    Style {
        width: Val::Px(200.),
        height: Val::Px(80.),
        ..Style::DEFAULT
    }
}
pub fn image_style() -> Style {
    Style {
        max_width: Val::Px(454.),
        max_height: Val::Px(124.),
        margin: UiRect::new(Val::Px(8.), Val::Px(8.), Val::Px(8.), Val::Px(8.)),
        ..Style::DEFAULT
    }
}
pub fn title_style() -> Style {
    Style {
        flex_direction: FlexDirection::Row,
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        width: Val::Px(300.),
        height: Val::Px(120.),
        ..Style::DEFAULT
    }
}

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
