use serde::Deserialize;

fn default_host() -> String {
    "0.0.0.0".to_owned()
}

fn default_port() -> u16 {
    8080
}

#[derive(Debug, Deserialize)]
pub struct Env {
    #[serde(default = "default_host")]
    pub host: String,
    #[serde(default = "default_port")]
    pub port: u16,
}

impl Env {
    pub fn from_env() -> Result<Self, envy::Error> {
        envy::from_env()
    }
}
