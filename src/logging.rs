use tracing_core;
use tracing_core::Level;
#[allow(unused_imports)]
use tracing_subscriber::{
    filter,
    filter::{LevelFilter, Targets},
    prelude::*,
};

#[derive(Debug, PartialEq)]
pub enum Format {
    Json,
    Text,
}

#[allow(dead_code)]
fn get_filter() -> Vec<(&'static str, LevelFilter)> {
    vec![("fred", LevelFilter::ERROR)]
}

#[allow(dead_code)]
fn parse_format(format: String) -> Format {
    match format.to_lowercase().as_str() {
        "json" => Format::Json,
        "text" => Format::Text,
        _ => Format::Text,
    }
}

#[allow(dead_code)]
fn init_text() {
    let stdout_log = tracing_subscriber::fmt::layer().compact();
    let filter = get_filter();
    tracing_subscriber::registry()
        // .with()
        // .with(filter)
        .with(stdout_log)
        .with(
            Targets::default()
                .with_targets(filter)
                .with_default(Level::DEBUG),
        )
        .init();
}

#[allow(dead_code)]
fn init_json() {
    let stdout_log = tracing_subscriber::fmt::layer().json();
    let filter = get_filter();
    tracing_subscriber::registry()
        // .with()
        // .with(filter)
        .with(stdout_log)
        .with(
            Targets::default()
                .with_targets(filter)
                .with_default(Level::DEBUG),
        )
        .init();
}

#[allow(dead_code)]
fn init_default_logger(format: String) {
    let format = parse_format(format);
    match format {
        Format::Text => init_text(),
        Format::Json => init_json(),
    }
}

#[cfg(test)]
pub fn init_testing_logger() {
    let stdout_log = tracing_subscriber::fmt::layer()
        .with_test_writer()
        .compact();

    tracing_subscriber::registry()
        .with(stdout_log.with_filter(filter::LevelFilter::DEBUG))
        .init();
}

pub fn init() {
    #[cfg(not(test))]
    {
        // let c = crate::config::Logging::from_env();
        // init_default_logger(c.format);
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
    fn test_parse_format_json() {
        let f = parse_format(String::from("json"));
        assert_eq!(f, Format::Json)
    }

    #[test]
    fn test_parse_format_text() {
        let f = parse_format(String::from("text"));
        assert_eq!(f, Format::Text)
    }

    #[test]
    fn test_parse_format_unknown() {
        let f = parse_format(String::from("bla"));
        assert_eq!(f, Format::Text)
    }
}
//endregion
