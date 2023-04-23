#[derive(serde::Deserialize)]
pub struct Settings {
    pub database: String,
    pub app_port: String
}

#[derive(serde::Deserialize)]
pub struct DatabaseSettings {
    pub username: String,
    pub password: String,
    pub port: u16,
    pub host: String,
    pub database_name: String
}

pub fn get_configuration() -> Result<Settings, config::ConfigError> {
    let settings = config::Config::builder()
        .add_source(
            config::File::new("configuration.yml", config::FileFormat::Yaml)
        )
        .build()?;
    settings.try_deserialize::<Settings>()
}
