#[derive(serde::Deserialize)]
pub struct Settings {
    pub database: DatabaseSettings,
    pub application_port: u16,
}

#[derive(serde::Deserialize)]
pub struct DatabaseSettings {
    pub username: String,
    pub password: String,
    pub port: u16,
    pub host: String,
    pub database_name: String,
}

pub fn get_configuration() -> Result<Settings, config::ConfigError> {
    // initialise the configuration reader
    let mut settings = config::Config::default();
    // add config values from a file named 'configuration', multiple extensions are supported,
    // like yaml, json
    settings.merge(config::File::with_name("configuration"))?;
    // try to convert the parsed values to an instance the `Settings` type
    settings.try_into()
}
