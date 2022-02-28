use figment::{
    providers::{Env, Format, Toml},
    Figment,
};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Config {
    pub token: String,
    pub owner_id: u64,
    pub prefix: String,
}

impl Config {
    pub fn get() -> Self {
        Figment::new()
            .merge(Toml::file("Midori.toml"))
            .join(Env::prefixed("MIDORI_"))
            .extract()
            .expect("Failed to load config")
    }
}
