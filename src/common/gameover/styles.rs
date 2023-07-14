use bevy::prelude::*;

// ───── Constants ────────────────────────────────────────────────────────── //

pub fn main_container() -> Style {
    Style {
        width: Val::Percent(100.),
        height: Val::Percent(100.),
        justify_content: JustifyContent::SpaceBetween,
        flex_direction: FlexDirection::Row,
        align_items: AlignItems::End,
        ..Style::DEFAULT
    }
}

pub fn left_container() -> Style {
    Style {
        //
        ..Style::DEFAULT
    }
}

pub fn right_container() -> Style {
    Style {
        bottom: Val::Percent(4.1),
        ..Style::DEFAULT
    }
}

pub fn left_button() -> Style {
    Style {
        width: Val::Px(381. / 2.5),
        height: Val::Px(771. / 2.5),
        ..Style::DEFAULT
    }
}

pub fn right_button() -> Style {
    Style {
        width: Val::Px(363. / 2.5),
        height: Val::Px(624. / 2.5),
        ..Style::DEFAULT
    }
}

pub fn spaceship() -> Style {
    Style {
        width: Val::Px(1764. / 2.5),
        height: Val::Px(311. / 2.5),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Start,
        ..Style::DEFAULT
    }
}

pub fn emitting_frame() -> Style {
    Style {
        width: Val::Px(1116. / 2.5),
        height: Val::Px(254. / 2.5),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::End,
        left: Val::Px(-4.4),
        bottom: Val::Px(47.5),
        ..Style::DEFAULT
    }
}

pub fn emitting_fill() -> Style {
    Style {
        width: Val::Px(1939. / 2.5),
        height: Val::Px(963. / 2.5),
        left: Val::Px(-7.1),
        bottom: Val::Px(24.9),
        ..Style::DEFAULT
    }
}

pub fn board_frame() -> Style {
    Style {
        width: Val::Px(1142. / 2.5),
        height: Val::Px(1030. / 2.5),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::End,
        left: Val::Px(167.),
        bottom: Val::Px(89.8),
        ..Style::DEFAULT
    }
}

pub fn board_fill() -> Style {
    Style {
        width: Val::Px(1102. / 2.5),
        height: Val::Px(990. / 2.5),
        flex_direction: FlexDirection::Column,
        align_self: AlignSelf::Center,
        align_items: AlignItems::Center,
        padding: UiRect::top(Val::Px(12.5)),
        ..Style::DEFAULT
    }
}

pub fn scroll_parent() -> Style {
    Style {
        flex_direction: FlexDirection::Column,
        margin: UiRect {
            top: Val::Px(10.),
            ..UiRect::DEFAULT
        },
        max_height: Val::Px(306.),
        overflow: Overflow::clip(),
        ..Style::DEFAULT
    }
}

pub fn scroll_view() -> Style {
    Style {
        flex_direction: FlexDirection::Column,
        width: Val::Px(439.9),
        ..Style::DEFAULT
    }
}

pub fn row() -> Style {
    Style {
        flex_direction: FlexDirection::Row,
        align_items: AlignItems::Center,
        ..Style::DEFAULT
    }
}

pub fn dog_face() -> Style {
    Style {
        width: Val::Px(312. / 2.7),
        height: Val::Px(308. / 2.7),
        ..Style::DEFAULT
    }
}

pub fn cat_face() -> Style {
    Style {
        width: Val::Px(114.1 / 2.7),
        height: Val::Px(107.6 / 2.7),
        ..Style::DEFAULT
    }
}

pub fn bag() -> Style {
    Style {
        width: Val::Px(251. / 2.7),
        height: Val::Px(240. / 2.7),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        padding: UiRect {
            top: Val::Px(28.4),
            right: Val::Px(31.4),
            ..UiRect::DEFAULT
        },
        ..Style::DEFAULT
    }
}
