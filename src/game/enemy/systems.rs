use std::time::Duration;

use bevy::{
    prelude::*, sprite::Anchor, text::Text2dBounds, utils::HashSet,
    window::PrimaryWindow,
};
use bevy_rapier2d::prelude::*;
use bevy_tweening::{lens::TransformScaleLens, Animator, EaseFunction, Tween};
use rand::prelude::*;

// ───── Current Crate Imports ────────────────────────────────────────────── //

use super::{
    components::{DogType, Enemy, MessageBox, NoteParticle, PatchOfLight},
    resources::OneDog,
    *,
};
use crate::{
    audio::assets::AudioSource,
    game::{
        enemy::components::BoyAnimation, fish::FISH_SIZE,
        gui::components::Avatar, score::ScoreUpdateEvent,
    },
    helper_functions::*,
};
use crate::{audio::resources::KiraManager, game::player::DOG_SIZE};
use crate::{
    audio::resources::SamplePack, game::fish::components::FishWasPickedEvent,
};

// ───── Body ─────────────────────────────────────────────────────────────── //

enum PhraseType {
    Hello(DogType),
    Rotation,
    Picking,
}

pub fn load_resources(mut commands: Commands, asset_server: Res<AssetServer>) {
    let names: Handle<DogData> = asset_server.load("json_data/dogs_data.json");
    let mut dogs: Vec<OneDog> = Vec::new();
    let mut avatars: Vec<Handle<Image>> = Vec::new();

    for i in 1..11 {
        let id = format!("{}", i);
        dogs.push(OneDog {
            texture_identifier: id.clone(),
            texture: asset_server.load(format!("sprites/Dogs/Face{}.png", id)),
            avatar: asset_server
                .load(format!("sprites/Avatars/Frame Dog {}.png", id)),
        });
    }

    let id = String::from("FaceHarry");
    dogs.push(OneDog {
        texture_identifier: id.clone(),
        texture: asset_server.load(format!("sprites/Dogs/{}.png", id)),
        avatar: asset_server.load("sprites/Avatars/Frame Harry.png"),
    });
    let id = String::from("FaceBigBoy");
    dogs.push(OneDog {
        texture_identifier: id.clone(),
        texture: asset_server.load(format!("sprites/Dogs/{}.png", id)),
        avatar: asset_server.load("sprites/Avatars/Frame BigBoy.png"),
    });

    let mut rng: StdRng =
        rand::SeedableRng::from_seed(rand::thread_rng().gen());
    dogs.shuffle(&mut rng);
    avatars.shuffle(&mut rng);

    commands.insert_resource(DogResource {
        json_data: names,
        dogs,
    });
}

pub fn despawn_enemies(
    mut commands: Commands,
    enemy_query: Query<Entity, With<Enemy>>,
) {
    for enemy in enemy_query.iter() {
        commands.entity(enemy).despawn_recursive();
    }
}

pub fn enemy_movement(
    mut enemy_query: Query<(Entity, Option<&mut Velocity>, &mut Enemy)>,
    time: Res<Time>,
    mut message_box_request: EventWriter<MessageBoxRequest>,
    dogs_resource: Res<DogResource>,
    assets: Res<Assets<DogData>>,
) {
    for (entity, velocity, mut enemy) in enemy_query.iter_mut() {
        let direction = enemy.direction.extend(0.);
        if let Some(mut velocity) = velocity {
            velocity.linvel =
                direction.truncate() * ENEMY_SPEED * time.delta_seconds();
            if velocity.angvel > 6.5 && enemy.phrase_timer.finished() {
                message_box_request.send(MessageBoxRequest(
                    entity,
                    generate_phrase(
                        &dogs_resource,
                        &assets,
                        PhraseType::Rotation,
                    ),
                ));
                enemy.phrase_timer = generate_phrase_timer();
            }
        }
    }
}

