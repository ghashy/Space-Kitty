use bevy::app::AppExit;
use bevy::prelude::*;
use kira::sound::static_sound::StaticSoundSettings;

// ───── Current Crate Imports ────────────────────────────────────────────── //

use crate::common::audio::assets::AudioSource;
use crate::common::audio::resources::KiraManager;
use crate::common::audio::resources::SamplePack;
use crate::common::components::DarkenScreenEvent;
use crate::common::main_menu::animation::*;
use crate::common::main_menu::components::*;
use crate::common::transition::TransitionRoute;

// ───── Body ─────────────────────────────────────────────────────────────── //

pub fn interact_with_play_button(
    mut button_query: Query<
        (&Interaction, &mut UiImage, &PlayButton),
        Changed<Interaction>,
    >,
    mut event_writer: EventWriter<DarkenScreenEvent>,
    mut kira_manager: NonSendMut<KiraManager>,
    audio_assets: Res<Assets<AudioSource>>,
    sample_pack: Res<SamplePack>,
) {
    if let Ok((interaction, mut image, play_button)) =
        button_query.get_single_mut()
    {
        match *interaction {
            Interaction::Clicked => {
                // Play button sound
                let sound_data = audio_assets
                    .get(&sample_pack.button)
                    .unwrap()
                    .get()
                    .with_settings(
                        StaticSoundSettings::new()
                            .volume(0.5)
                            .output_destination(kira_manager.get_master()),
                    );
                kira_manager.play(sound_data).unwrap();

                // Animate
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
