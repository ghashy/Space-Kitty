use bevy::{
    app::AppExit,
    core_pipeline::bloom::{BloomCompositeMode, BloomSettings},
    prelude::*,
    render::camera::ScalingMode,
    window::PrimaryWindow,
};
use bevy_hanabi::*;
use bevy_rapier2d::prelude::*;
use bevy_tweening::*;
use rand::Rng;

// ───── Current Crate Imports ────────────────────────────────────────────── //

use crate::components::*;
use crate::events::*;
use crate::game::SimulationState;
use crate::resources::CometTimer;
use crate::AppState;
use crate::{animation::*, RAND_STAR_ANIMATION_TIME_RANGE};
use crate::{audio::resources::SamplePack, COMET_SPEED};

// ───── Body ─────────────────────────────────────────────────────────────── //

pub fn setup(
    mut commands: Commands,
    mut rapier_config: ResMut<RapierConfiguration>,
) {
    // Setup physics gravity
    rapier_config.gravity = Vec2::ZERO;

    commands.spawn((SpatialBundle::default(), Comets, Name::new("Comets")));
}

pub fn setup_audio_assets(
    mut commands: Commands,
    asset_server: ResMut<AssetServer>,
) {
    commands.insert_resource(SamplePack {
        pick_fish1: asset_server
            .load("audio/SFX/Crackers/Space Kitty - Cracker 1.wav"),
        pick_fish2: asset_server
            .load("audio/SFX/Crackers/Space Kitty - Cracker 2.wav"),
        pick_fish3: asset_server
            .load("audio/SFX/Crackers/Space Kitty - Cracker 3.wav"),
        pick_fish4: asset_server
            .load("audio/SFX/Crackers/Space Kitty - Cracker 4.wav"),
        pick_fish5: asset_server
            .load("audio/SFX/Crackers/Space Kitty - Cracker 5.wav"),
        pick_fish6: asset_server
            .load("audio/SFX/Crackers/Space Kitty - Cracker 6.wav"),
        pick_fish7: asset_server
            .load("audio/SFX/Crackers/Space Kitty - Cracker 7.wav"),
        pick_fish8: asset_server
            .load("audio/SFX/Crackers/Space Kitty - Cracker 8.wav"),
        pick_fish9: asset_server
            .load("audio/SFX/Crackers/Space Kitty - Cracker 9.wav"),
        pick_fish10: asset_server
            .load("audio/SFX/Crackers/Space Kitty - Cracker 10.wav"),
        pick_fish11: asset_server
            .load("audio/SFX/Crackers/Space Kitty - Cracker 11.wav"),
        pick_fish12: asset_server
            .load("audio/SFX/Crackers/Space Kitty - Cracker 12.wav"),
        pick_fish13: asset_server
            .load("audio/SFX/Crackers/Space Kitty - Cracker 13.wav"),
        pick_fish14: asset_server
            .load("audio/SFX/Crackers/Space Kitty - Cracker 14.wav"),
        pick_fish15: asset_server
            .load("audio/SFX/Crackers/Space Kitty - Cracker 15.wav"),
        pick_fish16: asset_server
            .load("audio/SFX/Crackers/Space Kitty - Cracker 16.wav"),
        pick_fish17: asset_server
            .load("audio/SFX/Crackers/Space Kitty - Cracker 17.wav"),
        pick_fish18: asset_server
            .load("audio/SFX/Crackers/Space Kitty - Cracker 18.wav"),
        pick_fish19: asset_server
            .load("audio/SFX/Crackers/Space Kitty - Cracker 19.wav"),
        pick_fish20: asset_server
            .load("audio/SFX/Crackers/Space Kitty - Cracker 20.wav"),
        pick_fish21: asset_server
            .load("audio/SFX/Crackers/Space Kitty - Cracker 21.wav"),
        bark1: asset_server.load("audio/SFX/Barks/Space Kitty - Bark 1.wav"),
        bark2: asset_server.load("audio/SFX/Barks/Space Kitty - Bark 2.wav"),
        bark3: asset_server.load("audio/SFX/Barks/Space Kitty - Bark 3.wav"),
        bark4: asset_server.load("audio/SFX/Barks/Space Kitty - Bark 4.wav"),
        bark5: asset_server.load("audio/SFX/Barks/Space Kitty - Bark 5.wav"),
        bark6: asset_server.load("audio/SFX/Barks/Space Kitty - Bark 6.wav"),
        bark7: asset_server.load("audio/SFX/Barks/Space Kitty - Bark 7.wav"),
        bark8: asset_server.load("audio/SFX/Barks/Space Kitty - Bark 8.wav"),
        bark9: asset_server.load("audio/SFX/Barks/Space Kitty - Bark 9.wav"),
        bark10: asset_server.load("audio/SFX/Barks/Space Kitty - Bark 10.wav"),
        bark11: asset_server.load("audio/SFX/Barks/Space Kitty - Bark 11.wav"),
        bark12: asset_server.load("audio/SFX/Barks/Space Kitty - Bark 12.wav"),
        meow1: asset_server.load("audio/SFX/Meows/Space Kitty - Meow 1.wav"),
        meow2: asset_server.load("audio/SFX/Meows/Space Kitty - Meow 2.wav"),
        meow3: asset_server.load("audio/SFX/Meows/Space Kitty - Meow 3.wav"),
        meow4: asset_server.load("audio/SFX/Meows/Space Kitty - Meow 4.wav"),
        meow5: asset_server.load("audio/SFX/Meows/Space Kitty - Meow 5.wav"),
        meow6: asset_server.load("audio/SFX/Meows/Space Kitty - Meow 6.wav"),
        meow7: asset_server.load("audio/SFX/Meows/Space Kitty - Meow 7.wav"),
        meowroar: asset_server
            .load("audio/SFX/Meows/Space Kitty - Meow Roar.wav"),
        wall_collision: asset_server
            .load("audio/SFX/Space Kitty - Wall collision.wav"),
        title_theme: asset_server.load("audio/Space Kitty - Main theme.ogg"),
        main_theme: asset_server
            .load("audio/Space Kitty - Title screen theme.ogg"),
        alarm: asset_server.load("audio/SFX/Space Kitty - Alarm (no life).wav"),
        engine: asset_server.load("audio/SFX/Space Kitty  - Jet engine.wav"),
    });
}

