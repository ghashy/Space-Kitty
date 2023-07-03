use bevy::prelude::*;

// ───── Constants ────────────────────────────────────────────────────────── //

pub const MAIN_CONTAINER: Style = Style {
    size: Size::new(Val::Percent(100.), Val::Percent(100.)),
    justify_content: JustifyContent::Center,
    flex_direction: FlexDirection::Row,
    align_items: AlignItems::End,
    ..Style::DEFAULT
};

pub const SPACESHIP: Style = Style {
    size: Size::new(Val::Px(1764. / 2.5), Val::Px(311. / 2.5)),
    justify_content: JustifyContent::Center,
    align_items: AlignItems::Start,
    ..Style::DEFAULT
};

pub const EMITTING_FRAME: Style = Style {
    size: Size::new(Val::Px(1116. / 2.5), Val::Px(254. / 2.5)),
    justify_content: JustifyContent::Center,
    align_items: AlignItems::End,
    position: UiRect::new(
        Val::Px(-4.4),
        Val::Undefined,
        Val::Undefined,
        Val::Px(47.5),
    ),
    ..Style::DEFAULT
};

pub const EMITTING_FILL: Style = Style {
    size: Size::new(Val::Px(1939. / 2.5), Val::Px(963. / 2.5)),
    position: UiRect::new(
        Val::Px(-7.1),
        Val::Undefined,
        Val::Undefined,
        Val::Px(24.9),
    ),
    ..Style::DEFAULT
};

pub const BOARD_FRAME: Style = Style {
    size: Size::new(Val::Px(1142. / 2.5), Val::Px(1030. / 2.5)),
    justify_content: JustifyContent::Center,
    align_items: AlignItems::End,
    position: UiRect::new(
        Val::Px(167.),
        Val::Undefined,
        Val::Undefined,
        Val::Px(89.8),
    ),
    ..Style::DEFAULT
};

pub const BOARD_FILL: Style = Style {
    size: Size::new(Val::Px(1102. / 2.5), Val::Px(990. / 2.5)),
    flex_direction: FlexDirection::Column,
    align_self: AlignSelf::Center,
    align_items: AlignItems::Center,
    padding: UiRect::new(
        Val::Undefined,
        Val::Undefined,
        Val::Px(12.5),
        Val::Undefined,
    ),
    // position: UiRect::new(
    //     Val::Undefined,
    //     Val::Undefined,
    //     Val::Undefined,
    //     Val::Px(8.2),
    // ),
    ..Style::DEFAULT
};

pub const SCROLL_PARENT: Style = Style {
    flex_direction: FlexDirection::Column,
    margin: UiRect {
        top: Val::Px(10.),
        ..UiRect::DEFAULT
    },
    max_size: Size::height(Val::Px(312.6)),
    overflow: Overflow::Hidden,
    ..Style::DEFAULT
};

pub const SCROLL_VIEW: Style = Style {
    flex_direction: FlexDirection::Column,
    size: Size::width(Val::Px(439.9)),
    max_size: Size::UNDEFINED,
    ..Style::DEFAULT
};

pub const ROW: Style = Style {
    flex_direction: FlexDirection::Row,
    align_items: AlignItems::Center,
    ..Style::DEFAULT
};

pub const DOG_FACE: Style = Style {
    size: Size::new(Val::Px(312. / 2.7), Val::Px(308. / 2.7)),
    ..Style::DEFAULT
};

pub const CAT_FACE: Style = Style {
    size: Size::new(Val::Px(114.1), Val::Px(107.6)),
    ..Style::DEFAULT
};

pub const BAG: Style = Style {
    size: Size::new(Val::Px(251. / 2.7), Val::Px(240. / 2.7)),
    ..Style::DEFAULT
};
