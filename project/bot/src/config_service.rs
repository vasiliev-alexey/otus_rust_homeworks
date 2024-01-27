use config::Config;
use std::collections::HashMap;
#[derive(Debug)]
pub(crate) struct AppConfig {
    pub(crate) course_pattern: String,
    pub(crate) source_file: String,
    pub(crate) redis_url: String,
}

const DEFAULT_PATTERN: &str = "Rust";
const DEFAULT_REDIS: &str = "redis://127.0.0.1:6379";

pub fn read_config() -> Result<AppConfig, Box<dyn std::error::Error>> {
    let home_path = format!("{}/.config/project", std::env::var("HOME").unwrap()).to_string();
    let settings = Config::builder()
        .add_source(config::File::with_name(&home_path))
        .add_source(config::Environment::with_prefix("OTUS"))
        .build()
        .unwrap();

    let settings = settings
        .try_deserialize::<HashMap<String, String>>()
        .unwrap();

    let course_pattern = settings
        .get("course_pattern")
        .unwrap_or(&DEFAULT_PATTERN.to_string())
        .to_string();

    let redis_url = settings
        .get("redis_url")
        .unwrap_or(&DEFAULT_REDIS.to_string())
        .to_string();

    let source_file = settings
        .get("source_file")
        .unwrap_or(&"/tmp/ttt.html".to_string())
        .to_string();

    let config = AppConfig {
        course_pattern,
        redis_url,
        source_file,
    };

    Ok(config)
}