pub fn enemy_chatting(
    mut enemy_query: Query<(Entity, &mut Enemy)>,
    time: Res<Time>,
    dogs_resource: Res<DogResource>,
    assets: Res<Assets<DogData>>,
    mut message_box_request: EventWriter<MessageBoxRequest>,
    mut picked_fish_events: EventReader<FishWasPickedEvent>,
) {
    let events: Vec<_> = picked_fish_events.iter().collect();
    // First iterate enemy_query, because we need to tick all their timers
    for (entity, mut enemy) in enemy_query.iter_mut() {
        if enemy.phrase_timer.tick(time.delta()).finished() {
            // Events need to not be consumed
            if let Some(event) = events.iter().next() {
                if event.0 == entity {
                    message_box_request.send(MessageBoxRequest(
                        entity,
                        generate_phrase(
                            &dogs_resource,
                            &assets,
                            PhraseType::Picking,
                        ),
                    ));
                    enemy.phrase_timer = generate_phrase_timer();
                }
            }
        }
    }
}

pub fn update_enemy_direction(
    mut enemy_query: Query<(Entity, &mut Enemy), With<Enemy>>,
    mut collision_event: EventReader<CollisionEvent>,
    rapier_context: Res<RapierContext>,
    mut kira_manager: NonSendMut<KiraManager>,
    audio_assets: Res<Assets<AudioSource>>,
    sample_pack: Res<SamplePack>,
) {
    let mut direction_changed = false;

    // 0b00 - empty
    // 0b01 - entity1
    // 0b10 - entity2
    // 0b11 - both
    let mut entities_flags = 0b00;

    for event in collision_event.iter() {
        if let CollisionEvent::Started(entity1, entity2, _) = event {
            // For playing audio

            // If we found a collision
            if let Some(contact_pair) =
                rapier_context.contact_pair(*entity1, *entity2)
            {
                // Is collided entity1 actually enemy?
                if let Some(mut enemy) =
                    enemy_query.iter_mut().find_map(|enemy| {
                        if enemy.0 == *entity1 {
                            Some(enemy.1)
                        } else {
                            None
                        }
                    })
                {
                    direction_changed = true;
                    enemy
                        .direction
                        .reflect(contact_pair.manifold(0).unwrap().normal());
                    // Write entity1 to flags
                    entities_flags = entities_flags | 0b01;
                }

                // Is collided entity2 actually enemy?
                if let Some(mut enemy) =
                    enemy_query.iter_mut().find_map(|enemy| {
                        if enemy.0 == *entity2 {
                            Some(enemy.1)
                        } else {
                            None
                        }
                    })
                {
                    direction_changed = true;
                    enemy
                        .direction
                        .reflect(contact_pair.manifold(0).unwrap().normal());
                    // Write entity2 to flags
                    entities_flags = entities_flags | 0b10;
                }
            }
            // TODO: profile this break disabled
            // Break if got actual collision
            // if entities_flags != 0b00 {
            //     continue;
            // }
        }
    }
    // Play audio
    if direction_changed {
        kira_manager
            .play(audio_assets.get(&sample_pack.wall_collision).unwrap().get())
            .unwrap();
    }
}

pub fn system_add_collider_to_enemy(
    window_query: Query<&Window, With<PrimaryWindow>>,
    mut commands: Commands,
    mut entity_query: Query<(Entity, &mut Enemy, &Transform)>,
    mut events: EventWriter<MessageBoxRequest>,
    dogs_resource: Res<DogResource>,
    assets: Res<Assets<DogData>>,
) {
    for (entity, mut enemy, transform) in entity_query.iter_mut() {
        if !enemy.has_collider {
            let window = window_query.single();
            let size = DOG_SIZE * enemy.scale;
            if is_in_window(window, size, transform) {
                commands
                    .entity(entity)
                    .insert(Collider::ball(DOG_SIZE.x * enemy.scale * 0.47));
                enemy.has_collider = true;

                // Send event
                events.send(MessageBoxRequest(
                    entity,
                    generate_phrase(
                        &dogs_resource,
                        &assets,
                        PhraseType::Hello(enemy.dog_type),
                    ),
                ));
            }
        }
    }
}

