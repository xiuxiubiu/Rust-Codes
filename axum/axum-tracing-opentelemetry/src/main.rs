use axum::{routing, Router};
use opentelemetry::trace::{TraceContextExt, TracerProvider as _};
use opentelemetry_sdk::trace::TracerProvider;
use serde::{ser::SerializeMap, Serializer};
use std::io;
use tower::ServiceBuilder;
use tower_http::trace::TraceLayer;
use tracing::{metadata::LevelFilter, span, Subscriber};
use tracing_opentelemetry::OpenTelemetrySpanExt;
use tracing_serde::{fields::AsMap, AsSerde};
use tracing_subscriber::{
    fmt::{self, FormatEvent, FormatFields},
    layer::SubscriberExt,
    registry::LookupSpan,
    util::SubscriberInitExt,
    Layer,
};

pub struct WriteAdaptor<'a> {
    fmt_write: &'a mut dyn std::fmt::Write,
}

impl<'a> WriteAdaptor<'a> {
    pub fn new(fmt_write: &'a mut dyn std::fmt::Write) -> Self {
        Self { fmt_write }
    }
}

impl<'a> io::Write for WriteAdaptor<'a> {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        let s =
            std::str::from_utf8(buf).map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;

        self.fmt_write
            .write_str(s)
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;

        Ok(s.as_bytes().len())
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

pub struct TraceIDFormater;

impl<S, N> FormatEvent<S, N> for TraceIDFormater
where
    S: Subscriber + for<'a> LookupSpan<'a>,
    N: for<'a> FormatFields<'a> + 'static,
{
    fn format_event(
        &self,
        _ctx: &tracing_subscriber::fmt::FmtContext<'_, S, N>,
        mut writer: tracing_subscriber::fmt::format::Writer<'_>,
        event: &tracing::Event<'_>,
    ) -> std::fmt::Result {
        let meta = event.metadata();

        let mut visit = || {
            let mut serializer = serde_json::Serializer::new(WriteAdaptor::new(&mut writer));
            let mut serializer = serializer.serialize_map(None)?;
            serializer
                .serialize_entry("timestamp", &chrono::Local::now().naive_local().to_string())?;
            serializer.serialize_entry("level", &meta.level().as_serde())?;
            serializer.serialize_entry("fields", &event.field_map())?;
            serializer.serialize_entry("target", meta.target())?;
            serializer.serialize_entry(
                "trace_id",
                &span::Span::current()
                    .context()
                    .span()
                    .span_context()
                    .trace_id()
                    .to_string(),
            )?;

            serializer.end()
        };

        visit().map_err(|_| std::fmt::Error)?;
        writeln!(writer)
    }
}

#[tokio::main]
async fn main() {
    let provider = TracerProvider::builder().build();
    let tracer = provider.tracer("example");

    let telemetry = tracing_opentelemetry::layer().with_tracer(tracer);

    tracing_subscriber::Registry::default()
        .with(telemetry)
        .with(
            fmt::layer()
                .event_format(TraceIDFormater)
                .with_filter(LevelFilter::INFO),
        )
        .init();

    let service = ServiceBuilder::new().layer(TraceLayer::new_for_http());
    let app = Router::new().route("/", routing::get(hello)).layer(service);

    axum::Server::bind(&"127.0.0.1:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn hello() -> &'static str {
    tracing::info!("hello world!");
    "hello"
}
