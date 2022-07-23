// ---------------------------------------
// Dreamspell config
// ---------------------------------------
use std::env;

pub struct Config {
    pub api_key: String,
}

impl Config {
    pub fn new() -> Result<Config, env::VarError> {
        let api_key = env::var("APIKEY")?;
        Ok(Config { api_key })
    }
}
