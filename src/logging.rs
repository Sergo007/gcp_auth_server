use tracing_core::Level;
use tracing_json2::JsonLayer;
#[allow(unused_imports)]
use tracing_subscriber::{
    filter,
    filter::{LevelFilter, Targets},
    prelude::*,
};

#[derive(Debug, PartialEq)]
enum Format {
    Json, // Json variant for JSON format
    Text, // Text variant for plain text format
}

fn get_filter() -> Vec<(&'static str, LevelFilter)> {
    vec![
        ("handlebars", LevelFilter::WARN),
        ("rustls", LevelFilter::WARN),
        ("actix_http", LevelFilter::WARN),
        ("actix_web", LevelFilter::WARN),
        ("actix_server", LevelFilter::WARN),
        ("mio", LevelFilter::WARN),
        ("tokio_util", LevelFilter::WARN),
        ("mio", LevelFilter::WARN),
        ("async_nats", LevelFilter::INFO),
        ("h2", LevelFilter::INFO),
        ("actix_tls", LevelFilter::INFO),
        ("awc", LevelFilter::INFO),
    ]
}

impl From<String> for Format {
    fn from(value: String) -> Self {
        match value.to_lowercase().as_str() {
            "json" => Format::Json,
            "text" => Format::Text,
            _ => Format::Text,
        }
    }
}

#[allow(dead_code)]
fn init_text() {
    let stdout_log = tracing_subscriber::fmt::layer()
        .compact()
        .with_thread_ids(true)
        .with_file(true)
        .with_line_number(true)
        .with_target(true);
    let filter = get_filter();
    tracing_subscriber::registry()
        // .with()
        // .with(filter)
        .with(stdout_log)
        .with(
            Targets::default()
                .with_targets(filter)
                .with_default(Level::TRACE),
        )
        .init();
}

#[allow(dead_code)]
fn init_json() {
    let filter = get_filter();
    tracing_subscriber::registry()
        .with(JsonLayer)
        .with(
            Targets::default()
                .with_targets(filter)
                .with_default(Level::TRACE),
        )
        .init();
}

#[allow(dead_code)]
fn init_default_logger(format: String) {
    match format.into() {
        Format::Text => init_text(),
        Format::Json => init_json(),
    }
}

#[cfg(test)]
pub fn init_testing_logger() {
    let filter = get_filter();

    // #[cfg(feature = "")]
    let layer = {
        tracing_subscriber::fmt::layer()
            .with_thread_ids(true)
            .with_file(true)
            .with_line_number(true)
            .with_test_writer()
            .with_span_events(tracing_subscriber::fmt::format::FmtSpan::ACTIVE)
            .compact()
    };

    #[cfg(feature = "")]
    let layer = { JsonLayer };

    tracing_subscriber::registry()
        .with(layer)
        .with(
            Targets::default()
                .with_targets(filter)
                .with_default(Level::TRACE),
        )
        .init();
}

pub fn init() {
    #[cfg(not(test))]
    {
        // let c = crate::config::Logging::from_env();
        init_default_logger("text".to_string());
    }

    #[cfg(test)]
    {
        init_testing_logger();
    }
}

//region *** tests function here ***
#[cfg(test)]
mod logging_tests {
    use super::*;

    #[test]
    fn test_json_format_case_insensitive() {
        assert_eq!(Format::from(String::from("JSON")), Format::Json);
        assert_eq!(Format::from(String::from("json")), Format::Json);
        assert_eq!(Format::from(String::from("Json")), Format::Json);
    }

    #[test]
    fn test_text_format_case_insensitive() {
        assert_eq!(Format::from(String::from("TEXT")), Format::Text);
        assert_eq!(Format::from(String::from("text")), Format::Text);
        assert_eq!(Format::from(String::from("Text")), Format::Text);
    }

    #[test]
    fn test_default_to_text_format_for_unrecognized_input() {
        assert_eq!(Format::from(String::from("xml")), Format::Text);
        assert_eq!(Format::from(String::from("gibberish")), Format::Text);
    }
}
//endregion