pub fn spawn_message_box(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut message_box_show_events: EventReader<MessageBoxRequest>,
    entity_query: Query<(&Children, &Transform)>,
    mut kira_manager: NonSendMut<KiraManager>,
    audio_assets: Res<Assets<AudioSource>>,
    sample_pack: Res<SamplePack>,
) {
    for event in message_box_show_events.iter() {
        let (children, transform) = entity_query.get(event.0).unwrap();
        let mut message_box_transform = Transform::default();
        rotate_transform_with_parent_calibration(
            &transform.rotation,
            &mut message_box_transform,
            Vec2::NEG_X,
            Vec2::NEG_X,
            None,
        );
        let message_box = commands
            .spawn((
                SpriteBundle {
                    sprite: Sprite {
                        anchor: Anchor::Custom(Vec2::new(0.45, -0.45)),
                        ..default()
                    },
                    transform: Transform {
                        scale: Vec3::new(0.3, 0.3, 1.),
                        ..message_box_transform
                    },
                    texture: asset_server.load("sprites/Message icon.png"),
                    ..default()
                },
                MessageBox(Timer::from_seconds(3.5, TimerMode::Once), None),
            ))
            .with_children(|parent| {
                parent.spawn(Text2dBundle {
                    transform: Transform::from_translation(Vec3::new(
                        -160.4, 180.2, 1.,
                    )),
                    text: Text::from_section(
                        &event.1,
                        TextStyle {
                            color: Color::BLACK,
                            font_size: 65.5,
                            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                        },
                    )
                    .with_alignment(TextAlignment::Center),
                    text_2d_bounds: Text2dBounds {
                        size: Vec2::new(320.3, 186.1),
                    },

                    ..default()
                });
            })
            .id();
        if let Some(ch) = children.iter().next() {
            commands.entity(*ch).push_children(&[message_box]);
        }
        // Hello bark sound
        kira_manager
            .play(
                audio_assets
                    .get(get_random_bark(&sample_pack))
                    .unwrap()
                    .get(),
            )
            .unwrap();
    }
}

pub fn spawn_enemy_on_game_progress(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
    mut dogs_resource: ResMut<DogResource>,
    names_assets: Res<Assets<DogData>>,
    mut already_spawned_data: Local<(HashSet<String>, u8)>,
    mut picked_event: EventReader<ScoreUpdateEvent>,
    mut arriving_event: EventWriter<EnemyIsArrivingEvent>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let event = picked_event
        .iter()
        .find(|event| event.name.as_str().eq("Kitty"));

    // If we didn't found any Kitty in these events return
    if let None = event {
        return;
    }

    let score = event.unwrap().event_type.get_score();

    if score % 7 == 0 {
        let window = window_query.get_single().unwrap();
        let center = Vec2::new(window.width() / 2., window.height() / 2.);

        let mut rand_point = Vec2::new_rand();
        rand_point *= 1000.;
        rand_point += center;

        let direction = (center - rand_point).normalize();

        let (name, texture, scale_modifier, dog_type, avatar) = generate_dog(
            &mut dogs_resource,
            names_assets,
            &mut already_spawned_data,
        );

        let mut rng = rand::thread_rng();
        let mut angvel = if rng.gen_range(0..50) > 1 { 0.3 } else { 7. };
        angvel *= if rng.gen::<bool>() { 1. } else { -1. };

        let entity = commands
            .spawn((
                RigidBody::Dynamic,
                Velocity {
                    linvel: direction,
                    angvel,
                },
                Sleeping::disabled(),
                ActiveCollisionTypes::all(),
                ActiveEvents::COLLISION_EVENTS,
                Enemy {
                    direction,
                    has_collider: false,
                    scale: scale_modifier,
                    dog_type,
                    phrase_timer: Timer::from_seconds(12., TimerMode::Once),
                },
                avatar,
                Name::new(name.clone()),
            ))
            .with_children(|parent| {
                parent
                    .spawn(SpriteBundle {
                        sprite: Sprite {
                            custom_size: Some(
                                Vec2::splat(125.) * scale_modifier,
                            ),
                            ..default()
                        },
                        texture: asset_server
                            .load("sprites/Dog's spacesuit.png"),
                        transform: Transform::from_xyz(
                            -12.8 * 2. * scale_modifier,
                            13.2 * 2. * scale_modifier,
                            1.,
                        ),
                        ..default()
                    })
                    .with_children(|parent| {
                        parent.spawn((
                            SpriteBundle {
                                sprite: Sprite {
                                    custom_size: Some(
                                        Vec2::new(
                                            104. * 2. * scale_modifier,
                                            101. * 2. * scale_modifier,
                                        ) / 5.,
                                    ),
                                    anchor: Anchor::Custom(Vec2::new(
                                        0.55, -0.55,
                                    )),
                                    ..default()
                                },
                                texture: asset_server
                                    .load("sprites/Light reflection.png"),
                                transform: Transform::from_xyz(0., 0., 1.),
                                ..default()
                            },
                            PatchOfLight,
                        ));
                    });
            })
            .id();

        fn insert_ordinary_sprite(
            commands: &mut Commands,
            entity: Entity,
            scale_modifier: f32,
            rand_point: Vec2,
            texture: Handle<Image>,
        ) {
            commands.entity(entity).insert(SpriteBundle {
                sprite: Sprite {
                    custom_size: Some(DOG_SIZE * scale_modifier),
                    ..default()
                },
                transform: Transform::from_xyz(rand_point.x, rand_point.y, 10.),
                texture,
                ..default()
            });
        }

        match dog_type {
            DogType::Harry => {
                let wand = commands
                    .spawn(SpriteBundle {
                        transform: Transform {
                            translation: Vec3::new(-38.3, -42.8, -0.5),
                            rotation: Quat::from_rotation_z(-4.2),
                            scale: Vec3::new(0.2, 0.2, 0.),
                            ..default()
                        },
                        texture: asset_server.load("sprites/Magic wand.png"),
                        ..default()
                    })
                    .id();
                commands.entity(entity).push_children(&[wand]);
                insert_ordinary_sprite(
                    &mut commands,
                    entity,
                    scale_modifier,
                    rand_point,
                    texture,
                );
            }
            DogType::BigBoy => {
                // Load SpriteSheet
                let texture_atlas =
                    texture_atlases.add(TextureAtlas::from_grid(
                        asset_server.load("sprites/Big Boy Spritesheet.png"),
                        Vec2::new(313., 309.),
                        4,
                        3,
                        None,
                        None,
                    ));
                let timer = Timer::from_seconds(0.3, TimerMode::Repeating);
                commands.entity(entity).insert(BoyAnimation(timer, 11));
                commands.entity(entity).insert(SpriteSheetBundle {
                    sprite: TextureAtlasSprite {
                        custom_size: Some(DOG_SIZE * scale_modifier),
                        index: 0,
                        ..default()
                    },
                    texture_atlas,
                    transform: Transform::from_xyz(
                        rand_point.x,
                        rand_point.y,
                        10.,
                    ),
                    ..default()
                });
            }
            _ => {
                insert_ordinary_sprite(
                    &mut commands,
                    entity,
                    scale_modifier,
                    rand_point,
                    texture,
                );
            }
        }
        arriving_event.send(EnemyIsArrivingEvent(name));
    }
}

