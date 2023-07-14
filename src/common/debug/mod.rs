use bevy::diagnostic::{
    Diagnostic, DiagnosticsStore, RegisterDiagnostic,
    SystemInformationDiagnosticsPlugin,
};
#[cfg(debug_assertions)]
use bevy::diagnostic::{
    Diagnostics, EntityCountDiagnosticsPlugin, FrameTimeDiagnosticsPlugin,
};
use bevy::text::Text2dBounds;
use bevy::utils::HashMap;
use bevy::{prelude::*, window::PrimaryWindow};
use bevy_inspector_egui::{
    bevy_egui::{EguiContexts, EguiPlugin},
    egui,
    quick::WorldInspectorPlugin,
};
use bevy_rapier2d::render::RapierDebugRenderPlugin;

// ───── Current Crate Imports ────────────────────────────────────────────── //

pub use components::*;

// ───── Submodules ───────────────────────────────────────────────────────── //

// Top-level modules
mod components;

// ───── Body ─────────────────────────────────────────────────────────────── //

pub struct DebugPlugin;

#[derive(Component, Debug)]
struct UpdateFpsTimer {
    timer: Timer,
    current_fps: f64,
    cpu_usage: f64,
    mem_usage: f64,
}

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app.register_diagnostic(Diagnostic::new(
            FrameTimeDiagnosticsPlugin::FPS,
            "Fps",
            10,
        ))
        .register_diagnostic(Diagnostic::new(
            EntityCountDiagnosticsPlugin::ENTITY_COUNT,
            "Fps",
            10,
        ))
        .register_diagnostic(Diagnostic::new(
            SystemInformationDiagnosticsPlugin::CPU_USAGE,
            "Fps",
            10,
        ))
        .register_diagnostic(Diagnostic::new(
            SystemInformationDiagnosticsPlugin::MEM_USAGE,
            "Fps",
            10,
        ))
        // Events
        .add_event::<AddValueToDebugEvent>()
        // Plugins
        .add_plugins(EguiPlugin)
        .add_plugins(WorldInspectorPlugin::new())
        .add_plugins(RapierDebugRenderPlugin::default())
        .add_plugins(FrameTimeDiagnosticsPlugin::default())
        .add_plugins(EntityCountDiagnosticsPlugin::default())
        .register_type::<Text2dBounds>()
        // .add_plugin(SystemInformationDiagnosticsPlugin::default())
        // Startup Systems
        .add_systems(Startup, setup)
        // Systems
        .add_systems(Update, update_info_window);
        // .add_system(update_values_window.in_base_set(CoreSet::Update));
    }
}

fn setup(mut commands: Commands) {
    commands.spawn(UpdateFpsTimer {
        timer: Timer::from_seconds(0.1, TimerMode::Repeating),
        current_fps: 0.,
        cpu_usage: 0.,
        mem_usage: 0.,
    });
}

fn update_values_window(
    mut event_reader: EventReader<AddValueToDebugEvent>,
    mut contexts: EguiContexts,
    mut local: Local<HashMap<String, String>>,
) {
    // Collect
    let events: HashMap<String, String> = event_reader
        .into_iter()
        .map(|element| (element.0.clone(), element.1.clone()))
        .collect();
    local.extend(events);

    // Draw
    egui::Window::new("Values").default_open(false).show(
        contexts.ctx_mut(),
        |ui| {
            egui::Grid::new("Properties")
                .num_columns(2)
                .spacing([40., 4.])
                .striped(true)
                .show(ui, |ui| {
                    for (key, value) in local.iter() {
                        ui.label(key);
                        ui.label(value);
                        ui.end_row();
                    }
                })
        },
    );
}

fn update_info_window(
    diagnostics: Res<DiagnosticsStore>,
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
                    if timer.timer.tick(time.delta()).finished() {
                        timer.current_fps = extract_fps(&diagnostics)
                            .unwrap_or(timer.current_fps);
                        timer.cpu_usage = extract_cpu_usage(&diagnostics)
                            .unwrap_or(timer.cpu_usage);
                        timer.mem_usage = extract_mem_usage(&diagnostics)
                            .unwrap_or(timer.mem_usage);
                    }
                    ui.label("Fps");
                    ui.label(format!("{:.0}", timer.current_fps));
                    ui.end_row();
                    // ui.label("Cpu usage");
                    // ui.label(format!("{:.0} %", timer.cpu_usage));
                    // ui.end_row();
                    // ui.label("Mem usage");
                    // ui.label(format!("{:.0} %", timer.mem_usage));
                    // ui.end_row();
                    ui.label("Entities count");
                    ui.label(format!(
                        "{}",
                        extract_entities_count(&diagnostics).unwrap_or(0.)
                    ));
                })
        },
    );
}

fn extract_fps(diagnostics: &Res<DiagnosticsStore>) -> Option<f64> {
    diagnostics
        .get(bevy::diagnostic::FrameTimeDiagnosticsPlugin::FPS)
        .and_then(|fps| fps.average())
}

fn extract_entities_count(diagnostics: &Res<DiagnosticsStore>) -> Option<f64> {
    diagnostics
        .get(bevy::diagnostic::EntityCountDiagnosticsPlugin::ENTITY_COUNT)
        .and_then(|value| value.value())
}

#[allow(dead_code)]
fn extract_cpu_usage(diagnostics: &Res<DiagnosticsStore>) -> Option<f64> {
    diagnostics
        .get(bevy::diagnostic::SystemInformationDiagnosticsPlugin::CPU_USAGE)
        .and_then(|value| value.value())
}

#[allow(dead_code)]
fn extract_mem_usage(diagnostics: &Res<DiagnosticsStore>) -> Option<f64> {
    diagnostics
        .get(bevy::diagnostic::SystemInformationDiagnosticsPlugin::MEM_USAGE)
        .and_then(|value| value.value())
}
