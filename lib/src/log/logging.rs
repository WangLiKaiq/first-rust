use std::env;
use std::sync::Once;
use tracing::{Subscriber, subscriber::set_global_default};
use tracing_bunyan_formatter::{BunyanFormattingLayer, JsonStorageLayer};
use tracing_log::LogTracer;
use tracing_subscriber::fmt::MakeWriter;
use tracing_subscriber::{EnvFilter, Registry, layer::SubscriberExt};

use crate::env::load_system_properties;
static INIT: Once = Once::new();

#[ctor::ctor]
pub fn init_subscriber() {
    INIT.call_once(|| {
        load_system_properties();
        let default_filter_level = "info".to_string();
        let subscriber_name = env::var("APP").unwrap_or_else(|_| String::from("server"));
        LogTracer::init().expect("Unable to setup log tracer!");
        let subscriber = if env::var("TEST_LOG").is_ok() {
            get_subscriber(subscriber_name, &default_filter_level, std::io::stdout)
        } else {
            get_subscriber(subscriber_name, &default_filter_level, std::io::stdout)
        };
        set_global_default(subscriber).expect("Failed to set subscriber");
    });

    tracing::info!("Logging initialized globally.");
}

///<Sink> after function name is required.
/// Use a trait object (Box<dyn Subscriber + Send + Sync>) instead of impl Subscriber + Send + Sync.
/// Even though they return the same traits, the compiler sees them as different opaque types (impl Trait) — and you can’t mix different impl Trait return values in one expression like that.
fn get_subscriber<Sink>(
    name: String,
    env_filter: &String,
    sink: Sink,
) -> Box<dyn Subscriber + Send + Sync>
where
    Sink: for<'a> MakeWriter<'a> + Send + Sync + 'static,
{
    let env_filter =
        EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new(env_filter));
    let formatting_layer = BunyanFormattingLayer::new(name, sink);
    let subscriber = Registry::default()
        .with(env_filter)
        .with(JsonStorageLayer)
        .with(formatting_layer);
    Box::new(subscriber)
}