pub fn emit_notes(
    mut commands: Commands,
    dogs_query: Query<&GlobalTransform, With<Enemy>>,
    asset_server: Res<AssetServer>,
    mut doggy_theme_events: EventReader<DoggyTheme>,
) {
    for _ in doggy_theme_events.iter() {
        for dog_transform in dogs_query.iter() {
            let mut rng = rand::thread_rng();

            for i in 0..3 {
                let angle = std::f32::consts::PI * 2.0 / 3.0 * i as f32;
                let direction = Vec2::ONE.rotated(angle);
                let velocity = 100.;
                let timer = Timer::from_seconds(1.5, TimerMode::Once);
                let texture = get_random_note_texture(&asset_server);

                commands.spawn((
                    SpriteBundle {
                        sprite: Sprite {
                            custom_size: Some(Vec2::new(279., 270.) / 5.9),
                            //
                            ..default()
                        },
                        transform: Transform::from_translation(
                            dog_transform.translation(),
                        )
                        .with_rotation(Quat::from_rotation_z(rng.gen())),
                        texture,
                        ..default()
                    },
                    NoteParticle {
                        direction,
                        velocity,
                        timer,
                    },
                ));
            }
        }
    }
}

pub fn poll_and_despawn_collision_particles(
    mut commands: Commands,
    mut particles_query: Query<(
        Entity,
        &mut Sprite,
        &mut Transform,
        &mut NoteParticle,
    )>,
    time: Res<Time>,
) {
    for (entity, mut sprite, mut transform, mut particle) in
        particles_query.iter_mut()
    {
        if particle.timer.tick(time.delta()).finished() {
            commands.entity(entity).despawn();
        } else {
            let x =
                particle.direction.x * time.delta_seconds() * particle.velocity;
            let y =
                particle.direction.y * time.delta_seconds() * particle.velocity;

            transform.translation.x += x;
            transform.translation.y += y;
            sprite.color.set_a(particle.timer.percent_left());
            particle.velocity = particle.velocity - time.delta_seconds() * 87.;
        }
    }
}

