use config;
use serde::Deserialize;
#[derive(Debug, Deserialize, Clone)]
pub struct WikiConfig {
    pub language: String,
    pub max_iterations: u32,
    pub pllimit: u32,
    pub lhlimit: u32,
    pub using_date: bool,
}

#[derive(Debug, Deserialize)]
struct FullConfig {
    wiki: WikiConfig,
}

impl Default for WikiConfig {
    fn default() -> Self {
        Self {
            language: "en".to_string(),
            max_iterations: 1_000_000,
            pllimit: 500,
            lhlimit: 500,
            using_date: true,
        }
    }
}

pub fn load_config(path: &str) -> Result<WikiConfig, config::ConfigError> {
    let config = config::Config::builder()
        .add_source(config::File::with_name(path))
        .build()?;
    config
        .try_deserialize::<FullConfig>()
        .map(|cfg| cfg.wiki)
        .or_else(|_| Ok(WikiConfig::default()))
}

pub fn get_config() -> WikiConfig {
    load_config("config.toml").unwrap_or_else(|_| {
        log::error!("Не удалось загрузить конфиг, используются значения по умолчанию");
        WikiConfig::default()
    })
}
