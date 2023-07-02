use bevy::app::AppExit;
use bevy::prelude::*;

// ───── Current Crate Imports ────────────────────────────────────────────── //

use crate::components::DarkenScreenEvent;
use crate::main_menu::animation::*;
use crate::main_menu::components::*;
use crate::transition::TransitionRoute;

// ───── Body ─────────────────────────────────────────────────────────────── //

pub fn interact_with_play_button(
    mut button_query: Query<
        (&Interaction, &mut UiImage, &PlayButton),
        Changed<Interaction>,
    >,
    mut event_writer: EventWriter<DarkenScreenEvent>,
) {
    if let Ok((interaction, mut image, play_button)) =
        button_query.get_single_mut()
    {
        match *interaction {
            Interaction::Clicked => {
                animate_button_click(&mut image, play_button);
                event_writer
                    .send(DarkenScreenEvent(TransitionRoute::MenuToGame));
            }
            Interaction::Hovered => {
                animate_button_hover(&mut image, play_button);
            }
            Interaction::None => {
                animate_button_none(&mut image, play_button);
            }
        }
    }
}

pub fn interact_with_quit_button(
    mut button_query: Query<
        (&Interaction, &mut UiImage, &QuitButton),
        Changed<Interaction>,
    >,
    mut app_exit_event_writer: EventWriter<AppExit>,
) {
    if let Ok((interaction, mut image, button)) = button_query.get_single_mut()
    {
        match *interaction {
            Interaction::Clicked => {
                animate_button_click(&mut image, button);
                app_exit_event_writer.send(AppExit);
            }
            Interaction::Hovered => {
                animate_button_hover(&mut image, button);
            }
            Interaction::None => {
                animate_button_none(&mut image, button);
            }
        }
    }
}
