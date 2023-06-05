use bevy::diagnostic::{
    Diagnostics, EntityCountDiagnosticsPlugin, FrameTimeDiagnosticsPlugin,
};
use bevy::{prelude::*, window::PrimaryWindow};
use bevy_inspector_egui::{
    bevy_egui::{EguiContexts, EguiPlugin},
    egui,
    quick::WorldInspectorPlugin,
};
use bevy_rapier2d::render::RapierDebugRenderPlugin;

// ───── Body ─────────────────────────────────────────────────────────────── //

pub struct DebugPlugin;

#[derive(Component)]
struct UpdateFpsTimer {
    timer: Timer,
    current_fps: f32,
}

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        #[cfg(debug_assertions)]
        app
            // Plugins
            .add_plugin(EguiPlugin)
            .add_plugin(WorldInspectorPlugin::new())
            .add_plugin(RapierDebugRenderPlugin::default())
            .add_plugin(FrameTimeDiagnosticsPlugin::default())
            .add_plugin(EntityCountDiagnosticsPlugin::default())
            // Startup Systems
            .add_startup_system(setup)
            // Systems
            .add_system(update_info_window.in_base_set(CoreSet::Update));
    }
}

fn setup(mut commands: Commands) {
    commands.spawn(UpdateFpsTimer {
        timer: Timer::from_seconds(0.1, TimerMode::Repeating),
        current_fps: 0.,
    });
}

fn update_info_window(
    diagnostics: Res<Diagnostics>,
    camera_query: Query<(&GlobalTransform, &Camera)>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    mut timer_query: Query<&mut UpdateFpsTimer>,
    time: Res<Time>,
    // Egui
    mut contexts: EguiContexts,
) {
    let (camera_transform, camera) = camera_query.single();
    let window = window_query.single();
    let world_position = window
        .cursor_position()
        .and_then(|cursor| {
            camera.viewport_to_world_2d(camera_transform, cursor)
        })
        .unwrap_or(Vec2::default());

    egui::Window::new("Info").default_open(false).show(
        contexts.ctx_mut(),
        |ui| {
            egui::Grid::new("Properties")
                .num_columns(2)
                .spacing([40., 4.])
                .striped(true)
                .show(ui, |ui| {
                    ui.label("In world position");
                    ui.label(format!(
                        "x: {:.2}, y: {:.2}",
                        world_position.x, world_position.y
                    ));
                    ui.end_row();
                    let mut timer = timer_query.single_mut();
                    let mut fps = timer.current_fps;
                    if timer.timer.tick(time.delta()).finished() {
                        fps = extract_fps(&diagnostics).unwrap_or(fps as f64)
                            as f32;
                        timer.current_fps = fps;
                    }
                    ui.label("Fps");
                    ui.label(format!("{:.0}", fps));
                    ui.end_row();
                    ui.label("Entities count");
                    ui.label(format!(
                        "{}",
                        extract_entities_count(&diagnostics).unwrap_or(0.)
                    ));
                })
        },
    );
}

fn extract_fps(diagnostics: &Res<Diagnostics>) -> Option<f64> {
    diagnostics
        .get(bevy::diagnostic::FrameTimeDiagnosticsPlugin::FPS)
        .and_then(|fps| fps.average())
}

fn extract_entities_count(diagnostics: &Res<Diagnostics>) -> Option<f64> {
    diagnostics
        .get(bevy::diagnostic::EntityCountDiagnosticsPlugin::ENTITY_COUNT)
        .and_then(|value| value.value())
}
