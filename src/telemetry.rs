use cfg_if::cfg_if;

cfg_if! {
    if #[cfg(feature = "ssr")] {

use tracing::subscriber::set_global_default;
use tracing::Subscriber;
use tracing_bunyan_formatter::{BunyanFormattingLayer, JsonStorageLayer};
use tracing_log::LogTracer;
//use tracing_subscriber::prelude::*;
use tracing_subscriber::fmt::MakeWriter;
use tracing_subscriber::fmt::time;
use tracing_subscriber::{layer::SubscriberExt, EnvFilter, Registry};
//use tracing_logfmt;

// Compose multiple layers into a `tracing`'s subscriber.
///
/// # Implementation Notes
///
/// We are using `impl Subscriber` as return type to avoid having to
/// spell out the actual type of the returned subscriber, which is
/// indeed quite complex.
/// We need to explicitly call out that the returned subscriber is
/// `Send` and `Sync` to make it possible to pass it to `init_subscriber`
/// later on.
pub async fn get_subscriber_with_tracing<Sink>(
    name: String,
    env_filter: String,
    sink: Sink,
) -> impl Subscriber + Sync + Send
    where
    // This "weird" syntax is a higher-ranked trait bound (HRTB)
    // It basically means that Sink implements the `MakeWriter`
    // trait for all choices of the lifetime parameter `'a`
    // Check out https://doc.rust-lang.org/nomicon/hrtb.html
    // for more details.
        Sink: for<'a> MakeWriter<'a> + Send + Sync + 'static,
{
    let env_filter = EnvFilter::try_from_default_env().unwrap_or_else(|e| {
        print!("RUST_LOG not set! Defaulting to {}: {:#?}", e, env_filter);
        EnvFilter::new(env_filter)
    });
    let _formatting_layer = BunyanFormattingLayer::new(name, sink);
    let fmt = tracing_subscriber::fmt::format()
        .pretty()
        .with_timer(time::Uptime::default());
    let fmt_layer = tracing_subscriber::fmt::layer()
        .event_format(fmt)
        .with_thread_ids(true)
        .with_file(true)
        .with_line_number(true)
        .with_target(false);
    Registry::default()
        .with(env_filter)
        //.with(JsonStorageLayer)
        //.with(tracing_logfmt::layer())
        .with(fmt_layer)
}
pub fn get_subscriber<Sink>(
    name: String,
    env_filter: String,
    sink: Sink,
) -> impl Subscriber + Sync + Send
    where
    // This "weird" syntax is a higher-ranked trait bound (HRTB)
    // It basically means that Sink implements the `MakeWriter`
    // trait for all choices of the lifetime parameter `'a`
    // Check out https://doc.rust-lang.org/nomicon/hrtb.html
    // for more details.
        Sink: for<'a> MakeWriter<'a> + Send + Sync + 'static,
{
    let env_filter =
        EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new(env_filter));
    let formatting_layer = BunyanFormattingLayer::new(name, sink);

    Registry::default()
        .with(env_filter)
        .with(JsonStorageLayer)
        .with(formatting_layer)
}

/// Register a subscriber as global default to process span data.
///
/// It should only be called once!
pub fn init_subscriber(subscriber: impl Subscriber + Send + Sync) {
    let res = LogTracer::init();
    res.ok();
    set_global_default(subscriber).ok();
}

    }
}