pub fn rotate_patch_of_light(
    mut patch_query: Query<(Entity, &mut Transform), With<PatchOfLight>>,
    parents_query: Query<&Parent>,
    transforms_query: Query<&Transform, Without<PatchOfLight>>,
) {
    for (entity, mut transform) in patch_query.iter_mut() {
        let mut common_rotation = Quat::IDENTITY;
        // Collect all parents rotation
        for parent in parents_query.iter_ancestors(entity) {
            if let Ok(transform) = transforms_query.get(parent) {
                common_rotation = common_rotation.mul_quat(transform.rotation);
            }
        }
        // Apply rotation to patch
        rotate_transform_with_parent_calibration(
            &common_rotation,
            &mut transform,
            Vec2::NEG_X,
            Vec2::NEG_X,
            None,
        );
    }
}

pub fn despawn_message_box(
    mut commands: Commands,
    mut message_box_query: Query<(&mut MessageBox, Entity)>,
    time: Res<Time>,
) {
    for (mut message_box, entity) in message_box_query.iter_mut() {
        if let Some(ref mut timer) = message_box.1 {
            if timer.tick(time.delta()).just_finished() {
                commands.entity(entity).despawn_recursive();
            }
        }
    }
}

pub fn update_message_box(
    mut commands: Commands,
    mut message_box_query: Query<(&mut MessageBox, Entity, &mut Transform)>,
    parents_query: Query<&Parent>,
    transforms_query: Query<&Transform, (With<Enemy>, Without<MessageBox>)>,
    time: Res<Time>,
) {
    for (mut message_box, entity, mut transform) in message_box_query.iter_mut()
    {
        if message_box.0.tick(time.delta()).just_finished() {
            let scale_tween = Tween::new(
                EaseFunction::CubicOut,
                Duration::from_millis(700),
                TransformScaleLens {
                    start: Vec3::splat(0.3),
                    end: Vec3::splat(0.),
                },
            );
            message_box.1 = Some(Timer::from_seconds(0.7, TimerMode::Once));
            commands.entity(entity).insert(Animator::new(scale_tween));
        } else {
            let mut common_rotation = Quat::IDENTITY;
            // Collect all parents rotation
            for parent in parents_query.iter_ancestors(entity) {
                if let Ok(transform) = transforms_query.get(parent) {
                    common_rotation =
                        common_rotation.mul_quat(transform.rotation);
                }
            }
            // Apply rotation to patch
            rotate_transform_with_parent_calibration(
                &common_rotation,
                &mut transform,
                Vec2::NEG_X,
                Vec2::NEG_X,
                None,
            );
        }
    }
}

pub fn animate_big_boy(
    mut big_boy_query: Query<
        (&mut TextureAtlasSprite, &mut BoyAnimation),
        With<Enemy>,
    >,
    time: Res<Time>,
) {
    for (mut atlas, mut animation) in big_boy_query.iter_mut() {
        if animation.0.tick(time.delta()).finished() {
            atlas.index = if atlas.index == animation.1 as usize {
                0
            } else {
                atlas.index + 1
            };
        }
    }
}