pub fn update_app_state_after_audio_loaded(
    mut next_app_state: ResMut<NextState<AppState>>,
    sample_pack: Res<SamplePack>,
    asset_server: Res<AssetServer>,
) {
    for item in sample_pack.iter() {
        match asset_server.get_load_state(item) {
            bevy::asset::LoadState::Loaded => continue,
            bevy::asset::LoadState::NotLoaded => return,
            bevy::asset::LoadState::Loading => return,
            bevy::asset::LoadState::Failed => {
                panic!("Can not load audio assets!")
            }
            bevy::asset::LoadState::Unloaded => {
                panic!("Audio assets were unloaded!")
            }
        }
    }

    next_app_state.set(AppState::MainMenu);
}

pub fn spawn_camera(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window_query.get_single().unwrap();

    commands
        .spawn(Camera2dBundle {
            camera: Camera {
                // +1 percent load on cpu when true
                hdr: true,
                ..default()
            },
            transform: Transform::from_xyz(
                window.width() / 2.,
                window.height() / 2.,
                // IMPORTANT: Camera can see only objects down by z axis!
                1000.,
            ),
            projection: OrthographicProjection {
                scale: 1.,
                scaling_mode: ScalingMode::AutoMax {
                    max_width: window.width(),
                    max_height: window.height(),
                },
                ..default()
            },
            ..default()
        })
        .insert(BloomSettings {
            intensity: 0.7,
            low_frequency_boost: 0.1,
            low_frequency_boost_curvature: 0.1,
            high_pass_frequency: 1.4,
            prefilter_settings:
                bevy::core_pipeline::bloom::BloomPrefilterSettings {
                    threshold: 2.0,
                    threshold_softness: 32.0,
                },
            composite_mode: BloomCompositeMode::Additive,
        });
}

