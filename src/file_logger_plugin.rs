use bevy::prelude::*;

#[cfg(feature = "file_logger")]
use tracing_subscriber::filter::LevelFilter;
#[cfg(feature = "file_logger")]
use tracing_subscriber::prelude::__tracing_subscriber_SubscriberExt;

// ───── Body ─────────────────────────────────────────────────────────────── //

pub struct FileLoggerPlugin;

#[cfg(feature = "file_logger")]
#[derive(Resource)]
struct WorkerGuardResource(tracing_appender::non_blocking::WorkerGuard);

impl Plugin for FileLoggerPlugin {
    fn build(&self, app: &mut App) {
        // Setup logging into file
        #[cfg(feature = "file_logger")]
        {
            let fmt_layer = tracing_subscriber::fmt::layer()
                .with_target(false) // don't include event targets when logging
                .with_level(false); // don't include event levels when logging
            let default_filter = {
                format!(
                    "{},{}",
                    bevy::log::Level::INFO,
                    "wgpu=error".to_string()
                )
            };
            let subscriber = tracing_subscriber::Registry::default().with(
                tracing_subscriber::EnvFilter::try_from_default_env()
                    .or_else(|_| {
                        tracing_subscriber::EnvFilter::try_new(&default_filter)
                    })
                    .unwrap(),
            );

            let file_appender =
                tracing_appender::rolling::hourly("log", "space_kitty");
            let (non_blocking, worker_guard) =
                tracing_appender::non_blocking(file_appender);
            let file_fmt_layer = tracing_subscriber::fmt::Layer::default()
                .with_ansi(false)
                .with_writer(non_blocking);
            let subscriber = subscriber.with(file_fmt_layer).with(fmt_layer);

            bevy::utils::tracing::subscriber::set_global_default(subscriber)
                .unwrap();

            app.insert_resource(WorkerGuardResource(worker_guard));
        }
    }
}