fn generate_dog(
    dogs_resource: &mut ResMut<DogResource>,
    assets: Res<Assets<DogData>>,
    already_spawned_data: &mut Local<(HashSet<String>, u8)>,
) -> (String, Handle<Image>, f32, DogType, Avatar) {
    // Rand
    let mut rng = rand::thread_rng();
    let last_name_possibility = rng.gen::<bool>();
    let nickname_possibility = rng.gen::<bool>() && last_name_possibility;

    // Get handles
    let dogs_data = assets.get(&dogs_resource.json_data).unwrap();
    let id = dogs_resource.dogs[already_spawned_data.1 as usize]
        .texture_identifier
        .clone();
    let image = dogs_resource.dogs[already_spawned_data.1 as usize]
        .texture
        .clone();
    let avatar = Avatar(
        dogs_resource.dogs[already_spawned_data.1 as usize]
            .avatar
            .clone(),
    );

    {
        if already_spawned_data.1 < 11 {
            already_spawned_data.1 += 1;
        } else {
            already_spawned_data.1 = 0;
            // Shuffle sprites on next round of spawning
            dogs_resource.dogs.shuffle(&mut rng);
        }
    }
    let mut default_scale = 0.5;
    let mut default_dog_type = DogType::Ordinary;

    // Handle Doggy Potter case
    if &id == "FaceHarry" {
        return (
            String::from("Doggy Potter"),
            image,
            default_scale,
            DogType::Harry,
            avatar,
        );
    }

    // Handle Big Kid case
    if &id == "FaceBigBoy" {
        default_scale = 1.0;
        default_dog_type = DogType::BigBoy;
    }

    // Generate name
    let mut name = String::new();
    loop {
        name.push_str(
            &dogs_data.first_names
                [rng.gen_range(0..dogs_data.first_names.len())],
        );
        if last_name_possibility {
            name = dogs_data.last_names
                [rng.gen_range(0..dogs_data.last_names.len())]
            .to_string()
                + " "
                + &name;
        }
        if nickname_possibility {
            name = name
                + " aka "
                + &dogs_data.nicknames
                    [rng.gen_range(0..dogs_data.nicknames.len())]
                .to_string();
        }
        if !already_spawned_data.0.contains(&name) {
            break;
        }
    }

    (name, image, default_scale, default_dog_type, avatar)
}

fn is_in_window(window: &Window, size: Vec2, transform: &Transform) -> bool {
    let pos = transform.translation.truncate();
    let size = size + Vec2::new(2., 2.);
    pos.x - size.x > 0.0
        && pos.x + size.x < window.width()
        && pos.y - size.y > 0.0
        && pos.y + size.y < window.height()
}

fn get_random_bark<'a>(
    sample_pack: &'a Res<SamplePack>,
) -> &'a Handle<AudioSource> {
    match rand::thread_rng().gen_range(0..12) {
        0 => &sample_pack.bark1,
        1 => &sample_pack.bark2,
        2 => &sample_pack.bark3,
        3 => &sample_pack.bark4,
        4 => &sample_pack.bark5,
        5 => &sample_pack.bark6,
        6 => &sample_pack.bark7,
        7 => &sample_pack.bark8,
        8 => &sample_pack.bark9,
        9 => &sample_pack.bark10,
        10 => &sample_pack.bark11,
        11 => &sample_pack.bark12,
        _ => unreachable!(),
    }
}

fn generate_phrase(
    dogs_resource: &Res<DogResource>,
    assets: &Res<Assets<DogData>>,
    phrs_type: PhraseType,
) -> String {
    // Rand
    let mut rand = rand::thread_rng();

    // Get handles
    let vec = match phrs_type {
        PhraseType::Hello(dog_type) => match dog_type {
            DogType::Harry => {
                &assets.get(&dogs_resource.json_data).unwrap().potters_hellos
            }
            _ => &assets.get(&dogs_resource.json_data).unwrap().hellos,
        },
        PhraseType::Rotation => {
            &assets
                .get(&dogs_resource.json_data)
                .unwrap()
                .wildly_rotations
        }
        PhraseType::Picking => {
            &assets.get(&dogs_resource.json_data).unwrap().fish_picking
        }
    };

    vec.choose(&mut rand).unwrap_or(&String::new()).clone()
}

fn generate_phrase_timer() -> Timer {
    let rand = rand::thread_rng().gen_range(3.0..10.0);
    Timer::from_seconds(rand, TimerMode::Once)
}

fn get_random_note_texture(asset_server: &Res<AssetServer>) -> Handle<Image> {
    let idx = rand::thread_rng().gen_range(0..7);

    match idx {
        0 => asset_server.load("sprites/Notes/Dotted half note.png"),
        1 => asset_server.load("sprites/Notes/Eighth note.png"),
        2 => asset_server.load("sprites/Notes/Eighth notes down.png"),
        3 => asset_server.load("sprites/Notes/Eighth notes up.png"),
        4 => asset_server.load("sprites/Notes/Half note.png"),
        5 => asset_server.load("sprites/Notes/Quarter note down.png"),
        6 => asset_server.load("sprites/Notes/Quarter note up.png"),
        _ => asset_server.load("sprites/Notes/Sixteenth notes.png"),
    }
}