pub fn spawn_background_texture(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let window = window_query.get_single().unwrap();
    commands.spawn(SpriteBundle {
        sprite: Sprite {
            custom_size: Some(Vec2::new(window.width(), window.height())),
            color: Color::rgba(1., 1., 1., 0.999),
            ..default()
        },
        texture: asset_server.load("sprites/Background.png"),
        transform: Transform::from_xyz(
            window.width() / 2.,
            window.height() / 2.,
            0.,
        ),
        ..default()
    });
}

pub fn spawn_dust(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut effects: ResMut<Assets<EffectAsset>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let mut gradient = Gradient::new();
    gradient.add_key(0.0, Vec4::new(1., 1., 1., 0.0));
    gradient.add_key(0.5, Vec4::new(1., 1., 1., 0.1));
    gradient.add_key(1.0, Vec4::ZERO);

    // Create the effect asset
    let effect = effects.add(
        EffectAsset {
            name: "MenuBackgroundStars".to_string(),
            capacity: 1000,
            spawner: Spawner::rate(50.0.into()),
            ..default()
        }
        .init(InitPositionSphereModifier {
            center: Vec3::ZERO,
            radius: 500.0,
            dimension: ShapeDimension::Surface,
        })
        .init(InitLifetimeModifier {
            lifetime: 7_f32.into(),
        })
        .init(InitVelocityCircleModifier {
            center: Vec3::new(5., 5., 1.),
            axis: Vec3::Z,
            speed: 100.0.into(),
        })
        .render(ParticleTextureModifier {
            texture: asset_server.load("sprites/Star glowing.png"),
        })
        .render(SizeOverLifetimeModifier {
            gradient: Gradient::constant(Vec2::splat(5.0)),
        })
        .render(ColorOverLifetimeModifier { gradient }),
    );

    let window = window_query.single();

    commands
        .spawn(ParticleEffectBundle {
            effect: ParticleEffect::new(effect).with_z_layer_2d(Some(4.)),
            transform: Transform::from_xyz(
                window.width() / 2.,
                window.height() / 2.,
                0.,
            ),
            ..default()
        })
        .insert(Name::new("StarsInMenu"));
}

pub fn spawn_background_stars(
    window_query: Query<&Window, With<PrimaryWindow>>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let window = window_query.single();
    let star_handle = asset_server.load("sprites/Star glowing.png");

    let mut children = Vec::new();

    let mut rng = rand::thread_rng();

    for index in 0..250 {
        let rand_x = rng.gen_range(0.0..window.width());
        let rand_y = rng.gen_range(0.0..window.height());
        let child = commands
            .spawn((
                SpriteBundle {
                    sprite: Sprite {
                        color: Color::WHITE,
                        custom_size: Some(Vec2::splat(rng.gen_range(1.0..2.0))),

                        ..default()
                    },
                    texture: star_handle.clone(),
                    transform: Transform::from_xyz(rand_x, rand_y, 1.),
                    ..default()
                },
                BackgroundStar {
                    index,
                    timer: Timer::from_seconds(
                        rng.gen_range(RAND_STAR_ANIMATION_TIME_RANGE),
                        TimerMode::Once,
                    ),
                },
                Name::new(format!("Star: {}", index)),
            ))
            .id();
        children.push(child);
    }
    commands
        .spawn(BackgroundStars)
        .insert(SpatialBundle {
            transform: Transform::from_xyz(0., 0., 1.),
            ..default()
        })
        .insert(Name::new("BackgroundStars"))
        .push_children(&children);
}

pub fn update_background_stars(
    mut commands: Commands,
    mut star_query: Query<(Entity, &mut BackgroundStar)>,
    time: Res<Time>,
) {
    for (entity, mut star) in star_query.iter_mut() {
        if star.timer.tick(time.delta()).just_finished() {
            animate_star(&mut commands, entity, star.index)
        }
    }
}

pub fn animate_background_stars(
    mut event_reader: EventReader<TweenCompleted>,
    mut star_query: Query<&mut BackgroundStar>,
) {
    for event in event_reader.iter() {
        let mut rng = rand::thread_rng();
        for mut star in star_query.iter_mut() {
            if event.user_data as u8 == star.index {
                star.timer.set_duration(std::time::Duration::from_secs_f32(
                    rng.gen_range(RAND_STAR_ANIMATION_TIME_RANGE),
                ));
                star.timer.reset();
                star.timer.unpause();
            }
        }
    }
}

