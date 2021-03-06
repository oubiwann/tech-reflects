use cfglib;
use serde::Deserialize;
use twyg::LoggerOpts;

const ENV_PREFIX: &str = "EXP";
const CONFIG_FILE: &str = "config";

#[derive(Debug, Deserialize)]
pub struct Game {
    pub title: String,
}

#[derive(Debug, Deserialize)]
pub struct NPCs {
    pub count: i32,
    pub chr: char,
    pub fg_color: (u8, u8, u8),
    pub bg_color: (u8, u8, u8),
    pub init_x: i32,
    pub init_y: i32,
}

#[derive(Debug, Deserialize)]
pub struct Player {
    pub chr: char,
    pub fg_color: (u8, u8, u8),
    pub bg_color: (u8, u8, u8),
}

#[derive(Debug, Deserialize)]
pub struct AppConfig {
    pub game: Game,
    pub logging: LoggerOpts,
    pub npcs: NPCs,
    pub player: Player,
}

impl AppConfig {
    pub fn new() -> Result<Self, cfglib::ConfigError> {
        let mut c = cfglib::Config::new();
        // Start off by merging in the default configuration values
        c.merge(cfglib::File::with_name(CONFIG_FILE))?;
        // Merge in overrides from the environment
        c.merge(cfglib::Environment::with_prefix(ENV_PREFIX))?;
        c.try_into()
    }
}