pub fn finalize_transition_to_game(
    mut next_app_state: ResMut<NextState<AppState>>,
    mut event_reader: EventReader<TweenCompleted>,
) {
    for event in event_reader.iter() {
        if event.user_data == 300 {
            next_app_state.set(AppState::Game);
        }
    }
}

pub fn handle_pressing_g_key(
    keyboard_input: Res<Input<KeyCode>>,
    mut event_writer: EventWriter<DarkenScreenEvent>,
) {
    if keyboard_input.just_pressed(KeyCode::G) {
        event_writer.send(DarkenScreenEvent);
    }
}

pub fn handle_pressing_m_key(
    keyboard_input: Res<Input<KeyCode>>,
    app_state: Res<State<AppState>>,
    mut next_app_state: ResMut<NextState<AppState>>,
) {
    if keyboard_input.just_pressed(KeyCode::M) {
        if app_state.0 != AppState::MainMenu {
            next_app_state.set(AppState::MainMenu);
            println!("Entered AppState::MainMenu");
        }
    }
}

pub fn exit_game(
    keyboard_input: Res<Input<KeyCode>>,
    mut event_writer: EventWriter<AppExit>,
) {
    if keyboard_input.pressed(KeyCode::Escape) {
        event_writer.send(AppExit);
    }
}

pub fn handle_game_over(
    mut commands: Commands,
    mut game_over_event_reader: EventReader<GameOver>,
) {
    if let Some(event) = game_over_event_reader.iter().next() {
        println!("FinalScore: {}", event.final_score);
        commands.insert_resource(NextState(Some(AppState::GameOver)));
        commands.insert_resource(NextState(Some(SimulationState::Paused)));
    }
}

pub fn spawn_periodical_comet(
    mut commands: Commands,
    comets_group_query: Query<Entity, With<Comets>>,
    assets_server: Res<AssetServer>,
    mut timer: ResMut<CometTimer>,
    time: Res<Time>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    if timer.0.tick(time.delta()).finished() {
        let window = window_query.single();
        let mut rand = rand::thread_rng();
        let rand_x = rand.gen_range(0.0..=window.width());
        let y = window.height() + 100.;
        let (texture, comet) = get_random_comet_texture(&assets_server);
        commands
            .entity(comets_group_query.single())
            .with_children(|parent| {
                parent.spawn((
                    SpriteBundle {
                        sprite: Sprite {
                            custom_size: Some(comet.resolution / 3.),
                            ..default()
                        },
                        // After background stars, and before planets
                        transform: Transform::from_xyz(rand_x, y, 10.),
                        texture,
                        ..default()
                    },
                    comet,
                ));
            });

        // Update timer duration
        let rand_duration = rand.gen_range(1..5);
        timer
            .0
            .set_duration(std::time::Duration::from_secs(rand_duration));
    }
    //
}

pub fn despawn_outer_comets(
    mut commands: Commands,
    comets_query: Query<(Entity, &Transform), With<Comet>>,
) {
    for (entity, transform) in comets_query.iter() {
        if transform.translation.y < -100. {
            commands.entity(entity).despawn();
        }
    }
}

pub fn move_comets(
    mut comets_query: Query<(&mut Transform, &Comet)>,
    time: Res<Time>,
) {
    for (mut transform, comet) in comets_query.iter_mut() {
        transform.translation += Vec3::new(-40.37, -54.19, 0.).normalize()
            * COMET_SPEED
            * comet.speed_modifier
            * time.delta_seconds();
    }
}

fn get_random_comet_texture(
    asset_server: &Res<AssetServer>,
) -> (Handle<Image>, Comet) {
    let idx = rand::thread_rng().gen_range(0..3);

    match idx {
        0 => (
            asset_server.load("sprites/Komet Blue.png"),
            Comet::new(3., Vec2::new(254., 301.)),
        ),
        1 => (
            asset_server.load("sprites/Komet Purple.png"),
            Comet::new(1., Vec2::new(184., 213.)),
        ),
        2 => (
            asset_server.load("sprites/Komet Red.png"),
            Comet::new(2., Vec2::new(245., 293.)),
        ),
        _ => unreachable!(),
    }
}
